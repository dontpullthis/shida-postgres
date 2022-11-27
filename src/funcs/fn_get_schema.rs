use shida_core::ffi::casting;
use shida_core::ffi::typedefs::Result;
use shida_core::ffi::schema::level_d::LevelD;

pub fn get_schema() -> Result<LevelD> {
    Result::Err(casting::str_to_ccharptr("Not implemented"))
}