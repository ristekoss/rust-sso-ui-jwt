#[cfg(feature = "log")]
pub fn init_logger() {
    std::env::set_var("RUST_LOG", "sso_ui_jwt");

    env_logger::init();

    log::info!("Initialized logger");
}
