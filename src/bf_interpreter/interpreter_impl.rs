// use std::io

use std::{io::{Write, Read}, str, str::Utf8Error, cmp::min};

use enum_dispatch::enum_dispatch;

use crate::bf_interpreter::interpreter::InterpreterError;

use super::interpreter::{BFCommand, Config, Interpreter, ProgramType, RunCompletePayload};
use super::interpreter;
use std::result::Result;

#[enum_dispatch]
pub trait SteppedInterpreter {
    // fn instance(config: Config) -> Self;
    fn load(&mut self, program: ProgramType);
    fn next(&mut self) -> interpreter::Result<()>;
    fn current_cell_offset(&self) -> usize;
    fn get_program(&self) -> &ProgramType;
    fn peek_cells_sliced(&self, start: usize, max_end: usize) -> &[u8];
    fn peek_output_sliced(&self, start: usize, max_end: usize) -> &[u8];

    fn peek_cells(&self, start: Option<usize>, max_end: Option<usize>) -> &[u8] {
        self.peek_cells_sliced(start.unwrap_or(0), max_end.unwrap_or(usize::MAX))
    }
    fn peek_output(&self, start: Option<usize>, max_end: Option<usize>) -> &[u8] {
        self.peek_output_sliced(start.unwrap_or(0), max_end.unwrap_or(usize::MAX))
    }
    fn current_cell_val(&self) -> u8 {
        let curr = self.current_cell_offset();
        self.peek_cells(Some(curr), Some(curr + 1))[0]
    }
    fn peek_output_str(&self, start: Option<usize>, max_end: Option<usize>) 
        -> Result<&str, Utf8Error> 
    {
        str::from_utf8(self.peek_cells(start, max_end))
    }

}

pub struct NaiveSteppedInterpreter {
    program: ProgramType,   // the instruction memory
    output: Vec<u8>,  // the output memory
    ostream: Box<dyn Write>,
    istream: Box<dyn Read>,
    cells: Vec<u8>,   // the interpreter's memory
    instr_ptr: usize, // pointer to current instruction
    ptr: usize        // pointer to current cell memory
}

impl SteppedInterpreter for NaiveSteppedInterpreter {
    fn load(&mut self, program: ProgramType) {
        self.program = program
    }

    fn next(&mut self) -> interpreter::Result<()> {
        type ResType = interpreter::Result<()>;
        let chr = &self.program[self.instr_ptr];
        match chr {
            BFCommand::IncCell => self.cells[self.ptr] += 1,
            BFCommand::DecCell => self.cells[self.ptr] -= 1,
            BFCommand::NextPtr => self.ptr += 1,
            BFCommand::PrevPtr => self.ptr -= 1,
            BFCommand::GetChar => 
                self.istream.read_exact(&mut self.cells[self.ptr..self.ptr+1])?,
            BFCommand::PutChar => 
                write!(self.ostream, "{}", self.cells[self.ptr])?,
            BFCommand::BegLoop(end_ptr) => 
                if self.cells[self.ptr] == 0 {
                    self.instr_ptr = end_ptr.to_owned()
                },
            BFCommand::EndLoop => {},
        }
        self.instr_ptr += 1;
        if self.instr_ptr >= self.program.len() {
            ResType::Err(InterpreterError::InterpreterDone)
        } else {
            ResType::Ok(())
        }
    }

    fn current_cell_offset(&self) -> usize {
        self.ptr
    }
    fn get_program(&self) -> &ProgramType{
        &self.program
    }

    fn peek_cells_sliced(&self, start: usize,max_end: usize) ->  &[u8] {
        &self.cells[start..min(max_end, self.cells.len())]
    }

    fn peek_output_sliced(&self, start: usize,max_end: usize) ->  &[u8] {
        &self.output[start..min(max_end, self.output.len())]
    }

}

#[enum_dispatch(SteppedInterpreter)]
pub enum SteppedInterpreterEnum {
    NaiveSteppedInterpreter(NaiveSteppedInterpreter)
}

impl Interpreter for SteppedInterpreterEnum {
    fn run_sync(&mut self, program: ProgramType) -> interpreter::Result<RunCompletePayload>{
        self.load(program);
        let run = loop {
            let _run = self.next();
            if _run.is_err() {
                break _run;
            }
        };
        run.map(|val| val.into())
    }    
}