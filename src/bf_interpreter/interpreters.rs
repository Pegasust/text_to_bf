///! Consists of the list of interpreters
use enum_dispatch::enum_dispatch;

use super::interpreter_impl::*;
use super::interpreter::*;


#[enum_dispatch(Interpreter)]
pub enum InterpreterEnum {
    SteppedInterpreterEnum(SteppedInterpreterEnum)
}

pub enum Negativity {
    NotAllowed, // immediately errors if reaches negativity
    WrapAround, // min - 1 -> max
    Extend      // allows seemingly negative infinity
}
pub struct InterpreterSpecs {
    wrapped_cell: bool,             // if yes, then 255 + 1 -> 0
    mem_ptr_negativity: Negativity, // behavior with negative mem ptr
    
}

pub fn create_interpreter(conf: Option<Config>) {
    create_interpreter_conf(conf.unwrap_or_else(Config::default))
}

fn create_interpreter_conf(conf: Config) {

}