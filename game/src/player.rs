use fyrox::{
    core::{
        algebra::{Vector2, Vector3},
        reflect::prelude::*,
        uuid::{uuid, Uuid},
        visitor::prelude::*,
        TypeUuidProvider,
    },
    event::{ElementState, Event, WindowEvent},
    impl_component_provider,
    keyboard::{KeyCode, PhysicalKey},
    scene::dim2::rigidbody::RigidBody,
    script::{ScriptContext, ScriptDeinitContext, ScriptTrait},
};

use crate::constants::MAP_OFFSET;

#[derive(Visit, Reflect, Default, Debug, Clone)]
pub struct Player {
    move_left: bool,
    move_right: bool,
    move_up: bool,
    move_down: bool,
}

impl_component_provider!(Player);

impl TypeUuidProvider for Player {
    fn type_uuid() -> Uuid {
        uuid!("85e2051e-d569-4bcf-b590-2483bda17302")
    }
}

impl ScriptTrait for Player {
    fn on_init(&mut self, context: &mut ScriptContext) {
        // Put initialization logic here.

        // Set the position of the player to the center of the offset map
        context.scene.graph[context.handle]
            .cast_mut::<RigidBody>()
            .unwrap()
            .local_transform_mut()
            .set_position(Vector3::new(MAP_OFFSET as f32, MAP_OFFSET as f32, 0.0));
    }

    fn on_start(&mut self, _context: &mut ScriptContext) {
        // There should be a logic that depends on other scripts in scene.
        // It is called right after **all** scripts were initialized.
    }

    fn on_deinit(&mut self, _context: &mut ScriptDeinitContext) {
        // Put de-initialization logic here.
    }

    fn on_os_event(&mut self, event: &Event<()>, _context: &mut ScriptContext) {
        // Destructure the event object if the event is a WindowEvent
        if let Event::WindowEvent { event, .. } = event {
            // Destructure the WindowEvent if it is a KeyboardInput
            if let WindowEvent::KeyboardInput { event, .. } = event {
                // Check if the key is currently being pressed
                let pressed = event.state == ElementState::Pressed;

                // Check if the key being pressed is W, A, S, or D
                // Update state accordingly
                match event.physical_key {
                    PhysicalKey::Code(KeyCode::KeyA) => self.move_left = pressed,
                    PhysicalKey::Code(KeyCode::KeyD) => self.move_right = pressed,
                    PhysicalKey::Code(KeyCode::KeyW) => self.move_up = pressed,
                    PhysicalKey::Code(KeyCode::KeyS) => self.move_down = pressed,
                    _ => {}
                }
            }
        }
    }

    fn on_update(&mut self, context: &mut ScriptContext) {
        // Grab the rigid body component from the entity
        if let Some(rigid_body) = context.scene.graph[context.handle].cast_mut::<RigidBody>() {
            // Determine the x and y speed based on the state of the keyboard input
            let x_speed = match (self.move_left, self.move_right) {
                (true, false) => 3.0,  // If the player is moving left, set the x speed to 3.0
                (false, true) => -3.0, // If the player is moving right, set the x speed to -3.0
                _ => 0.0, // If the player is not moving left or right, set the x speed to 0.0
            };
            let y_speed = match (self.move_up, self.move_down) {
                (true, false) => 3.0,  // If the player is moving up, set the y speed to 3.0
                (false, true) => -3.0, // If the player is moving down, set the y speed to -3.0
                _ => 0.0, // If the player is not moving up or down, set the y speed to 0.0
            };

            // Set the linear velocity of the rigid body based on the state of the player
            rigid_body.set_lin_vel(Vector2::new(x_speed, y_speed));
        }
    }

    fn id(&self) -> Uuid {
        Self::type_uuid()
    }
}
