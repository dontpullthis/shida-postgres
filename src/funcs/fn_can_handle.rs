use shida_core::ffi::casting;
use shida_core::ffi::typedefs;

pub fn can_handle(connection_type: typedefs::ConstCCharPtr) -> bool {
    let connection_type_str = match casting::ccharptr_to_string(connection_type) {
        Ok(s) => s,
        Err(_) => return false,
    };
    vec!["pg", "psql", "postgres", "postgresql"].contains(&connection_type_str.as_str())
}