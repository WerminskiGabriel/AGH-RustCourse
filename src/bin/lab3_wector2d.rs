use std::fmt::{Display, Formatter};

struct Vec2D {
    x: f32,
    y: f32,
}
impl Display for Vec2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)?;
        Ok(())
    }
}

use std::ops::{Add, Sub};

impl Add for &Vec2D {
    type Output = Vec2D;

    fn add(self, other: &Vec2D) -> Vec2D {
        Vec2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl Sub for &Vec2D {
    type Output = Vec2D;

    fn sub(self, other: &Vec2D) -> Vec2D {
        Vec2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl PartialEq for Vec2D {
    fn eq(&self, other: &Vec2D) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn main() {
    let v3_2 = Vec2D { x: 3.0, y: 2.0 };
    let v_5__2 = Vec2D { x: -5.0, y: -2.0 };
    println!( "\nA : {}\nB : {}\n", v3_2 ,v_5__2 );

    println!( "A + B : {}", &v3_2 + &v_5__2 );

    println!( "A - B : {}", &v3_2 - &v_5__2 );

    println!( "A == B : {}", &v3_2 == &v_5__2 );

}
