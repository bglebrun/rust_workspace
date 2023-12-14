#[cfg(all(target_arch = "x86", unsafe_code))]
use std::arch::x86::*;
#[cfg(all(target_arch = "x86_64", unsafe_code))]
use std::arch::x86_64::*;


// error estimation macro
macro_rules! error_percentage {
    ($est:expr, $control:expr) => {
        let error = ($est - $control).abs() / $control;
        let error_percentage = format!("{:.2}", error*100.0);
        println!("Error: {} %", error_percentage);
    };
}

#[cfg(not(unsafe_code))]
use std::time::Instant;
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

pub fn test_sqrt(x:f32) {

    let control_time = Instant::now();
    let control = f32::sqrt(x).powi(-1);
    let test_elapsed = control_time.elapsed();
    println!("Control Answer {:.32}", control);
    println!("Control: {:.2?}", test_elapsed);

    // only compile if the "unsafe" flag is passed
    #[cfg(unsafe_code)]
    unsafe {
        let now = Instant::now();
        let est = fast_inv_sqrt(x);
        let elapsed = now.elapsed();
        println!("Answer {:.32}", est);
        println!("Elapsed: {:.2?}", elapsed);
        error_percentage!(est, control);
    }
    #[cfg(not(unsafe_code))]
    {
        let now = Instant::now();
        let est = fast_inv_sqrt(x);
        let elapsed = now.elapsed();
        println!("Answer {:.32}", est);
        println!("Elapsed: {:.2?}", elapsed);
        error_percentage!(est, control);
    }
    
    // only compile this block if the target architecture is x86 or x86_64 and the "unsafe" flag is passed
    #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), unsafe_code))]
    unsafe {
        let lock_n_load = _mm_set_ps(0.0, 0.0, 0.0, x);
        let now = Instant::now();
        let unsafe_est = _mm_rsqrt_ps(lock_n_load);
        let elapsed = now.elapsed();
        let est = _mm_cvtss_f32(unsafe_est);
        println!("Answer unsafe {:.32}", est);
        println!("Elapsed unsafe: {:.2?}", elapsed);
        // precision of calculated values vs control
        error_percentage!(est, control);
    }
}