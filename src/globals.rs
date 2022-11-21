use std::sync::Mutex;

use crate::context::writer::WriterContext;

pub struct Globals {
    pub writer_context: Option<WriterContext>,
}

impl Globals {
    fn new() -> Globals {
        Globals { writer_context: None }
    }
}

lazy_static! {
    pub static ref GLOBALS: Mutex<Globals> = {
        Mutex::new(Globals::new())
    };
}
