use shida_core::ffi::casting;
use shida_core::ffi::schema::{LevelA, LevelB};

use crate::types::Column;

#[derive(Clone)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
    pub current_row: usize,
    pub total_rows: usize,
}

impl Table {
    pub fn new<S: Into<String>>(name: S) -> Table {
        Table {
            name: name.into(),
            columns: Vec::new(),
            current_row: 0,
            total_rows: 0,
        }
    }
}

impl Into<LevelB> for Table {
    fn into(self) -> LevelB {
        let mut level_b = LevelB::new();
        let levels_b: Vec<LevelA> = self.columns.iter().map(|column| column.clone().into()).collect();
        level_b.set_children(levels_b);

        level_b
    }
}

impl From<LevelB> for Table {
    fn from(level_b: LevelB) -> Table {
        let mut table = Table::new(casting::ccharptr_to_string(level_b.name)
            .unwrap_or(String::from("")));
        let levels_a: Vec<LevelA> = level_b.children.clone().into();
        table.columns = levels_a.iter().map(|level_a| level_a.clone().into() ).collect();
        
        table
    }
}