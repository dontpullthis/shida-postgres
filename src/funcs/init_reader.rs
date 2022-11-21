use shida_core::ffi::app_config::AppConfig;
use shida_core::ffi::typedefs;

pub fn init_reader(_app_config: *const AppConfig, _paramsc: typedefs::Size, _paramsv: *const typedefs::ConstCCharPtr) -> typedefs::ConstCCharPtr {
    panic!("Not implemented")
}