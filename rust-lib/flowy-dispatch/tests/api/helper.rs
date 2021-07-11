use flowy_dispatch::prelude::*;
use std::sync::Once;

#[allow(dead_code)]
pub fn setup_env() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        std::env::set_var("RUST_LOG", "flowy_dispatch=debug,debug");
        env_logger::init();
    });
}

pub fn init_dispatch<F>(module_factory: F)
where
    F: FnOnce() -> Vec<Module>,
{
    EventDispatch::construct(module_factory);
}