extern crate adi_screen;
use adi_screen::Sprite;

const GRAVITY:f32 = 9.81;
const METER:f32 = 1.0;

pub struct RigidBody {
    mass:f32,
    x:f32,
    y:f32,
    z:f32,
    sprite:Sprite,
}

pub fn force(object: RigidBody) {

}