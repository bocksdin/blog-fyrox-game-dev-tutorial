use fyrox::{
    core::{
        algebra::{Vector2, Vector3},
        color::Color,
        pool::Handle,
        reflect::prelude::*,
        uuid::{uuid, Uuid},
        visitor::prelude::*,
        TypeUuidProvider,
    },
    event::{ElementState, Event, WindowEvent},
    impl_component_provider,
    keyboard::{KeyCode, PhysicalKey},
    scene::{
        base::BaseBuilder,
        dim2::{rectangle::RectangleBuilder, rigidbody::RigidBody},
        node::Node,
        transform::TransformBuilder,
    },
    script::{ScriptContext, ScriptDeinitContext, ScriptTrait},
};

use crate::constants::MAP_OFFSET;

#[derive(Visit, Reflect, Default, Debug, Clone)]
pub struct Player {
    move_left: bool,
    move_right: bool,
    move_up: bool,
    move_down: bool,

    // Health Bar
    max_health: f32,
    health: f32,
    health_bar_background: Handle<Node>,
    health_bar_progress: Handle<Node>,
}

impl_component_provider!(Player);

impl TypeUuidProvider for Player {
    fn type_uuid() -> Uuid {
        uuid!("85e2051e-d569-4bcf-b590-2483bda17302")
    }
}

impl Player {
    pub const PLAYER_STARTING_HEALTH: f32 = 100.0;
    pub const MAX_HEALTH_BAR_WIDTH: f32 = 1.0;

    pub fn take_damage(&mut self, damage: &f32) {
        self.health -= damage;
    }
}

impl ScriptTrait for Player {
    fn on_init(&mut self, context: &mut ScriptContext) {
        // Set the position of the player to the center of the offset map
        context.scene.graph[context.handle]
            .cast_mut::<RigidBody>()
            .unwrap()
            .local_transform_mut()
            .set_position(Vector3::new(MAP_OFFSET as f32, MAP_OFFSET as f32, 0.0));

        // Create the healthbar background
        let health_bar_background = RectangleBuilder::new(
            BaseBuilder::new()
                .with_name("HealthBarBackground")
                .with_local_transform(
                    TransformBuilder::new()
                        // Resize the healthbar so it is wide and short
                        .with_local_scale(Vector3::new(
                            Self::MAX_HEALTH_BAR_WIDTH,
                            Self::MAX_HEALTH_BAR_WIDTH / 4.,
                            1.,
                        ))
                        // Move the pivot center to the top left of the healthbar
                        // This is so it can be scaled from the left to the right
                        .with_scaling_pivot(Vector3::new(
                            Self::MAX_HEALTH_BAR_WIDTH / 2.,
                            Self::MAX_HEALTH_BAR_WIDTH / 4.,
                            1.,
                        ))
                        // Position the healthbar just above the player
                        .with_local_position(Vector3::new(0., 0.75, 0.01))
                        .build(),
                ),
        )
        // Gray color
        .with_color(Color::opaque(50, 50, 50))
        .build(&mut context.scene.graph);

        // Create the healthbar progress
        let health_bar_progress = RectangleBuilder::new(
            BaseBuilder::new()
                .with_name("HealthBarProgress")
                .with_local_transform(
                    TransformBuilder::new()
                        // Resize the healthbar so it is wide and short
                        .with_local_scale(Vector3::new(
                            Self::MAX_HEALTH_BAR_WIDTH,
                            Self::MAX_HEALTH_BAR_WIDTH / 4.,
                            1.,
                        ))
                        // Move the pivot center to the top left of the healthbar
                        // This is so it can be scaled from the left to the right
                        .with_scaling_pivot(Vector3::new(
                            Self::MAX_HEALTH_BAR_WIDTH / 2.,
                            Self::MAX_HEALTH_BAR_WIDTH / 4.,
                            1.,
                        ))
                        // Position the healthbar just above the player, and just in front of the background
                        .with_local_position(Vector3::new(0., 0.75, 0.))
                        .build(),
                ),
        )
        .with_color(Color::GREEN)
        .build(&mut context.scene.graph);

        // Make the healthbar background and progress child nodes of the player
        context
            .scene
            .graph
            .link_nodes(health_bar_background, context.handle);
        context
            .scene
            .graph
            .link_nodes(health_bar_progress, context.handle);

        // Set the Player struct properties
        self.max_health = Self::PLAYER_STARTING_HEALTH;
        self.health = Self::PLAYER_STARTING_HEALTH;
        self.health_bar_background = health_bar_background;
        self.health_bar_progress = health_bar_progress;
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

        // Grab the healthbar progess node
        let health_bar_progress = context.scene.graph[self.health_bar_progress].as_rectangle_mut();

        // Grab the healthbar progess node's transform
        let health_bar_transform = health_bar_progress.local_transform_mut();

        // Grab the current scale of the healthbar progess node
        let health_bar_scale = health_bar_transform.scale();

        // Calculate the new scale of the healthbar progess node
        // Don't let it go below 0 HP
        let new_health = f32::max(
            (self.health / self.max_health) * Self::MAX_HEALTH_BAR_WIDTH,
            0.0,
        );

        // If the new scale is different from the current scale, update the scale
        if health_bar_scale.x != new_health {
            health_bar_transform.set_scale(Vector3::new(
                new_health,
                // Don't change the y or z scale
                health_bar_scale.y,
                health_bar_scale.z,
            ));
        }
    }

    fn id(&self) -> Uuid {
        Self::type_uuid()
    }
}
