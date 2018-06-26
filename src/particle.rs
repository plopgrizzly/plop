// The Cala Physics Engine
//
// Copyright Jeron A. Lau 2018.
// Copyright Brandon Ly 2018.
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use prelude::*;
use ami::{ BBox, Collider };
use adi_screen::{ Window, Sprite, Transform };
use euler;
use constants;

// a particle is the simplest object that can be simulated in the physics engine
struct Particle {
    // the linear position of the particle in the world space
    position: Vec3;

    // holds the linear velocity of the particle
    velocity: Vec3;

    // holds the accer
}