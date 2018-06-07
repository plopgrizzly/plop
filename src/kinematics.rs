// acceleration due to spring dampering.
pub fn acceleration(x: f32, v: f32, t: f32)
-> f32
    {
    let k:f32 = 15.0;
    let b:f32 = 0.1;

    return -k *x - b * v;
}