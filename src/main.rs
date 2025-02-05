// uses std::env;
use std::env;

// main function
fn main() {
    // gets arguments
    let args: Vec<String> = env::args().collect();
    
    // prints help message
    if args.len() == 1 {
        println!("not enough arguments (-h or --help for help)")
    }
    // doesn't echo
    else {
        // returns nothing
        return
    }

// closing curly brackets
}
