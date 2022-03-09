use std::sync;

use pulldown_cmark;

use crate::errors;
use crate::issues::issue;
use crate::phases::global;
use crate::issues::parser::make_markdown_parser;
use crate::issues::parser::issue_file::{free_text_section_parser, section_parser};

use crate::issues::parser::issue_file::top_level::make_appropriate_section_parser;
use crate::issues::parser::issue_file::top_level::top_level_logger::log_top_level_trace;
use crate::issues::parser::issue_file::field_mapping_parsing_result::FieldMappingParsingResult;

type TitleFragmentsList<'input> = Vec<pulldown_cmark::CowStr<'input>>;
type IgnoreTitleFragments       = bool;
type EventConsumed              = bool;

enum TopLevelParserState<'input> {
    ParsingSectionTitle(TitleFragmentsList<'input>),
    ParsingIssueTitle(TitleFragmentsList<'input>, IgnoreTitleFragments),
    ParsingSection(Box<dyn section_parser::SectionParser<'input> + 'input>),
}

pub async fn parse_issue_file<'input>(
    global:        sync::Arc<antidote::RwLock<global::Global>>,
    field_mapping: FieldMappingParsingResult,
    input:         &'input str
) -> issue::Issue<'input> {
    let mut issue        = issue::Issue::create_empty();
    let mut parser_state = create_new_free_text_section();

    for event in make_markdown_parser::make_markdown_parser(input) {
        let parsed_at_top_level = match &event {
            pulldown_cmark::Event::Text(text) => consume_content_at_top_level(global.clone(), &mut parser_state, text.clone()),

            pulldown_cmark::Event::Start(tag) => {
                match tag {
                    pulldown_cmark::Tag::Heading(level, _, _) => {
                        let (consumed, state) = handle_title_start(global.clone(), field_mapping.clone(), &mut issue, *level, parser_state).await;

                        parser_state = state;

                        consumed
                    },

                    _ => false,
                }
            }

            pulldown_cmark::Event::End(tag) => {
                match tag {
                    pulldown_cmark::Tag::Heading(level, _, _) => {
                        let (consumed, state) = handle_title_end(global.clone(), &mut issue, *level, parser_state);

                        parser_state = state;

                        consumed
                    },

                    _ => false,
                }
            }

            _ => false,
        };

        if parsed_at_top_level {
            continue;
        }

        let section = parser_state
        .section_being_parsed()
        .expect("Expected to get a section!");

        section.process(global.clone(), &mut issue, event);
    }

    if issue.title.is_none() {
        log_top_level_trace(&global.read().logger, "No issue title found after fully parsing the issue");

        issue.push_error(errors::IssueParsingError::NoIssueTitleFound)
    }

    issue
}

async fn handle_title_start<'input>(
    global:        sync::Arc<antidote::RwLock<global::Global>>,
    field_mapping: FieldMappingParsingResult,
    issue:         &mut issue::Issue<'input>,
    title_level:   pulldown_cmark::HeadingLevel,
    parser_state:  TopLevelParserState<'input>,
) -> (EventConsumed, TopLevelParserState<'input>) {
    let mut should_ignore_title = false;

    if title_level > pulldown_cmark::HeadingLevel::H2 {
        return (false, parser_state);
    }

    if !parser_state.is_ready_for_new_title() {
        log_top_level_trace(&global.read().logger, "Found an unexpected title start");

        issue.push_error(errors::IssueParsingError::UnexpectedTitleStart);
    }

    if title_level == pulldown_cmark::HeadingLevel::H1 && issue.title.is_some() {
        log_top_level_trace(&global.read().logger, "Found an unexpected title start");

        issue.push_error(errors::IssueParsingError::AdditionalTopLevelHeadingFound);

        should_ignore_title = true;
    }

    parser_state.apply(global.clone(), field_mapping, issue).await;

    (true, match title_level {
        pulldown_cmark::HeadingLevel::H1 => {
            log_top_level_trace(&global.read().logger, "Now parsing an issue title…");

            TopLevelParserState::ParsingIssueTitle(Vec::new(), should_ignore_title)
        },

        pulldown_cmark::HeadingLevel::H2 => {
            log_top_level_trace(&global.read().logger, "Now parsing a section title…");

            TopLevelParserState::ParsingSectionTitle(Vec::new())
        },

        _ => panic!("Got an unexpected heading start!"),
    })
}

fn consume_content_at_top_level<'input>(
    global:       sync::Arc<antidote::RwLock<global::Global>>,
    parser_state: &mut TopLevelParserState<'input>,
    content:      pulldown_cmark::CowStr<'input>,
) -> EventConsumed {
    match parser_state {
        TopLevelParserState::ParsingSection(_) => false,

        TopLevelParserState::ParsingIssueTitle(fragments, should_ignore_title_fragments) => {
            if !*should_ignore_title_fragments {
                log_top_level_trace(&global.read().logger, "Accumulated a title fragment");

                fragments.push(content);
            } else {
                log_top_level_trace(&global.read().logger, "Ignored a title fragment");
            }

            true
        }

        TopLevelParserState::ParsingSectionTitle(fragments) => {
            log_top_level_trace(&global.read().logger, "Accumulated a section title fragment");

            fragments.push(content);

            true
        }
    }
}

fn handle_title_end<'input>(
    global:       sync::Arc<antidote::RwLock<global::Global>>,
    issue:        &mut issue::Issue,
    title_level:  pulldown_cmark::HeadingLevel,
    parser_state: TopLevelParserState<'input>,
) -> (EventConsumed, TopLevelParserState<'input>) {
    if title_level > pulldown_cmark::HeadingLevel::H2 {
        return (false, parser_state);
    }

    match parser_state {
        TopLevelParserState::ParsingIssueTitle(fragments, should_ignore_title_fragments) => {
            if title_level != pulldown_cmark::HeadingLevel::H1 {
                panic!("Incorrect heading level detected while parsing an end tag!");
            }

            if !should_ignore_title_fragments {
                log_top_level_trace(&global.read().logger, "Found the end of the issue title");

                issue.title = Some(fragments.join(""));
            } else {
                log_top_level_trace(&global.read().logger, "Ignoring the issue title end");
            }

            (true, create_new_free_text_section())
        }

        TopLevelParserState::ParsingSectionTitle(fragments) => {
            if title_level != pulldown_cmark::HeadingLevel::H2 {
                panic!("Incorrect heading level detected while parsing an end tag!");
            }

            log_top_level_trace(&global.read().logger, "Creating a new section using a section name");

            (true, TopLevelParserState::ParsingSection::<'input>(make_appropriate_section_parser(&global.read().logger, fragments.join(""), issue)))
        },

        _ => (false, parser_state)
    }
}

impl<'input> TopLevelParserState<'input> {
    pub fn section_being_parsed<'self_lifetime>(&'self_lifetime mut self)
    -> Option<&'self_lifetime mut Box<dyn section_parser::SectionParser<'input> + 'input>>
    {
        match self {
            TopLevelParserState::ParsingSection::<'input>(section) => Some(section),
            _                                                      => None,
        }
    }

    pub fn into_section_being_parsed(self)
    -> Option<Box<dyn section_parser::SectionParser<'input> + 'input>>
    {
        match self {
            TopLevelParserState::ParsingSection(section) => Some(section),
            _                                            => None,
        }
    }

    pub async fn apply(
        self,
        global:        sync::Arc<antidote::RwLock<global::Global>>,
        field_mapping: FieldMappingParsingResult,
        issue:         &mut issue::Issue<'input>
    ) {
        let section = match self.into_section_being_parsed() {
            Some(section) => section,
            None          => return,
        };

        section.save_on(global, field_mapping, issue).await;
    }

    pub fn is_ready_for_new_title(&self) -> bool {
        match self {
            Self::ParsingSection(_)       => true,
            Self::ParsingIssueTitle(_, _) => false,
            Self::ParsingSectionTitle(_)  => false,
        }
    }
}

fn create_new_free_text_section<'input>() -> TopLevelParserState<'input> {
    TopLevelParserState::ParsingSection::<'input>(Box::new(free_text_section_parser::FreeTextSectionParser::<'input>::new_empty()))
}
