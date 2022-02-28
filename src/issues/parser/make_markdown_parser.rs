use pulldown_cmark;

pub fn make_markdown_parser(input: &str) -> pulldown_cmark::Parser {
    pulldown_cmark::Parser::new_ext(input, pulldown_cmark::Options::ENABLE_TABLES)
}
