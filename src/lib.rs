#[macro_use]
extern crate lazy_static;

mod context;
mod funcs;
mod globals;

use shida_core::module::Module;

use crate::funcs::can_handle::can_handle;
use crate::funcs::init_reader::init_reader;
use crate::funcs::init_writer::init_writer;
use crate::funcs::read::read;


#[no_mangle]
fn load() -> Module {
    Module {
        can_handle,
        init_reader,
        init_writer,
        read,
    }
}