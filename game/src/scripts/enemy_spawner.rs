use fyrox::{
    core::{
        algebra::Vector2,
        pool::Handle,
        reflect::prelude::*,
        uuid::{uuid, Uuid},
        visitor::prelude::*,
        TypeUuidProvider,
    },
    event::Event,
    impl_component_provider,
    rand::{self, Rng},
    scene::{base::BaseBuilder, dim2::rectangle::RectangleBuilder, graph::Graph, node::Node},
    script::{Script, ScriptContext, ScriptDeinitContext, ScriptTrait},
};

use super::Enemy;
use crate::constants::{MAX_MAP_XY_WITH_OFFSET, MIN_MAP_XY_WITH_OFFSET};
use crate::types;

#[derive(Visit, Reflect, Default, Debug, Clone)]
pub struct EnemySpawner {
    // Number of seconds between enemy spawn
    pub spawn_rate: f32,
    pub spawn_timer: f32,

    // Radius of enemy spawn in relation to player
    pub spawn_radius: f32,

    // Enemy properties for spawning
    pub enemy: types::Enemy,

    // Player node handle
    pub player_handle: Handle<Node>,
}

impl EnemySpawner {
    pub fn new() -> Self {
        Self {
            spawn_rate: 0.0,
            spawn_timer: 0.0,
            spawn_radius: 0.0,
            enemy: types::Enemy::new(),
            player_handle: Handle::NONE,
        }
    }

    pub fn with_spawn_rate(mut self, spawn_rate: f32) -> Self {
        self.spawn_rate = spawn_rate + 0.05;
        self
    }

    pub fn with_spawn_radius(mut self, spawn_radius: f32) -> Self {
        self.spawn_radius = spawn_radius;
        self
    }

    pub fn with_enemy(mut self, enemy: types::Enemy) -> Self {
        self.enemy = enemy;
        self
    }

    pub fn build(self, graph: &mut Graph) -> Handle<Node> {
        RectangleBuilder::new(
            BaseBuilder::new()
                .with_script(Script::new(self))
                .with_visibility(false),
        )
        .build(graph)
    }

    // ScriptContext implements three lifetimes, but we don't use them here so leave them anonymous
    fn spawn_enemy(&self, context: &mut ScriptContext<'_, '_, '_>) {
        let mut rng = rand::thread_rng();
        let player_position = context.scene.graph[self.player_handle]
            .local_transform()
            .position();
        let min_x = f32::max(
            player_position.x - self.spawn_radius,
            MIN_MAP_XY_WITH_OFFSET as f32,
        );
        let max_x = f32::min(
            player_position.x + self.spawn_radius,
            MAX_MAP_XY_WITH_OFFSET as f32,
        );
        let min_y = f32::max(
            player_position.y - self.spawn_radius,
            MIN_MAP_XY_WITH_OFFSET as f32,
        );
        let max_y = f32::min(
            player_position.y + self.spawn_radius,
            MAX_MAP_XY_WITH_OFFSET as f32,
        );
        let starting_position =
            Vector2::new(rng.gen_range(min_x..max_x), rng.gen_range(min_y..max_y));
        Enemy::new()
            .with_name(&self.enemy.name)
            .with_speed(self.enemy.speed)
            .with_starting_position(starting_position)
            .with_scale(self.enemy.scale)
            .with_attack_damage(self.enemy.attack_damage)
            .with_attack_speed(self.enemy.attack_speed)
            .build(context);
    }
}

impl_component_provider!(EnemySpawner);

impl TypeUuidProvider for EnemySpawner {
    fn type_uuid() -> Uuid {
        uuid!("745dcf2b-913a-4d6f-9a5c-738acf0a45c4")
    }
}

impl ScriptTrait for EnemySpawner {
    fn on_init(&mut self, context: &mut ScriptContext) {
        // Put initialization logic here.
        match context.scene.graph.find_by_name_from_root("Player") {
            Some(handle) => self.player_handle = handle.0,
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

    fn on_update(&mut self, context: &mut ScriptContext) {
        // Put object logic here.
        self.spawn_timer += context.dt;

        if self.spawn_timer >= self.spawn_rate {
            self.spawn_enemy(context);
            self.spawn_timer = 0.0;
        }
    }

    fn id(&self) -> Uuid {
        Self::type_uuid()
    }
}
