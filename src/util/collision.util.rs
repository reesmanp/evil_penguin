use amethyst::{
    renderer::sprite::{
        TextureCoordinates,
        Sprite
    },
    core::{
        transform::Transform,
        math::Vector2
    }
};

pub fn is_collision(object1: TextureCoordinates, object2: TextureCoordinates) -> bool {
    (object1.left >= object2.left && (
        (point_in_range(object1.top, Vector2::new(object2.top, object2.bottom))) ||
            (point_in_range(object1.bottom, Vector2::new(object2.top, object2.bottom)))
    ) && object1.left <= object2.right)
        ||
        (object1.right <= object2.right &&
            ((point_in_range(object1.top, Vector2::new(object2.top, object2.bottom))) ||
                (point_in_range(object1.bottom, Vector2::new(object2.top, object2.bottom)))
            ) && object1.right >= object2.left)
}

fn point_in_range(a: f32, b: Vector2<f32>) -> bool {
    (a <= b.x && a >= b.y) || (a >= b.x && a <= b.y)
}

pub fn get_sprite_coordinates(transform: &Transform, sprite: &Sprite) -> TextureCoordinates {
    let translation = transform.translation();
    let scale = transform.scale();

    TextureCoordinates {
        left: translation.x,
        right: translation.x + sprite.width * scale.x,
        bottom: translation.y,
        top: translation.y + sprite.height * scale.y
    }
}
