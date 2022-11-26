use shida_core::ffi::casting;
use shida_core::ffi::typedefs::Result;
use shida_core::ffi::schema::level_e::LevelE;

pub fn get_schema() -> Result<LevelE> {
    Result::Err(casting::str_to_ccharptr("Not implemented"))
}