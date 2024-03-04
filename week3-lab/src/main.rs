use std::env;

// Define a struct
#[derive(Debug)]
struct Sizes {
    bytes: String,
    kilobytes: String,
    megabytes: String,
    gigabytes: String,
}

impl Sizes {
    fn new(bytes: String, kilobytes: String, megabytes: String, gigabytes: String) -> Sizes {
        Sizes {
            bytes,
            kilobytes,
            megabytes,
            gigabytes,
        }
    }   
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // The first argument is the path that was used to call the program.
    println!("My path is {}.", args[0]);

    if args.len() < 2 {
        println!("Call with a command line argument string to the size and units, ie \"64 mb\"");
        return;
    }

    // The second argument is the string we passed to the program.
    // Must use quotes to read this as a single argument
    // You'll need to split the input string to capture the number and the size.
    // You can use the size (e.g. "kb") to match on how to process that number. 
    
    // Split a string on whitesapce
    let size: Vec<&str> = args[1].split_whitespace().collect();
    
    // convert string to integer
    let size_int = size[0].parse::<f64>().unwrap();
    
    let unit = size[1].to_string().to_lowercase();

    let bytes: f64 = match unit.as_str() {
        "b" => size_int,
        "kb" => size_int * 1000.0,
        "mb" => size_int * 1_000_000.0,
        "gb" => size_int * 1_000_000_000.0,
        _ => 0.0,
    };

    let my_sizes = Sizes::new(
        format!("{} bytes", bytes),
        format!("{:.2} KB", bytes / 1000.0),
        format!("{:.2} MB", bytes / 1_000_000.0),
        format!("{:.2} GB", bytes / 1_000_000_000.0),
    );

    println!("{:?}.", my_sizes);
}
