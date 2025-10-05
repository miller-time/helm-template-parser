pub enum Token {
    Space,
    Letter(char),
    Number(u8),
    Dash,
    Colon,
    LeftBrace,
    RightBrace,
}
