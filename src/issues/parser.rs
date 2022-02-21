use pulldown_cmark;

pub fn parse_title(input: &str) -> Option<String> {
    let mut accumulate_text = false;
    let mut fragments       = Vec::<pulldown_cmark::CowStr>::new();

    for event in pulldown_cmark::Parser::new(input) {
        match event {
            pulldown_cmark::Event::Start(tag)
            | pulldown_cmark::Event::End(tag) => {
                if let pulldown_cmark::Tag::Heading(level, _, _) = &tag {
                    if *level == pulldown_cmark::HeadingLevel::H1 {
                        if accumulate_text {
                            return Some(fragments.join("").as_str().into())
                        } else {
                            accumulate_text = true;
                        }
                    }
                }
            },

            pulldown_cmark::Event::Text(text) if accumulate_text => {
                fragments.push(text);
            },

            _ => (),
        }
    }

    None
}
