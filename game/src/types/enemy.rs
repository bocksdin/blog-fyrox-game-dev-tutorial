use fyrox::core::{reflect::prelude::*, visitor::prelude::*};

// Visit and Reflect are required for Fyrox structs
#[derive(Default, Visit, Reflect, Debug, Clone)]
pub struct Enemy {
    // Name of Node
    pub name: String,
    // Movement speed
    pub speed: f32,
    // Rectangle node scaling
    pub scale: f32,
    // Attack damage
    pub attack_damage: f32,
    // Attack speed
    pub attack_speed: f32,
}

impl Enemy {
    pub fn new() -> Self {
        // Build with default values
        Self {
            name: "".to_owned(),
            speed: 0.0,
            scale: 1.0,
            attack_damage: 0.0,
            attack_speed: 0.0,
        }
    }

    // Define name of Node
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_owned();
        self
    }

    // Define movement speed
    pub fn with_speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }

    // Define rectangle node scaling
    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }

    // Define attack damage
    pub fn with_attack_damage(mut self, attack_damage: f32) -> Self {
        self.attack_damage = attack_damage;
        self
    }

    // Define attack speed
    pub fn with_attack_speed(mut self, attack_speed: f32) -> Self {
        self.attack_speed = attack_speed;
        self
    }
}
