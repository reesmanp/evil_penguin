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

#[derive(Eq, PartialEq, Debug)]
enum Orientation {
    Colinear,
    Clockwise,
    CounterClockwise
}

impl Orientation {
    fn get_orientation(value: f32) -> Self {
        if value == 0.0 {
            Self::Colinear
        } else if value > 0.0 {
            Self::Clockwise
        } else {
            Self::CounterClockwise
        }
    }
}

fn get_points(object: TextureCoordinates) -> Vec<(f32, f32)> {
    vec![
        (object.left, object.top),
        (object.right, object.top),
        (object.left, object.bottom),
        (object.right, object.bottom)
    ]
}

fn max_float(num1: f32, num2: f32) -> f32 {
    if num1 > num2 {
        return num1;
    }

    num2
}

fn min_float(num1: f32, num2: f32) -> f32 {
    if num1 > num2 {
        return num2;
    }

    num1
}

fn on_segment(p1: (f32, f32), p2: (f32, f32), p3: (f32, f32)) -> bool {
    if p2.0 <= max_float(p1.0, p3.0) && p2.0 >= min_float(p1.0, p3.0) &&
        p2.1 <= max_float(p1.1, p3.1) && p2.1 >= min_float(p1.1, p3.1) {
        return true;
    }

    false
}

fn get_orientation(p1: (f32, f32), p2: (f32, f32), p3: (f32, f32)) -> Orientation {
    Orientation::get_orientation(
        (p2.1 - p1.1) * (p3.0 - p2.0)
            - (p2.0 - p1.0) * (p3.1 - p2.1)
    )
}

fn do_intersect(line_1: &((f32, f32), (f32, f32)), line_2: &((f32, f32), (f32, f32))) -> bool {
    let o1 = get_orientation(line_1.0, line_2.0, line_1.1);
    let o2 = get_orientation(line_1.0, line_2.0, line_2.1);
    let o3 = get_orientation(line_1.1, line_2.1, line_1.0);
    let o4 = get_orientation(line_1.1, line_2.1, line_2.0);

    if o1 != o2 && o3 != o4 {
        println!("{:?} {:?}", line_1, line_2);
        println!("{:?} {:?} {:?} {:?}", o1, o2, o3, o4);
        return true;
    }

    if o1 == Orientation::Colinear && on_segment(line_1.0, line_1.1, line_2.0) {
        return true;
    } else if o2 == Orientation::Colinear && on_segment(line_1.0, line_2.1, line_2.0) {
        return true;
    } else if o3 == Orientation::Colinear && on_segment(line_1.1, line_1.0, line_2.1) {
        return true;
    } else if o4 == Orientation::Colinear && on_segment(line_1.1, line_2.0, line_2.1) {
        return true;
    }

    false
}

pub fn is_collision(object1: TextureCoordinates, object2: TextureCoordinates) -> bool {
    let object_1_points = get_points(object1);
    let object_2_points = get_points(object2);

    let object_1_lines = vec![
        (object_1_points[0], object_1_points[1]),
        (object_1_points[0], object_1_points[2]),
        (object_1_points[1], object_1_points[3]),
        (object_1_points[2], object_1_points[3])
    ];

    let object_2_lines = vec![
        (object_2_points[0], object_2_points[1]),
        (object_2_points[0], object_2_points[2]),
        (object_2_points[1], object_2_points[3]),
        (object_2_points[2], object_2_points[3])
    ];

    for line_1 in &object_1_lines {
        for line_2 in &object_2_lines {
            if do_intersect(line_1, line_2) {
                return true;
            }
        }
    }

    false
}

pub fn get_sprite_coordinates(transform: &Transform, sprite: &Sprite) -> TextureCoordinates {
    let translation = transform.translation();
    let scale = transform.scale();

    TextureCoordinates {
        left: translation.x,
        right: translation.x + sprite.width * scale.x * 0.5,
        bottom: translation.y,
        top: translation.y + sprite.height * scale.y * 0.5
    }
}
