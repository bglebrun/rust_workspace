#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
// ARM and ARM64 do not implement the mm_rsqrt functionality
#[cfg(target_arch = "arm")]
use std::arch::arm::*;
#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::*;

use std::time::Instant;

// you will never beat a half century of compiler optimizations and chip development
// its so over
fn fast_inv_sqrt(x: f32) -> f32 {
    let j = f32::from_bits(0x5f3759df - (x.to_bits() >> 1));
    j*(1.5-(0.5*x)*j*j)
}

pub fn test_sqrt(x:f32) {

    let now = Instant::now();
    let est = fast_inv_sqrt(x);
    let elapsed = now.elapsed();
    println!("Answer {:.32}", est);
    println!("Elapsed: {:.2?}", elapsed);
    
    let new_now = Instant::now();
    let control = 1.0 / f32::sqrt(x);
    let test_elapsed = new_now.elapsed();
    println!("Control Answer {:.32}", control);
    println!("Control: {:.2?}", test_elapsed);


    let precision = est - control/est;
    let precision_percentage = format!("{:.2}", precision.abs()*100.0);
    println!("Precision fast_sqrt: {} %", precision_percentage);
    
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
        let precision = est - control/est;
        let precision_percentage = format!("{:.2}", precision.abs()*100.0);
        println!("Precision intrinsic: {} %", precision_percentage);
    }
}