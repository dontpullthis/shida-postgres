use shida_core::ffi::casting;
use shida_core::ffi::schema::{LevelB, LevelC};

use crate::types::Table;

#[derive(Clone)]
pub struct Schema {
    pub name: String,
    pub tables: Vec<Table>,
}

impl Schema {
    pub fn new<S: Into<String>>(name: S) -> Schema {
        Schema {
            name: name.into(),
            tables: Vec::new(),
        }
    }
}

impl Into<LevelC> for Schema {
    fn into(self) -> LevelC {
        let mut level_c  = LevelC::new();
        let levels_b: Vec<LevelB> = self.tables.iter().map(|table| table.clone().into()).collect();
        level_c.set_children(levels_b);

        level_c
    }
}

impl From<LevelC> for Schema {
    fn from(level_c: LevelC) -> Schema {
        let mut schema_name = casting::ccharptr_to_string(level_c.name).unwrap_or(String::from(""));
        if "" == schema_name {
            schema_name = String::from("public")
        }
        let mut schema = Schema::new(schema_name);
        let levels_b: Vec<LevelB> = level_c.children.clone().into();
        schema.tables = levels_b.iter().map(|level_b| level_b.clone().into() ).collect();

        schema
    }
}