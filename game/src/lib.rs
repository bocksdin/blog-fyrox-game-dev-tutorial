//! Game project.
use constants::{MAP_OFFSET, MAX_MAP_XY, MIN_MAP_XY};
use fyrox::{
    asset::manager::ResourceManager,
    core::{algebra::Vector3, math::Rect, pool::Handle},
    event::Event,
    gui::message::UiMessage,
    plugin::{Plugin, PluginConstructor, PluginContext, PluginRegistrationContext},
    rand::{thread_rng, Rng},
    resource::texture::Texture,
    scene::{
        base::BaseBuilder,
        dim2::{
            collider::ColliderBuilder, rectangle::RectangleBuilder, rigidbody::RigidBodyBuilder,
        },
        graph::Graph,
        rigidbody::RigidBodyType,
        transform::TransformBuilder,
        Scene,
    },
};
use std::path::Path;

mod constants;
mod player;
use player::Player;

pub struct GameConstructor;

impl PluginConstructor for GameConstructor {
    fn register(&self, context: PluginRegistrationContext) {
        // Register your scripts here.
        context
            .serialization_context
            .script_constructors
            .add::<Player>("Player");
    }

    fn create_instance(&self, scene_path: Option<&str>, context: PluginContext) -> Box<dyn Plugin> {
        Box::new(Game::new(scene_path, context))
    }
}

pub struct Game {
    scene: Handle<Scene>,
}

impl Game {
    pub fn new(scene_path: Option<&str>, context: PluginContext) -> Self {
        context
            .async_scene_loader
            .request(scene_path.unwrap_or("data/scene.rgs"));

        Self {
            scene: Handle::NONE,
        }
    }

    pub fn build_tilemap(&mut self, graph: &mut Graph, resource_manager: &ResourceManager) {
        // Load textures once for reuse
        let grass_texture = resource_manager.request::<Texture, _>("data/grass_tileset.png");
        let stone_texture = resource_manager.request::<Texture, _>("data/stone_tileset.png");

        // Build tilemap in x and y directions
        // Add 1 to the max and min values to account for the boundary
        for x in MIN_MAP_XY - 1..=MAX_MAP_XY + 1 {
            for y in MIN_MAP_XY - 1..=MAX_MAP_XY + 1 {
                // Determine x and y position of current tile
                let tile_position = ((x + MAP_OFFSET), (y + MAP_OFFSET));

                // Build positional transform for tile
                let rb_transform = TransformBuilder::new()
                    .with_local_position(Vector3::new(
                        tile_position.0 as f32,
                        tile_position.1 as f32,
                        1.0,
                    ))
                    .build();

                // If the tile is a boundary, build a stone tile
                if x.abs() == MAX_MAP_XY + 1 || y.abs() == MAX_MAP_XY + 1 {
                    // Build a 2D rigid body with a collider and a stone tile sprite
                    RigidBodyBuilder::new(
                        BaseBuilder::new()
                            .with_children(&[
                                // Collider to prevent player from moving past boundary
                                ColliderBuilder::new(BaseBuilder::new()).build(graph),
                                // Stone tile sprite
                                RectangleBuilder::new(BaseBuilder::new())
                                    .with_texture(stone_texture.clone())
                                    // Sprite is located in top left corner of sprite sheet
                                    // Sprite is 96px wide and 96px tall (aka, 37.5% of 256px)
                                    .with_uv_rect(Rect::new(0.0, 0.0, 0.375, 0.375))
                                    .build(graph),
                            ])
                            // Optional, set name of tile
                            .with_name(format!("Boundary ({x}, {y})",))
                            // Set position of tile
                            .with_local_transform(rb_transform),
                    )
                    // Turn off gravity for tile
                    .with_gravity_scale(0.)
                    // Set tile to be static and not rotate
                    .with_rotation_locked(true)
                    .with_body_type(RigidBodyType::Static)
                    .build(graph);
                } else {
                    // Otherwise, build a grass tile

                    // Spritesheet is 8x8, 32px tiles
                    // Select random texture in sprite sheet
                    // Select from top half of sprite sheet
                    let random_tile_x = thread_rng().gen_range(0..=8);
                    let random_tile_y = thread_rng().gen_range(0..=4);

                    // Account for floating point inaccuracy by multiplying by 10000
                    let accurate_x = random_tile_x * 10000;
                    let accurate_y = random_tile_y * 10000;

                    // Convert to f32 and divide by 80000 to get UV coordinates as percentage of full spritesheet
                    // Spritesheet is 8 tiles wide and 8 tiles tall
                    // 8 * 10000 = 80000
                    // Resulting coordinate value will be between 0 and 1, increments of 0.125
                    let random_x_coordinate = (accurate_x as f32) / 80000.;
                    let random_y_coordinate = (accurate_y as f32) / 80000.;

                    // Build a grass tile sprite
                    RectangleBuilder::new(
                        BaseBuilder::new()
                            // Optional, set name of tile
                            .with_name(format!("Tile ({x}, {y})",))
                            // Set position of tile
                            .with_local_transform(rb_transform),
                    )
                    .with_texture(grass_texture.clone())
                    // Sprite is 32px wide and 32px tall (aka, 12.5% of 256px)
                    .with_uv_rect(Rect::new(
                        random_x_coordinate,
                        random_y_coordinate,
                        0.125,
                        0.125,
                    ))
                    .build(graph);
                }
            }
        }
    }
}

impl Plugin for Game {
    fn on_deinit(&mut self, _context: PluginContext) {
        // Do a cleanup here.
    }

    fn update(&mut self, _context: &mut PluginContext) {
        // Add your global update code here.
    }

    fn on_os_event(&mut self, _event: &Event<()>, _context: PluginContext) {
        // Do something on OS event here.
    }

    fn on_ui_message(&mut self, _context: &mut PluginContext, _message: &UiMessage) {
        // Handle UI events here.
    }

    fn on_scene_begin_loading(&mut self, _path: &Path, ctx: &mut PluginContext) {
        if self.scene.is_some() {
            ctx.scenes.remove(self.scene);
        }
    }

    fn on_scene_loaded(
        &mut self,
        _path: &Path,
        scene: Handle<Scene>,
        _data: &[u8],
        context: &mut PluginContext,
    ) {
        self.scene = scene;

        let graph: &mut Graph = &mut context.scenes[self.scene].graph;
        let resource_manager: &ResourceManager = &context.resource_manager;

        // Build Tilemap
        self.build_tilemap(graph, resource_manager);
    }
}
