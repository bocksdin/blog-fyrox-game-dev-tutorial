//! Editor with your game connected to it as a plugin.
use crowd_control::GameConstructor;
use fyrox::event_loop::EventLoop;
use fyroxed_base::{Editor, StartupData};

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut editor = Editor::new(
        &event_loop,
        Some(StartupData {
            working_directory: Default::default(),
            scene: "data/scene.rgs".into(),
        }),
    );
    editor.add_game_plugin(GameConstructor);
    editor.run(event_loop)
}
