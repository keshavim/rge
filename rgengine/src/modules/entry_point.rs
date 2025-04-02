//must derive from or have a default function
#[macro_export]
macro_rules! engine_run {
    ($a:ty) => {
        fn main() {
            let _ = rgengine::log::init();
            rgengine::rge_info!("init done");
            let sandbox: Box<dyn Application> = Box::new(<$a>::default());
            sandbox.run();
        }
    };
}
