use std::collections::HashMap;

use postgres::{Client, NoTls};

use shida_core::ffi::app_config::AppConfig;
use shida_core::ffi::casting;
use shida_core::ffi::typedefs;
use shida_core::sys::args::string_to_keyvalue;

use crate::context::writer::WriterContext;
use crate::globals;

pub fn init_writer(app_config: *const AppConfig, paramsc: typedefs::Size, paramsv: *const typedefs::ConstCCharPtr) -> typedefs::ConstCCharPtr {
    let writer_params = match casting::cchar_ptr_to_vec_string(paramsc, paramsv) {
        Ok(p) => p,
        Err(e) => return casting::string_to_ccharptr(format!("Failed to convert param: {}", e)),
    };

    let mut conn_params: std::collections::HashMap<String, String> = HashMap::new();
    for param in writer_params.iter() {
        let (key, value) = match string_to_keyvalue(&param) {
            Some(r) => r,
            None => continue,
        };

        let key = match key.as_str() {
            "database" => "dbname",
            "host" => "host",
            "password" => "password",
            "port" => "port",
            "user" => "user",
            _ => continue,
        };
        let key = String::from(key);
        conn_params.insert(key, value);
    }
    let db_name = match conn_params.get("dbname") {
        Some(d) => Some(d.clone()),
        None => return casting::string_to_ccharptr("The database name is not provided."),
    };

    let conn_params = conn_params.iter()
        .map(|(k, v)| vec![k.clone(), v.clone()].join("="))
        .collect::<Vec<String>>()
        .join(" ");
    let client = match Client::connect(&conn_params, NoTls) {
        Ok(p) => p,
        Err(e) => return casting::string_to_ccharptr(format!("Failed to create a postgres client: {}", e)),
    };

    let context = WriterContext::new(app_config, db_name, client);
    globals::GLOBALS.lock().as_mut().unwrap().writer_context = Some(context);

    std::ptr::null()
}