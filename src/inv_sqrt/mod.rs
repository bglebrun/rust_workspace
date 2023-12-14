#[cfg(unsafe_code)]
use std::{time::Instant, mem};

// you will never beat a half century of compiler optimizations and chip development
// its so over
// only compile this block if the "unsafe" flag is passed
#[cfg(unsafe_code)]
unsafe fn fast_inv_sqrt(x: f32) -> f32 {
    // I saw the wizard behind the curtain and supposedly we can do mem::transmute to skip additional instructions
    let j = mem::transmute::<u32,f32>(0x5f3759df - (mem::transmute::<f32,u32>(x) >> 1));
    // I literally do not know how to make this faster without the code being unreadable
    j*(1.5-(0.5*x)*j*j)
}
// otherwise, compile this block
#[cfg(not(unsafe_code))]
fn fast_inv_sqrt(x: f32) -> f32 {
    let j = f32::from_bits(0x5f3759df - (x.to_bits() >> 1));
    j*(1.5-(0.5*x)*j*j)
}