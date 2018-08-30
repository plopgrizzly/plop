// Copyright Brandon Ly 2018.
// Copyright Jeron A. Lau 2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use prelude::*;
use ami::{ BBox, Collider };
use euler;
use constants;

// a particle is the simplest object that can be simulated in the physics engine
struct Particle {
    // the linear position of the particle in the world space
    position: Vec3;

    // holds the linear velocity of the particle
    velocity: Vec3;

    // holds the acceration of the particle
    acceleration: Vec3;

    // holds the damping applied to linear motion
    // damping is required to remove energy added through
    // numerical instability in the integrator
    damping : f32;
}
