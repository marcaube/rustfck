#[derive(PartialEq)]
pub enum Token {
    Right,
    Left,
    Add,
    Sub,
    Output,
    Input,
    LoopStart,
    LoopEnd,
    Other,
}
