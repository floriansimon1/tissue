mod top_level;
mod section_parser;
mod meta_section_parser;
mod ignored_section_parser;
mod comments_section_parser;
mod free_text_section_parser;

pub use top_level::parse_issue_file;
