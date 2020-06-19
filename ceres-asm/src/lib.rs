use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    // general stuff
    #[regex(";.+")]
    Comment,
    #[token("@")]
    At,
    #[regex("/[a-z-_]+:/g")]
    Label,
    // registers
    #[token("$t0")]
    ZeroRegister,
    #[token("$pc")]
    ProgramCounter,
    #[token("$sp")]
    StackPointer,
    #[token("$ra")]
    ReturnAddress,
    #[token("$a0")]
    ArgumentZero,
    #[token("$a1")]
    ArgumentOne,
    #[token("$a2")]
    ArgumentTwo,
    #[token("$v0")]
    ReturnOne,
    #[token("$v1")]
    ReturnTwo,
    #[regex("\\$t[0-6]")]
    TemoraryRegister,
    #[regex("\\$[0-9]+")]
    RegisterIndex,

    // literals
    #[regex("0x[a-fA-F0-9]+", |lex| u16::from_str_radix(&lex.slice()[2..], 16).unwrap() )]
    HexLiteral(u16),
    #[regex("0b[0-1]+", |lex| u16::from_str_radix(&lex.slice()[2..], 2).unwrap())]
    BinaryLiteral(u16),
    #[regex("[0-9]+", |lex| lex.slice().parse::<u16>())]
    DecimalLiteral(u16),

    // instructions
    #[token("add")]
    Add,
    #[token("sub")]
    Sub,
    #[token("mul")]
    Mul,
    #[token("div")]
    Div,
    #[token("jmp")]
    Jmp,

    // logos error
    #[error]
    Error,
}
