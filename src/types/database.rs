use shida_core::ffi::casting;
use shida_core::ffi::schema::{LevelC, LevelD};

use crate::types::Schema;

#[derive(Clone)]
pub struct Database {
    pub name: String,
    pub schemas: Vec<Schema>,
}

impl Database {
    pub fn new<S: Into<String>>(name: S) -> Database {
        Database {
            name: name.into(),
            schemas: Vec::new(),
        }
    }
}

impl Into<LevelD> for Database {
    fn into(self) -> LevelD {
        let mut level_d = LevelD::new();
        let levels_c: Vec<LevelC> = self.schemas.iter().map(|table| table.clone().into()).collect();
        level_d.set_children(levels_c);

        level_d
    }
}


impl From<LevelD> for Database {
    fn from(level_d: LevelD) -> Database {
        let mut database = Database::new(casting::ccharptr_to_string(level_d.name)
            .unwrap_or(String::from("")));
        let levels_c: Vec<LevelC> = level_d.children.clone().into();
        let schemas = levels_c.iter().map(|level_c| level_c.clone().into() ).collect();
        database.schemas = schemas;
        
        database
    }
}