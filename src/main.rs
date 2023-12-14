mod inv_sqrt;

fn main() {
    // get command line arguments
    let args: Vec<String> = std::env::args().collect();

    // cmd line will be test_sqrt x
    // where x is a float
    if args.len() != 3 {
        println!("Usage: test_sqrt x");
        std::process::exit(1);
    }
    let mut x = args[2].parse::<f32>().unwrap();
    
    // if x is negative, make it positive
    if x < 0.0 {
        x = -x;
    }
    // output architecture
    println!("Target architecture: {}", std::env::consts::ARCH);
    inv_sqrt::test_sqrt(x);
}

