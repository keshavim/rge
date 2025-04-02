pub trait Application {
    fn run(&self) {
        crate::rge_engine_info!("runing");
    }
}
