use shida_core::ffi::casting;
use shida_core::ffi::typedefs;
use shida_core::ffi::schema::LevelE;


use crate::globals;

pub fn apply_schema(_level_e: LevelE) -> typedefs::ConstCCharPtr {
    let lock = globals::GLOBALS.lock().unwrap();
    let writer_context = lock.writer_context.as_ref().unwrap();
    let log = &(writer_context.app_config).functions.log;
    (log.debug)(casting::string_to_ccharptr("Applying schema..."));

    std::ptr::null()
}