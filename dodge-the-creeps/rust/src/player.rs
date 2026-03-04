use godot::classes::{AnimatedSprite2D, Area2D, CollisionShape2D, IArea2D, Input};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Player {
    /// How fast the player moves in meters per second.
    #[export]
    speed: real,
    /// The size of the viewport in pixels.
    screen_size: Vector2,
    /// The base node of the player.
    base: Base<Area2D>,
}

#[godot_api]
impl Player {
    // Public signal, since it's used by Main struct.
    #[signal]
    pub fn hit();

    #[func]
    fn on_player_body_entered(&mut self, _body: Gd<Node2D>) {
        // 看起来要做到什么和引擎有关的事都需要借用基类来进行，倒是在GDScript之中
        // 是可以直接调用的，现在想起来是当然的，因为GDScript实际上也可以算作一个面向
        // 对象的语言，而在这里Rust实际上是在模拟面向对象，所以隐式的调用父类的方式在这
        // 里必须是显式的
        self.base_mut().hide();
        self.signals().hit().emit();

        // 在GDScript之中，这里直接使用了`$CollisionShape2D`来获取节点，这是语法糖
        // 展开之后就是`get_node("CollisionShape2D")`，Rust之中没有这种语法糖
        let mut collision_shape = self
            .base()
            .get_node_as::<CollisionShape2D>("CollisionShape2D");

        // 在当前帧的末尾，将给定属性`propertye`的值分配为对应的`value`，因为在引擎的碰撞处理过程中
        // 禁用区域的碰撞形状可能会导致错误。需要告诉Godot等待可以安全地禁用形状时再这样做
        collision_shape.set_deferred("disabled", &true.to_variant());
    }

    #[func]
    pub fn start(&mut self, pos: Vector2) {
        self.base_mut().set_global_position(pos);
        self.base_mut().show();

        let mut collision_shape = self
            .base()
            .get_node_as::<CollisionShape2D>("CollisionShape2D");

        collision_shape.set_disabled(false);
    }
}

#[godot_api]
impl IArea2D for Player {
    fn init(base: Base<Area2D>) -> Self {
        Player {
            speed: 400.0,
            screen_size: Vector2::new(0.0, 0.0),
            base,
        }
    }

    fn ready(&mut self) {
        let viewport = self.base().get_viewport_rect();
        self.screen_size = viewport.size;
        self.base_mut().hide();

        // Signal setup
        self.signals()
            .body_entered()
            .connect_self(Self::on_player_body_entered);
    }

    // `delta` can be f32 or f64; #[godot_api] macro converts transparently.
    fn process(&mut self, delta: f32) {
        let mut animated_sprite = self
            .base()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        let mut velocity = Vector2::new(0.0, 0.0);

        // Note: exact=false by default, in Rust we have to provide it explicitly
        let input = Input::singleton();
        if input.is_action_pressed("move_right") {
            velocity += Vector2::RIGHT;
        }
        if input.is_action_pressed("move_left") {
            velocity += Vector2::LEFT;
        }
        if input.is_action_pressed("move_down") {
            velocity += Vector2::DOWN;
        }
        if input.is_action_pressed("move_up") {
            velocity += Vector2::UP;
        }

        if velocity.length() > 0.0 {
            velocity = velocity.normalized() * self.speed;

            let animation;

            if velocity.x != 0.0 {
                animation = "right";

                animated_sprite.set_flip_v(false);
                animated_sprite.set_flip_h(velocity.x < 0.0)
            } else {
                animation = "up";

                animated_sprite.set_flip_v(velocity.y > 0.0)
            }

            animated_sprite.play_ex().name(animation).done();
        } else {
            animated_sprite.stop();
        }

        let change = velocity * delta;
        let position = self.base().get_global_position() + change;
        let position = Vector2::new(
            position.x.clamp(0.0, self.screen_size.x),
            position.y.clamp(0.0, self.screen_size.y),
        );
        self.base_mut().set_global_position(position);
    }
}
