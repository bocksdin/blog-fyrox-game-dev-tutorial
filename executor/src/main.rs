//! Executor with your game connected to it as a plugin.
use crowd_control::GameConstructor;
use fyrox::{
    engine::{executor::Executor, GraphicsContextParams},
    event_loop::EventLoop,
    window::WindowAttributes,
};

fn main() {
    let mut window_attributes = WindowAttributes::default();
    window_attributes.title = "Crowd Control".to_string();
    let mut executor = Executor::from_params(
        EventLoop::new().unwrap(),
        GraphicsContextParams {
            window_attributes,
            vsync: false,
        },
    );
    executor.add_plugin_constructor(GameConstructor);
    executor.run()
}
