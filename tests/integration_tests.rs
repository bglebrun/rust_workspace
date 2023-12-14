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

macro_rules! assert_estimation {
    ($est:expr, $control:expr) => {
        let error = ($est - $control).abs() / $control;
        assert!(error < 0.05);
    };
}

// TODO some sort of significant figures based error estimation

#[cfg(test)]
mod tests {
    // Import the necessary modules or functions from your codebase
    // to be tested in the integration tests
    use super::*;

    #[test]
    fn test_sorts() {
        let mut arr = vec![1, 5, 2, 4, 3];
        let control_array = arr.sort();

        let bubble_sorted_arr = bubble_sort(arr);
        assert_eq!(bubble_sorted_arr, control_array);

        let merge_sorted_arr = merge_sort(arr);
        assert_eq!(merge_sorted_arr, control_array);
    }

    // only compile this block if the target architecture is x86 or x86_64 and the "unsafe" flag is passed
    #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), unsafe_code))]
    unsafe {
        let mem_set = _mm_set_ps(0.0, 0.0, 0.0, x);
        let unsafe_est = _mm_rsqrt_ps(mem_set);
        let est = _mm_cvtss_f32(unsafe_est);
        println!("Answer unsafe {:.32}", est);
        // precision of calculated values vs control
        error_percentage!(est, control);
    }

    #[test]
    fn test_inv_sqrt() {
        let x = 2.0;
        let control = f32::sqrt(x).powi(-1);
        let est = fast_inv_sqrt(x);
        
        error_percentage!(est, control);
        assert_estimation!(est, control);
    }

    // Add more integration tests as needed
}
