#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
// ARM and ARM64 do not implement the mm_rsqrt functionality
#[cfg(target_arch = "arm")]
use std::arch::arm::*;
#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::*;

// error estimation macro
macro_rules! error {
    ($est:expr, $control:expr) => {
        let error = ($est - $control).abs() / $control;
        let error_percentage = format!("{:.2}", error*100.0);
        println!("Error: {} %", error_percentage);
    };
}

use std::{time::Instant, mem};

// you will never beat a half century of compiler optimizations and chip development
// its so over
unsafe fn fast_inv_sqrt(x: f32) -> f32 {
    // I saw the wizard behind the curtain and supposedly we can do mem::transmute to skip additional instructions
    let j = mem::transmute::<u32,f32>(0x5f3759df - (mem::transmute::<f32,u32>(x) >> 1));
    // I literally do not know how to make this faster without the code being unreadable
    j*(1.5-(0.5*x)*j*j)
}

pub fn test_sqrt(x:f32) {

    let control_time = Instant::now();
    let control = 1.0 / f32::sqrt(x);
    let test_elapsed = control_time.elapsed();
    println!("Control Answer {:.32}", control);
    println!("Control: {:.2?}", test_elapsed);

    unsafe {
        let now = Instant::now();
        let est = fast_inv_sqrt(x);
        let elapsed = now.elapsed();
        println!("Answer {:.32}", est);
        println!("Elapsed: {:.2?}", elapsed);
        error!(est, control);
    }
    
    // only compile this block if the target architecture is x86 or x86_64
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    unsafe {
        let lock_n_load = _mm_set_ps(0.0, 0.0, 0.0, x);
        let now = Instant::now();
        let unsafe_est = _mm_rsqrt_ps(lock_n_load);
        let elapsed = now.elapsed();
        let est = _mm_cvtss_f32(unsafe_est);
        println!("Answer unsafe {:.32}", est);
        println!("Elapsed unsafe: {:.2?}", elapsed);
        // precision of calculated values vs control
        error!(est, control);
    }
}