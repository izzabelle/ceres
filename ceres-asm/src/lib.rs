use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
enum Token {
    // general stuff
    #[regex(";.+")]
    Comment,
    // registers
    #[token("$t0")]
    #[token("$0")]
    ZeroRegister,
    #[token("$gp")]
    #[token("$1")]
    GlobalPointer,
    #[token("$sp")]
    #[token("$2")]
    StackPointer,

    // literals
    #[regex("0x[a-fA-F0-9]+")]
    HexLiteral,
    #[regex("0b[0-1]+")]
    BinaryLiteral,
    #[regex("[0-9]+")]
    DecimalLiteral,

    // instructions
    #[token("add")]
    Add,
    #[token("sub")]
    Sub,
    #[token("mul")]
    Mul,
    #[token("div")]
    Div,

    // logos error
    #[error]
    Error,
}
