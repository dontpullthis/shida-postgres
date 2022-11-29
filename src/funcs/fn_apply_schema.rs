use shida_core::ffi::casting;
use shida_core::ffi::typedefs;
use shida_core::ffi::schema::LevelD;

use crate::globals;
use crate::types::{Database, Schema};
use crate::context::writer::WriterContext;

pub fn apply_schema(level_d: LevelD) -> typedefs::ConstCCharPtr {
    let mut lock = globals::GLOBALS.lock().unwrap();
    let writer_context = lock.writer_context.as_mut().unwrap();
    let log = &mut (writer_context.app_config).functions.log.clone();
    (log.debug)(casting::string_to_ccharptr("Applying schema..."));

    let database: Database = level_d.into();
    (log.debug)(casting::string_to_ccharptr(format!("Database name: {}", database.name)));
    for schema in &database.schemas {
        let err = create_schema(writer_context, schema);
        if !err.is_null() {
            return err;
        }

        for table in &schema.tables {
            (log.debug)(casting::string_to_ccharptr(format!("-- Creating table: {}", table.name)));
            let mut create_table_ddl = format!("CREATE TABLE {}.{} (", schema.name, table.name);

            let columns: Vec<String> = table.columns.iter()
                .map(|col| format!("{} {}", col.name, col.column_type))
                .collect();
            let columns = columns.join("\n,");

            create_table_ddl += format!("{});", columns).as_str();

            (log.debug)(casting::string_to_ccharptr(format!("SQL: {}", create_table_ddl)));
            let client = &mut writer_context.common_context.postgres_client;
            match client.execute(create_table_ddl.as_str(), &[]) {
                Ok(_) => std::ptr::null(),
                Err(e) => casting::string_to_ccharptr(format!("Failed to execute a Postgres query: {}\nError: {}", create_table_ddl, e)),
            };

        }

    }

    std::ptr::null()
}

fn create_schema(writer_context: &mut WriterContext, schema: &Schema) -> typedefs::ConstCCharPtr {
    let log = &(writer_context.app_config).functions.log;
    let client = &mut writer_context.common_context.postgres_client;

    (log.debug)(casting::string_to_ccharptr(format!("- Creating a schema: {}", schema.name)));
    let query = format!("CREATE SCHEMA IF NOT EXISTS {}", schema.name);
    match client.execute(query.as_str(), &[]) {
        Ok(_) => std::ptr::null(),
        Err(e) => casting::string_to_ccharptr(format!("Failed to execute a Postgres query: {}\nError: {}", query, e)),
    }
}