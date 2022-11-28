use std::fmt;

use shida_core::ffi::casting;
use shida_core::ffi::schema::LevelA;

#[derive(Clone)]
pub enum ColumnType {
    Date,
    Int,
    Timestamp,
    Varchar,
}

impl From<String> for ColumnType {
    fn from(i: String) -> Self {
        match i.as_str() {
            "date" => ColumnType::Date,
            "int" => ColumnType::Int,
            "timestamp" => ColumnType::Timestamp,
            "varchar" => ColumnType::Varchar,
            _ => ColumnType::Varchar,
        }
    }
}

impl fmt::Display for ColumnType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        let string = match self {
            ColumnType::Date => "date",
            ColumnType::Int => "int",
            ColumnType::Timestamp => "timestamp",
            ColumnType::Varchar => "varchar",
        };
        write!(f, "{}", string)
    }
}

#[derive(Clone)]
pub struct Column {
    pub name: String,
    pub column_type: ColumnType,

    pub max_length: Option<usize>,
}

impl Column {
    pub fn new<S: Into<String>>(name: S, column_type: ColumnType) -> Column {
        Column {
            name: name.into(),
            column_type,
            max_length: None,
        }
    }
}

impl Into<LevelA> for Column {
    fn into(self) -> LevelA {
        let level_a = LevelA::new();

        level_a
    }
}

impl From<LevelA> for Column {
    fn from(level_a: LevelA) -> Column {
        let table = Column::new(casting::ccharptr_to_string(level_a.name)
            .unwrap_or(String::from("")), ColumnType::Varchar); // TODO: replace the static type with calculated value
        
        table
    }
}