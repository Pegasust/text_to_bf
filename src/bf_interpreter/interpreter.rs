use std::{io::{Write, Read}, ops::Index, borrow::Cow};

// use std::io;
use enum_dispatch::enum_dispatch;
type DynWrite = Box<dyn Write>;
type DynRead = Box<dyn Read>;

pub enum BFCommand {
    IncCell, // increments the current cell by 1
    DecCell, // decrements the current cell by 1
    NextPtr, // move the current cell to the next cell
    PrevPtr, // move the current cell to the previous cell
    GetChar, // prompts from the user one single byte, put value to current cell
    PutChar, // puts the byte value of the current cell to the terminal
    BegLoop(usize), // while(*ptr) { (the usize stores the corresponding endloop instr position)
    EndLoop, // }
}
pub enum InterpreterError {
    IOError(std::io::Error),
    IllegalStateError(String),
    InterpreterDone
}
impl From<std::io::Error> for InterpreterError {
    fn from(err: std::io::Error) -> Self {
        Self::IOError(err)
    }
}

impl From<()> for InterpreterError {
    fn from(_: ()) -> Self {
        Self::InterpreterDone
    }
}

pub enum RunCompletePayload {
    None
}

impl From<()> for RunCompletePayload {
    fn from(_: ()) -> Self {
        Self::None
    }
}

pub type Result<T> = std::result::Result<T, InterpreterError>;
pub struct Config {
    pub cells_sz: usize,
    pub ostream: DynWrite,
    pub istream: DynRead
}

impl Config {
    pub fn ctor(cells_sz: Option<usize>, ostream: Option<DynWrite>,
        istream: Option<DynRead>) -> Self 
    {
        Config { 
            cells_sz: cells_sz.unwrap_or(0x8000), 
            ostream: ostream.unwrap_or_else(|| Box::new(std::io::stdout())), 
            istream: istream.unwrap_or_else(|| Box::new(std::io::stdin())) 
        }
    }
    pub fn default() -> Config {
        Self::ctor(None, None, None)
    }
}

pub enum ProgramType {
    Vec(Vec<BFCommand>)
}

impl Index<usize> for ProgramType {
    type Output = BFCommand;
    fn index(&self, index: usize) -> &Self::Output {
        match &self {
            ProgramType::Vec(v) => &v[index]
        }
    }
}

impl ProgramType {
    pub fn len(&self) -> usize {
        match &self {
            ProgramType::Vec(v) => v.len()
        }
    }
}

#[enum_dispatch]
pub trait Interpreter {
    /// Runs the entire program synchronously
    fn run_sync(&mut self, program: ProgramType) -> Result<RunCompletePayload>;
}