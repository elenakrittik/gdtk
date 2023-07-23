#[derive(Debug, Default)]
pub struct State {
    pub indent_style: Option<IndentStyle>,
}

#[derive(Debug)]
pub enum IndentStyle {
    Spaces,
    Tabs,
}
