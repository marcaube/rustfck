#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Right,
    Left,
    Add(u8),
    Sub,
    Output,
    Input,
    LoopStart,
    LoopEnd,
    Other,
}
