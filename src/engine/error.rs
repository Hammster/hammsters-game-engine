use std::fmt;

#[derive(Debug)]
pub enum EngineError {
    Testcase(EngineErrorMsg),
}

#[derive(Debug)]
pub struct EngineErrorMsg {
    errcode: u32,
    msg: &'static str,
}

impl EngineErrorMsg {
    pub fn new(errorcode: u32, msg: &'static str) -> Self {
        EngineErrorMsg {
            errcode: errorcode,
            msg: msg,
        }
    }
}

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // custom error actions go here
            _ => write!(f, "{:?}", self),
        }
    }
}
