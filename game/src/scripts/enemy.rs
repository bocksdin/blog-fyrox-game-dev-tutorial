use fyrox::{
    core::{
        algebra::{Vector2, Vector3},
        pool::Handle,
        reflect::prelude::*,
        uuid::{uuid, Uuid},
        visitor::prelude::*,
        TypeUuidProvider,
    },
    event::Event,
    impl_component_provider,
    scene::{
        base::BaseBuilder,
        collider::{BitMask, InteractionGroups},
        dim2::{
            collider::{Collider, ColliderBuilder, ColliderShape, CuboidShape},
            rectangle::RectangleBuilder,
            rigidbody::RigidBodyBuilder,
        },
        node::Node,
        transform::TransformBuilder,
    },
    script::{Script, ScriptContext, ScriptDeinitContext, ScriptTrait},
};

#[derive(Visit, Reflect, Default, Debug, Clone)]
pub struct Enemy {
    // Self node handles
    handle: Handle<Node>,
    sprite: Handle<Node>,

    // Self properties
    name: String,
    speed: f32,
    scale: f32,
    attack_damage: f32,
    attack_speed: f32,

    // Initial spawn point
    starting_position: Vector2<f32>,

    // Timer for attacks
    attack_timer: f32,

    // Player node handles
    player_handle: Handle<Node>,
    player_collider: Handle<Node>,
}

impl Enemy {
    pub fn new() -> Self {
        Self {
            handle: Handle::NONE,
            name: "".to_owned(),
            speed: 0.0,
            scale: 1.0,
            attack_damage: 0.0,
            attack_speed: 0.0,
            starting_position: Vector2::new(0.0, 0.0),
            player_handle: Handle::NONE,
            player_collider: Handle::NONE,
            attack_timer: 0.0,
            sprite: Handle::NONE,
        }
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_owned();
        self
    }

    pub fn with_speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }

    pub fn with_starting_position(mut self, position: Vector2<f32>) -> Self {
        self.starting_position = position;
        self
    }

    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }

    pub fn with_attack_damage(mut self, attack_damage: f32) -> Self {
        self.attack_damage = attack_damage;
        self
    }

    pub fn with_attack_speed(mut self, attack_speed: f32) -> Self {
        self.attack_speed = attack_speed;
        self
    }

    // ScriptContext implements three lifetimes, but we don't use them here so leave them anonymous
    pub fn build(mut self, context: &mut ScriptContext<'_, '_, '_>) -> Handle<Node> {
        // Build a 2D rigid body
        RigidBodyBuilder::new(
            BaseBuilder::new()
                // Instantiate at the initial starting position and scale defined
                .with_local_transform(
                    TransformBuilder::new()
                        .with_local_position(Vector3::new(
                            self.starting_position.x,
                            self.starting_position.y,
                            0.0,
                        ))
                        .with_local_scale(Vector3::new(self.scale, self.scale, 1.0))
                        .build(),
                )
                .with_children(&[
                    // Add a 2D collider
                    ColliderBuilder::new(BaseBuilder::new())
                        // Fit to the square based on the rigid body scale
                        .with_shape(ColliderShape::Cuboid(CuboidShape {
                            half_extents: Vector2::new(self.scale / 2., self.scale / 2.),
                        }))
                        .with_collision_groups(InteractionGroups {
                            // Assign it to the second collision membership group only
                            memberships: BitMask(0b0100_0000_0000_0000_0000_0000_0000_0000),
                            // Have it interact with all memberships except the first two
                            filter: BitMask(0b0011_1111_1111_1111_1111_1111_1111_1111),
                        })
                        .build(&mut context.scene.graph),
                    // Add a 2D rectangle to display our sprite eventually
                    {
                        self.sprite = RectangleBuilder::new(BaseBuilder::new())
                            .build(&mut context.scene.graph);
                        self.sprite
                    },
                ])
                // Add *this* instance of the Enemy script
                // to *this* instance of an Enemy node
                .with_script(Script::new(self)),
        )
        // Remove gravity and lock rotation
        // to ensure the node moves as we want
        .with_gravity_scale(0.)
        .with_rotation_locked(true)
        .build(&mut context.scene.graph)
    }
}

impl_component_provider!(Enemy);

impl TypeUuidProvider for Enemy {
    fn type_uuid() -> Uuid {
        uuid!("52295cf9-80b1-4b90-8504-e27e262713c5")
    }
}

impl ScriptTrait for Enemy {
    fn on_init(&mut self, context: &mut ScriptContext) {
        // Put initialization logic here.
        self.handle = context.handle;
        match context.scene.graph.find_by_name_from_root("Player") {
            Some(handle) => {
                self.player_handle = handle.0;

                for child in handle.1.children().iter() {
                    if let Some(_) = context.scene.graph[*child].cast::<Collider>() {
                        self.player_collider = *child;
                    }
                }
            }
            None => {}
        }
    }

    fn on_start(&mut self, _context: &mut ScriptContext) {
        // There should be a logic that depends on other scripts in scene.
        // It is called right after **all** scripts were initialized.
    }

    fn on_deinit(&mut self, _context: &mut ScriptDeinitContext) {
        // Put de-initialization logic here.
    }

    fn on_os_event(&mut self, _event: &Event<()>, _context: &mut ScriptContext) {
        // Respond to OS events here.
    }

    fn on_update(&mut self, _context: &mut ScriptContext) {
        // Put object logic here.
    }

    fn id(&self) -> Uuid {
        Self::type_uuid()
    }
}
