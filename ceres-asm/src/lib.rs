// izzy do not forget to remove these later dummy
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

// namespacing
use logos::{Lexer, Logos};

/// ceres-asm result type
pub type Result<T> = std::result::Result<T, CeresAsmError>;

/// ceres-asm error type
#[derive(Debug, thiserror::Error)]
pub enum CeresAsmError {
    #[error("wrong token found:\nexpected: {expected:?}\n found: {found:?}")]
    BadToken { expected: Token, found: Token },
    #[error("bad token: {token:?}")]
    LazyBadToken { token: Token },
    #[error("was looking for token, found nothing ???")]
    NoToken,
}

/// assembler struct
pub struct Assembler<'a> {
    lexer: Lexer<'a, Token>,
}

impl<'a> Assembler<'a> {
    /// create a new assembler
    pub fn new(data: &'a str) -> Assembler<'a> {
        let lexer = Token::lexer(data);
        Assembler { lexer }
    }

    /// assemble
    pub fn assemble(&mut self) -> Result<Vec<u32>> {
        let mut machine_code: Vec<u32> = Vec::new();

        while let Some(token) = self.lexer.next() {
            match token {
                // cases that should continue
                Token::Comment => continue,
                Token::Newline => continue,
                // cases that like, actually make sense to process
                Token::Load => {
                    if let Some(new_token) = self.lexer.next() {
                        let signifier = Signifier::from_token(new_token)?;
                        machine_code.push(self.load(signifier)?);
                    } else {
                        return Err(CeresAsmError::NoToken);
                    }
                }
                // cases that should straight up not happen
                _ => return Err(CeresAsmError::LazyBadToken { token }),
            }
        }

        Ok(machine_code)
    }

    // load instruction assembly
    fn load(&mut self, signifier: Signifier) -> Result<u32> {
        let opcode: u32 = 0b00001;
        let signifier: u32 = signifier.as_bits() as u32;
        let instruction: u32 = (opcode << 27) | (signifier << 24);
        Ok(instruction)
    }
}

enum Signifier {
    VideoMemory,
    CartMemory,
    Immediate,
}

impl Signifier {
    fn from_token(token: Token) -> Result<Self> {
        match token {
            Token::VideoMemory => Ok(Self::VideoMemory),
            Token::CartMemory => Ok(Self::CartMemory),
            Token::Immediate => Ok(Self::Immediate),
            _ => Err(CeresAsmError::LazyBadToken { token }),
        }
    }

    // returns the signifier as bits stored in a u8
    fn as_bits(self) -> u8 {
        match self {
            Self::VideoMemory => 0b010,
            Self::CartMemory => 0b001,
            Self::Immediate => 0b100,
        }
    }
}

/// token
#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    // general stuff
    #[regex(";.+")]
    Comment,
    #[token("\n")]
    Newline,

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
    ReturnZero,
    #[token("$v1")]
    ReturnOne,
    #[regex("\\$t[0-6]", |lex| lex.slice()[2..].parse::<u16>())]
    TemporaryRegister(u16),
    #[regex("\\$[0-9]+", |lex| lex.slice()[1..].parse::<u16>())]
    RegisterIndex(u16),

    // types
    #[token("immd")]
    Immediate,
    #[token("vram")]
    VideoMemory,
    #[token("cram")]
    CartMemory,

    // literals
    #[regex("0x[a-fA-F0-9]+", |lex| u16::from_str_radix(&lex.slice()[2..], 16).unwrap() )]
    HexLiteral(u16),
    #[regex("0b[0-1]+", |lex| u16::from_str_radix(&lex.slice()[2..], 2).unwrap())]
    BinaryLiteral(u16),
    #[regex("[0-9]+", |lex| lex.slice().parse::<u16>())]
    DecimalLiteral(u16),

    // instructions
    #[token("ld:")]
    Load,

    // logos error
    #[error]
    #[regex(" +", logos::skip)]
    Error,
}

impl Token {
    // returns the token as a str
    fn as_str(&self) -> String {
        match self {
            // general stuff
            Self::Comment => "a comment".to_owned(),
            Self::Newline => "\\n".to_owned(),
            // registers
            Self::ZeroRegister => "$z0".to_owned(),
            Self::ProgramCounter => "$pc".to_owned(),
            Self::StackPointer => "$sp".to_owned(),
            Self::ReturnAddress => "$ra".to_owned(),
            Self::ArgumentZero => "$a0".to_owned(),
            Self::ArgumentOne => "$z1".to_owned(),
            Self::ArgumentTwo => "$a2".to_owned(),
            Self::ReturnZero => "$v0".to_owned(),
            Self::ReturnOne => "$v1".to_owned(),
            Self::TemporaryRegister(idx) => format!("$t{}", idx),
            Self::RegisterIndex(idx) => format!("${}", idx),
            // types
            Self::Immediate => "immd".to_owned(),
            Self::VideoMemory => "vram".to_owned(),
            Self::CartMemory => "cram".to_owned(),
            // literals
            Self::HexLiteral(_) => "hex lit".to_owned(),
            Self::BinaryLiteral(_) => "bin lit".to_owned(),
            Self::DecimalLiteral(_) => "dec lit".to_owned(),
            // instructions
            Self::Load => "ld:".to_owned(),
            // errors
            Self::Error => "ERR".to_owned(),
        }
    }

    // returns true if given token is a register
    fn is_register(&self) -> bool {
        match self {
            Self::ZeroRegister
            | Self::ProgramCounter
            | Self::StackPointer
            | Self::ReturnAddress
            | Self::ArgumentZero
            | Self::ArgumentOne
            | Self::ArgumentTwo
            | Self::ReturnZero
            | Self::ReturnOne
            | Self::TemporaryRegister(_)
            | Self::RegisterIndex(_) => true,
            _ => false,
        }
    }

    // returns true if given token is a literal
    fn is_literal(&self) -> bool {
        match self {
            Self::HexLiteral(_) | Self::BinaryLiteral(_) | Self::DecimalLiteral(_) => true,
            _ => false,
        }
    }

    // returns true if it's a valid memory location or immediate
    fn is_signifier(&self) -> bool {
        match self {
            Self::VideoMemory | Self::CartMemory | Self::Immediate => true,
            _ => false,
        }
    }
}
