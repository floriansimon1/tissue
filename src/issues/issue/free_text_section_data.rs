use pulldown_cmark;

pub struct FreeTextSectionData<'input> {
    pub title:  Option<String>,
    pub events: Vec<pulldown_cmark::Event<'input>>,
}
