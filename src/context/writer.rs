use postgres::Client;

use shida_core::ffi::app_config::AppConfig;

use crate::context::Context;

pub struct WriterContext {
    pub app_config: AppConfig,
    pub common_context: Context,
}

impl WriterContext {
    pub fn new(app_config: *const AppConfig, db_name: Option<String>, postgres_client: Client) -> WriterContext {
        WriterContext {
            app_config: unsafe { app_config.as_ref().unwrap().clone() },
            common_context: Context::new(db_name, postgres_client),
        }
    }
}