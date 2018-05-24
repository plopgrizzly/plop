extern crate adi_screen;
use adi_screen::Sprite;

mod window;

pub use window::Window;

const GRAVITY: f32 = 9.81;
const METER: f32 = 1.0;
const DT: f32 = 1f32/100f32;

#[derive(PartialEq)]
pub enum Dimensions {
	X,
	Y,
	Z,
}

/****************************************
 ********** STRUCT DEFINITIONS **********
 ****************************************/

/// The structure that represents physical objects in the gameworld
pub struct RigidBody {
	mass: f32,
	x: f32,
	y: f32,
	z: f32,
	fx: f32,
	fy: f32,
	fz: f32,
	sprite: Sprite,
}

/// This struct will be used to store the position and velocity of the state
/// values for the RK4 approach
pub struct State {
	x: f32, // position
	v: f32, // velocity
}

/// this struct will be used to store the derivatives of position and velocity
/// of for the RK4 approach
pub struct Derivative {
	dx: f32, // velocity
	dv: f32, // acceleration
}

/****************************************
 ********* FUNCTION DEFINITIONS *********
 ****************************************/

/// This method specifically applies the force of gravity to a given rigidbody
pub fn applyGravity(object: &mut RigidBody) {
	applyForce(object, Dimensions::Y, GRAVITY);
}

/// This function applies a force to a rigidbody with a given dimension
pub fn applyForce(object: &mut RigidBody, dimension: Dimensions, force: f32) {
	if dimension == Dimensions::X {
		object.fx = object.fx - force;
	} else if dimension == Dimensions::Y {
		object.fy = object.fy - force;
	} else if dimension == Dimensions::Z {
		object.fz = object.fz - force;
	}
}

/// Evaluates and updates the state and derivatives of an object and returns the
/// derivative
pub fn evaluate(_initial: State, _d: Derivative, _t: f32, _dt: f32) -> Derivative {
	let mut _state: State;
	unreachable!()
}
