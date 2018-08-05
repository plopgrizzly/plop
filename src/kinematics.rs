// Copyright Brandon Ly 2018.
// Copyright Jeron A. Lau 2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

/// acceleration due to spring dampering.
pub fn acceleration(x: f32, v: f32, _t: f32) -> f32 {
	let k: f32 = 15.0;
	let b: f32 = 0.1;

	return -k * x - b * v;
}
