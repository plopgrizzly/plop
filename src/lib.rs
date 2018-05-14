extern crate adi_screen;
use adi_screen::Sprite;

const GRAVITY:f32 = 9.81;
const METER:f32 = 1.0;

pub enum Dimensions {
    X,
    Y,
    Z,
}

pub struct RigidBody {
    mass: f32,
    x: f32,
    y: f32,
    z: f32,
    fx: f32,
    fy: f32,
    fz: f32,
    sprite:Sprite,
}

/**
 * This method specifically applies the force of gravity to a given rigidbody
 */
pub fn applyGravity(object: RigidBody) {
   applyForce(object,Dimensions.Y,GRAVITY);
}
/**
 * This function applies a force to a rigidbody with a given dimension
 */
pub fn applyForce(object: RigidBody, dimension: Dimensions, f32 force) {
    if dimension == Dimensions.X {
        object.fx = object.fx - force;
    } else if dimension == Dimensions.Y {
        object.fy = object.fy - force;
    } else if dimension == Dimension.Z {
        object.fz = object.fz - force;
    }
}
