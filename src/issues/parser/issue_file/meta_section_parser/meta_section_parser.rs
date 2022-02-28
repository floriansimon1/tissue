use pulldown_cmark;

use crate::issues::parser::issue_file::section_parser;
use crate::phases::prepare_project_lazy_values;
use crate::issues::{issue, meta};
use crate::logging::logger;
use crate::errors;

use crate::issues::parser::issue_file::meta_section_parser::meta_section_parser_trace_logger::log_meta_trace;

pub const META_SECTION: &'static str = "meta";

type RawFieldTitle<'input> = pulldown_cmark::CowStr<'input>;
type RawFieldId<'input>    = pulldown_cmark::CowStr<'input>;

#[derive(Clone)]
enum RawFieldCell<'input> {
    Link(RawFieldId<'input>, RawFieldTitle<'input>),
    Text(pulldown_cmark::CowStr<'input>),
}

struct RawValuedField<'input> {
    pub field: RawFieldCell<'input>,
    pub value: RawFieldCell<'input>,
}

pub struct MetaSectionParser<'input> {
    pub has_ignored_data_outside_of_table: bool,
    pub meta:                              meta::Meta,

    state:  MetaParserState<'input>,
    fields: Vec<RawValuedField<'input>>,
}

impl<'input> MetaSectionParser<'input> {
    pub fn new() -> Box<MetaSectionParser<'input>> {
        Box::new(MetaSectionParser {
            has_ignored_data_outside_of_table: false,
            fields:                            Vec::new(),
            meta:                              meta::Meta::new(),
            state:                             MetaParserState::Start,
        })
    }
}

enum MetaParserState<'input> {
    Start,
    ParsingTableHead,
    LookingForFieldCell,
    ParsingField,
    SkippingRowUntilNextFieldCell,
    LookingForFieldValueCell(RawFieldCell<'input>),
    ParsingFieldValue(RawFieldCell<'input>),
    TableFullyParsed,
}

impl<'input> section_parser::SectionParser<'input> for MetaSectionParser<'input> {
    fn process(&mut self, logger: &logger::Logger, issue: &mut issue::Issue, event: pulldown_cmark::Event<'input>) {
        if let pulldown_cmark::Event::End(ref tag) = event {
            if let pulldown_cmark::Tag::Table(_) = tag {
                log_meta_trace(logger, "Found the meta table end");

                self.state = MetaParserState::TableFullyParsed;

                return;
            }
        }

        match &self.state {
            MetaParserState::Start                         => self.find_table_start(logger, event),
            MetaParserState::ParsingTableHead              => self.find_table_fields_start(logger, event),
            MetaParserState::LookingForFieldCell           => self.find_field_cell(logger, event),
            MetaParserState::ParsingField                  => self.find_field(logger, issue, event),
            MetaParserState::SkippingRowUntilNextFieldCell => self.skip_row_until_next_field_cell(logger, event),
            MetaParserState::TableFullyParsed              => (),

            MetaParserState::ParsingFieldValue(raw_field) => {
                let field_clone = raw_field.clone(); self.find_field_value(logger, issue, event, field_clone);
            },

            MetaParserState::LookingForFieldValueCell(raw_field) => {
                let field_clone = raw_field.clone(); self.find_field_value_cell(logger, event, field_clone);
            },
        }
    }

    fn save_on(self: Box<Self>, logger: &logger::Logger, project_lazy_values: &prepare_project_lazy_values::ProjectLazyValues, issue: &mut issue::Issue) {
        self.do_save_on(logger, project_lazy_values, issue);
    }
}

impl<'input> MetaSectionParser<'input> {
    fn find_table_start(&mut self, logger: &logger::Logger, event: pulldown_cmark::Event<'input>) {
        match event {
            pulldown_cmark::Event::Start(tag) => {
                match tag {
                    pulldown_cmark::Tag::Table(_) => {
                        log_meta_trace(logger, "Found the meta table start");

                        self.state = MetaParserState::ParsingTableHead;
                    }

                    _ => { self.has_ignored_data_outside_of_table = true; }
                }
            }

            _ => { self.has_ignored_data_outside_of_table = true; }
        }
    }

    fn find_table_fields_start(&mut self, logger: &logger::Logger, event: pulldown_cmark::Event<'input>) {
        if let pulldown_cmark::Event::End(tag) = event {
            if let pulldown_cmark::Tag::TableHead = tag {
                log_meta_trace(logger, "Found the end of the table header");

                self.state = MetaParserState::LookingForFieldCell;
            }
        }
    }

    fn find_field_cell(&mut self, logger: &logger::Logger, event: pulldown_cmark::Event<'input>) {
        match event {
            pulldown_cmark::Event::End(tag) => {
                if let pulldown_cmark::Tag::TableRow = tag {
                    log_meta_trace(logger, "Skipping an empty row");
                }
            },

            pulldown_cmark::Event::Start(tag) => {
                if let pulldown_cmark::Tag::TableCell = tag {
                    log_meta_trace(logger, "Found the cell start");

                    self.state = MetaParserState::ParsingField;
                }
            },

            _ => ()
        }
    }

    fn skip_row_until_next_field_cell(&mut self, logger: &logger::Logger, event: pulldown_cmark::Event<'input>) {
        if let pulldown_cmark::Event::End(ref tag) = event {
            if let pulldown_cmark::Tag::TableRow = tag {
                log_meta_trace(logger, "Looking for a new field cell");

                self.state = MetaParserState::LookingForFieldCell;
            }
        }
    }

    fn find_field(&mut self, logger: &logger::Logger, issue: &mut issue::Issue, event: pulldown_cmark::Event<'input>) {
        match event {
            pulldown_cmark::Event::End(tag) => {
                match tag {
                    pulldown_cmark::Tag::TableRow => {
                        log_meta_trace(logger, "Skipping empty row");

                        self.state = MetaParserState::LookingForFieldCell;
                    }

                    pulldown_cmark::Tag::TableCell => {
                        log_meta_trace(logger, "Skipping empty field");

                        issue.push_error(errors::IssueParsingError::UnidentifiedMetaField);

                        self.state = MetaParserState::SkippingRowUntilNextFieldCell;
                    }

                    _ => (),
                }
            },

            pulldown_cmark::Event::Start(tag) => {
                if let pulldown_cmark::Tag::Link(_, destination, title) = tag {
                    log_meta_trace(logger, "Found a referenced field");

                    self.state = MetaParserState::LookingForFieldValueCell(RawFieldCell::Link(destination, title));
                }
            },
            pulldown_cmark::Event::Text(text) => {
                log_meta_trace(logger, "Found a free-form field");

                self.state = MetaParserState::LookingForFieldValueCell(RawFieldCell::Text(text));
            },

            _ => ()
        }
    }

    fn find_field_value_cell(&mut self, logger: &logger::Logger, event: pulldown_cmark::Event<'input>, field: RawFieldCell<'input>) {
        match event {
            pulldown_cmark::Event::End(tag) => {
                if let pulldown_cmark::Tag::TableRow = tag {
                    log_meta_trace(logger, "Skipping a field without a value");

                    return;
                }
            },

            pulldown_cmark::Event::Start(tag) => {
                if let pulldown_cmark::Tag::TableCell = tag {
                    log_meta_trace(logger, "Found the field value cell start");

                    self.state = MetaParserState::ParsingFieldValue(field);
                }
            },

            _ => ()
        }
    }

    fn find_field_value(&mut self, logger: &logger::Logger, issue: &mut issue::Issue, event: pulldown_cmark::Event<'input>, field: RawFieldCell<'input>) {
        match event {
            pulldown_cmark::Event::End(tag) => {
                match tag {
                    pulldown_cmark::Tag::TableCell => {
                        log_meta_trace(logger, "Skipping field without a value");

                        issue.push_error(errors::IssueParsingError::FieldWithoutValue);

                        self.state = MetaParserState::SkippingRowUntilNextFieldCell;
                    }

                    _ => (),
                }
            },

            pulldown_cmark::Event::Start(tag) => {
                if let pulldown_cmark::Tag::Link(_, destination, title) = tag {
                    log_meta_trace(logger, "Found a referenced field value");

                    self.fields.push(RawValuedField { field, value: RawFieldCell::Link(destination, title) });

                    self.state = MetaParserState::SkippingRowUntilNextFieldCell;
                }
            },

            pulldown_cmark::Event::Text(text) => {
                log_meta_trace(logger, "Found a free-form field value");

                self.fields.push(RawValuedField { field, value: RawFieldCell::Text(text) });

                self.state = MetaParserState::SkippingRowUntilNextFieldCell;
            },

            _ => ()
        }
    }
}
