extern crate adi_screen;
use adi_screen::Sprite;

const GRAVITY:f32 = 9.81;
const METER:f32 = 1.0;
const dt:f32 = 1/100;

pub enum Dimensions {
    X,
    Y,
    Z,
}


/////////////////////////////////////////////////////////////
///                STRUCT DEFINITIONS                     ///  
/////////////////////////////////////////////////////////////

/**
 * The structure that represents physical objects in the gameworld
 */
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
 * This struct will be used to store the position and velocity of 
 * the state values for the RK4 approach
 */
pub struct State {
    x: f32; // position
    v: f32; // velocity

}

/**
 * this struct will be used to store the derivatives of position and velocity of
 * for the RK4 approach
 */
pub struct Derivative {
    dx: f32; // velocity
    dv: f32; // acceleration
}

/////////////////////////////////////////////////////////////
///                 FUNCTION DEFINITIONS                  ///  
/////////////////////////////////////////////////////////////

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
    } else if dimension == Dimensions.Z {
        object.fz = object.fz - force;
    }
}

/**
 * Evaluates and updates the state and derivatives of an object and
 * returns the derivative
 */
pub Derivative evaluate(State: initial, Derivative: d, f32 t, f32 dt) {
    let mut state: State;
}