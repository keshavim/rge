//must derive from or have a default function
#[macro_export]
macro_rules! engine_run {
    () => {
        fn main() {
            let _ = rgengine::log::init();
            rgengine::rge_info!("init done");
            let mut app = Application::new();
            app.push_layer(Box::new(Example_layer::default()));
            app.run();
        }
    };
}
