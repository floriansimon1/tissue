use pulldown_cmark;

use crate::errors;
use crate::issues::issue;
use crate::logging::logger;
use crate::phases::prepare_project_lazy_values;
use crate::issues::parser::make_markdown_parser;
use crate::issues::parser::issue_file::{free_text_section_parser, section_parser};

use crate::issues::parser::issue_file::top_level::make_appropriate_section_parser;
use crate::issues::parser::issue_file::top_level::top_level_logger::log_top_level_trace;

type TitleFragmentsList<'input> = Vec<pulldown_cmark::CowStr<'input>>;
type IgnoreTitleFragments       = bool;
type EventConsumed              = bool;

enum TopLevelParserState<'input> {
    ParsingSectionTitle(TitleFragmentsList<'input>),
    ParsingIssueTitle(TitleFragmentsList<'input>, IgnoreTitleFragments),
    ParsingSection(Box<dyn section_parser::SectionParser<'input> + 'input>),
}

pub fn parse_issue_file<'input>(
    logger:              &logger::Logger,
    project_lazy_values: &prepare_project_lazy_values::ProjectLazyValues,
    input:               &'input str
) -> issue::Issue<'input> {
    let mut issue        = issue::Issue::create_empty();
    let mut parser_state = create_new_free_text_section();

    for event in make_markdown_parser::make_markdown_parser(input) {
        let parsed_at_top_level = match &event {
            pulldown_cmark::Event::Text(text) => consume_content_at_top_level(logger, &mut parser_state, text.clone()),

            pulldown_cmark::Event::Start(tag) => {
                match tag {
                    pulldown_cmark::Tag::Heading(level, _, _) => {
                        let (consumed, state) = handle_title_start(logger, project_lazy_values, &mut issue, *level, parser_state);

                        parser_state = state;

                        consumed
                    },

                    _ => false,
                }
            }

            pulldown_cmark::Event::End(tag) => {
                match tag {
                    pulldown_cmark::Tag::Heading(level, _, _) => {
                        let (consumed, state) = handle_title_end(logger, &mut issue, *level, parser_state);

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

        section.process(&logger, &mut issue, event);
    }

    if issue.title.is_none() {
        log_top_level_trace(logger, "No issue title found after fully parsing the issue");

        issue.push_error(errors::IssueParsingError::NoIssueTitleFound)
    }

    issue
}

fn handle_title_start<'input>(
    logger:              &logger::Logger,
    project_lazy_values: &prepare_project_lazy_values::ProjectLazyValues,
    issue:               &mut issue::Issue<'input>,
    title_level:         pulldown_cmark::HeadingLevel,
    parser_state:        TopLevelParserState<'input>,
) -> (EventConsumed, TopLevelParserState<'input>) {
    let mut should_ignore_title = false;

    if title_level > pulldown_cmark::HeadingLevel::H2 {
        return (false, parser_state);
    }

    if !parser_state.is_ready_for_new_title() {
        log_top_level_trace(logger, "Found an unexpected title start");

        issue.push_error(errors::IssueParsingError::UnexpectedTitleStart);
    }

    if title_level == pulldown_cmark::HeadingLevel::H1 && issue.title.is_some() {
        log_top_level_trace(logger, "Found an unexpected title start");

        issue.push_error(errors::IssueParsingError::AdditionalTopLevelHeadingFound);

        should_ignore_title = true;
    }

    parser_state.apply(&logger, project_lazy_values, issue);

    (true, match title_level {
        pulldown_cmark::HeadingLevel::H1 => {
            log_top_level_trace(logger, "Now parsing an issue title…");

            TopLevelParserState::ParsingIssueTitle(Vec::new(), should_ignore_title)
        },

        pulldown_cmark::HeadingLevel::H2 => {
            log_top_level_trace(logger, "Now parsing a section title…");

            TopLevelParserState::ParsingSectionTitle(Vec::new())
        },

        _ => panic!("Got an unexpected heading start!"),
    })
}

fn consume_content_at_top_level<'input>(
    logger:       &logger::Logger,
    parser_state: &mut TopLevelParserState<'input>,
    content:      pulldown_cmark::CowStr<'input>,
) -> EventConsumed {
    match parser_state {
        TopLevelParserState::ParsingSection(_) => false,

        TopLevelParserState::ParsingIssueTitle(fragments, should_ignore_title_fragments) => {
            if !*should_ignore_title_fragments {
                log_top_level_trace(logger, "Accumulated a title fragment");

                fragments.push(content);
            } else {
                log_top_level_trace(logger, "Ignored a title fragment");
            }

            true
        }

        TopLevelParserState::ParsingSectionTitle(fragments) => {
            log_top_level_trace(logger, "Accumulated a section title fragment");

            fragments.push(content);

            true
        }
    }
}

fn handle_title_end<'input>(
    logger:       &logger::Logger,
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
                log_top_level_trace(logger, "Found the end of the issue title");

                issue.title = Some(fragments.join(""));
            } else {
                log_top_level_trace(logger, "Ignoring the issue title end");
            }

            (true, create_new_free_text_section())
        }

        TopLevelParserState::ParsingSectionTitle(fragments) => {
            if title_level != pulldown_cmark::HeadingLevel::H2 {
                panic!("Incorrect heading level detected while parsing an end tag!");
            }

            log_top_level_trace(logger, "Creating a new section using a section name");

            (true, TopLevelParserState::ParsingSection::<'input>(make_appropriate_section_parser(logger, fragments.join(""), issue)))
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

    pub fn apply(self, logger: &logger::Logger, project_lazy_values: &prepare_project_lazy_values::ProjectLazyValues, issue: &mut issue::Issue<'input>) {
        let section = match self.into_section_being_parsed() {
            Some(section) => section,
            None          => return,
        };

        section.save_on(&logger, project_lazy_values, issue);
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
