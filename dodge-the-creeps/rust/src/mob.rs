//! 玩家必须躲避的敌人，它们的行为很简单：怪物将随机生成在屏幕的边缘，沿着随机的方向进行直线移动

use godot::classes::{AnimatedSprite2D, IRigidBody2D, RigidBody2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=RigidBody2D)]
pub struct Mob {
    pub min_speed: real,
    pub max_speed: real,

    base: Base<RigidBody2D>,
}

#[godot_api]
impl Mob {
    #[func]
    fn on_visibility_screen_exited(&mut self) {
        self.base_mut().queue_free();
    }

    #[func]
    fn on_start_game(&mut self) {
        self.base_mut().queue_free();
    }
}

#[godot_api]
impl IRigidBody2D for Mob {
    fn init(base: Base<RigidBody2D>) -> Self {
        Mob {
            min_speed: 150.0,
            max_speed: 250.0,
            base,
        }
    }

    fn ready(&mut self) {
        // 从三个动画类型之中随机选择一个进行播放
        let mut sprite = self
            .base()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        sprite.play();
        let anim_names = sprite.get_sprite_frames().unwrap().get_animation_names();

        let anim_names = anim_names.to_typed_array();
        let animation_name = anim_names.pick_random().unwrap();

        sprite.set_animation(&Into::<StringName>::into(&animation_name));
    }
}
