pub mod writer;

use postgres::Client;

pub struct Context {
    pub db_name: Option<String>,
    pub postgres_client: Client,
}

impl Context {
    pub fn new(db_name: Option<String>, postgres_client: Client) -> Context {
        Context {
            db_name,
            postgres_client,
        }
    }
}