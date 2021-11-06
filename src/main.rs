use std::rc::Rc;
use std::sync::{Arc,Mutex };

fn main() {
    println!("Hello, world!");
    // Executing the function.
    greet_world();
    process_csv();
    create_integer()
}

fn create_integer(){
    // 1. safety first priority
    // 2. data is immutable by default
    // 3. compile time checks strongly preferred
    // 4. safety should be "Zero-cost abstraction"
    let a = 10;
    println!("Stack-Integer : {:?}",a );

    let b = Box::new(20);
    println!("Heap known as a boxed integer : {:?}",b);

    let c = Rc::new(Box::new(30));
    println!("Boxed integer wrapped within a ref counter : {:?}", c);

    let d = Arc::new(Mutex::new(40));
    println!("Intger wrapped in an atomic ref counter & protected by a mutual exclusion lock {:?} ", d);
}


fn greet_world(){
    println!("Greet World Function Execution started...");
    // Assignment in Rust
    // More properly called variable binding.
    let southern_germany = "Grüß Gott!";
    let japan = "ハロー・ワールド";
    let regions = [ southern_germany, japan];
    // Many type can have iter() to return the iterator
    for region in regions.iter() {
        // & amperdand "borrows" region for the read only access
        println!("{}",&region)
    }
}

fn process_csv(){
    let penguin_data = "/
    common name, length (cm)
    Little penguin,33
    Yello-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";

    let records = penguin_data.lines();
    for (index, record) in records.enumerate(){
        if index == 0 || record.trim().len() == 0 {
            println!("Iteration Skipped because of the whitespace");
            continue;
        }
        // Vec<_> shorthand for vector
        // can expand dynamically
        //  (_) instructs rust to infer type of the elements.
        let fields:Vec<_> = record
            .split(',')
            .map(|f | f.trim())
            .collect();
        // checks the configuration at the compile time.
        if cfg!(debug_assertions) {
            // eprintln1 prints to the standard error (stderr)
            eprintln!("debug: {:?} -> {:?}", record,fields)
        }
        // NOTE :
        // macros are similar to functions except that instead if returning data,
        // these return code. often used to simplify common pattern
        let name = fields[0];
        if let Ok(l) = fields[1].parse::<f32>(){ // attempt to parse field as a floating point number
            // The placeholder {} tekks rust to use a programmer method  to represent the value as string
            // rather than the default representation available with {:?}
            println!("Name :{}, Length : {}", name,l); // this prints to the std out
        }
        if index == 2 {
            break
        }
    }
}

