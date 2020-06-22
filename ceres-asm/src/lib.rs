// izzy do not forget to remove these later dummy
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

// namespacing
use logos::{Lexer, Logos};
use std::{fs::File, io::prelude::*, path::PathBuf};

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
    #[error(transparent)]
    StdIo(#[from] std::io::Error),
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
    pub fn assemble(&mut self) -> Result<MachineCode> {
        let mut machine_code: Vec<u32> = Vec::new();

        while let Some(token) = self.lexer.next() {
            match token {
                // cases that should continue
                Token::Comment => continue,
                Token::Newline => continue,
                // cases that like, actually make sense to process
                Token::Load => machine_code.push(self.load()?),
                Token::Add => machine_code.push(self.add()?),
                // cases that should straight up not happen
                _ => return Err(CeresAsmError::LazyBadToken { token }),
            }
        }
        Ok(MachineCode(machine_code))
    }

    // wrapping for lexer.next() with changing to result
    fn next(&mut self) -> Result<Token> {
        if let Some(token) = self.lexer.next() {
            Ok(token)
        } else {
            Err(CeresAsmError::NoToken)
        }
    }

    // load instruction assembly
    fn load(&mut self) -> Result<u32> {
        let opcode = 0b00001u32;

        let token = self.next()?;
        let signifier = Signifier::from_token(&token)?.as_bits() as u32;

        let token = self.next()?;
        let register = token.register_index()? as u32;

        let token = self.next()?;
        let literal = token.literal()? as u32;
        Ok((opcode << 27) | (signifier << 24) | (register << 20) | literal)
    }

    // add instruction assembly
    fn add(&mut self) -> Result<u32> {
        let opcode = 0b00010u32;

        let token = self.next()?;
        let source_one = token.register_index()? as u32;
        let token = self.next()?;
        let source_two = token.register_index()? as u32;
        let token = self.next()?;
        let dest = token.register_index()? as u32;

        Ok((opcode << 27) | (source_one << 8) | (source_two << 4) | dest)
    }
}

/// assembled machine code
pub struct MachineCode(pub Vec<u32>);

impl MachineCode {
    /// write the machine code to a file
    pub fn write_to_file(&self, dest: Option<PathBuf>) -> Result<()> {
        let dest = if let Some(path) = dest { path } else { PathBuf::from("out.bin") };
        let mut file = File::create(dest)?;
        let data = self.as_u8_vec();
        file.write_all(&data)?;
        Ok(())
    }

    // return the machine code as a vec of u8
    fn as_u8_vec(&self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        self.0.iter().for_each(|instruction| {
            data.extend_from_slice(&instruction.to_le_bytes());
        });
        data
    }
}

enum Signifier {
    VideoMemory,
    CartMemory,
    Immediate,
}

impl Signifier {
    fn from_token(token: &Token) -> Result<Self> {
        match token {
            Token::VideoMemory => Ok(Self::VideoMemory),
            Token::CartMemory => Ok(Self::CartMemory),
            Token::Immediate => Ok(Self::Immediate),
            _ => Err(CeresAsmError::LazyBadToken { token: token.clone() }),
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
#[derive(Logos, Debug, PartialEq, Clone, Copy)]
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
    #[token("add")]
    Add,

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
            Self::Add => "add".to_owned(),
            // errors
            Self::Error => "ERR".to_owned(),
        }
    }

    // returns the index of a register
    fn register_index(&self) -> Result<u8> {
        match self {
            Self::RegisterIndex(index) => Ok(index.clone() as u8),
            Self::TemporaryRegister(index) => Ok(index.clone() as u8 + 9),
            Self::ZeroRegister => Ok(0),
            Self::ProgramCounter => Ok(1),
            Self::StackPointer => Ok(2),
            Self::ReturnAddress => Ok(3),
            Self::ArgumentZero => Ok(4),
            Self::ArgumentOne => Ok(5),
            Self::ArgumentTwo => Ok(6),
            Self::ReturnZero => Ok(7),
            Self::ReturnOne => Ok(8),
            _ => return Err(CeresAsmError::LazyBadToken { token: self.clone() }),
        }
    }

    // returns literals as a u16
    fn literal(&self) -> Result<u16> {
        match self {
            Self::HexLiteral(val) => Ok(*val),
            Self::BinaryLiteral(val) => Ok(*val),
            Self::DecimalLiteral(val) => Ok(*val),
            _ => Err(CeresAsmError::LazyBadToken { token: self.clone() }),
        }
    }
}
