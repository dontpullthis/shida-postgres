#[macro_use]
extern crate lazy_static;

mod context;
mod funcs;
mod globals;
mod types;

use shida_core::module::Module;

use crate::funcs::{apply_schema, can_handle, get_schema, init_reader, init_writer, read};

#[no_mangle]
fn load() -> Module {
    Module {
        apply_schema,
        can_handle,
        get_schema,
        init_reader,
        init_writer,
        read,
    }
}