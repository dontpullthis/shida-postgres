use shida_core::ffi::casting;
use shida_core::ffi::typedefs;
use shida_core::ffi::schema::LevelD;

use crate::globals;
use crate::types::Database;

pub fn apply_schema(level_d: LevelD) -> typedefs::ConstCCharPtr {
    let lock = globals::GLOBALS.lock().unwrap();
    let writer_context = lock.writer_context.as_ref().unwrap();
    let log = &(writer_context.app_config).functions.log;
    (log.debug)(casting::string_to_ccharptr("Applying schema..."));

    let database: Database = level_d.into();
    (log.debug)(casting::string_to_ccharptr(format!("Database name: {}", database.name)));
    for schema in &database.schemas {
        (log.debug)(casting::string_to_ccharptr(format!("- Schema name: {}", schema.name)));

        for table in &schema.tables {
            (log.debug)(casting::string_to_ccharptr(format!("-- Table name: {}", table.name)));

            for column in &table.columns {
                (log.debug)(casting::string_to_ccharptr(format!("--- Column name: {}", column.name)));
            }
        }
    }

    std::ptr::null()
}