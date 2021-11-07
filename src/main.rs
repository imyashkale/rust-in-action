use std::convert::TryInto;
use num::complex::Complex;
use std::time::{ Duration, Instant};

fn main() {
    // let result = add_with_lifetimes(&1,&2);
    // println!("{}",result)
    addgen(1,2);
    // let (result , ok) = functions(1,2);
    // println!("Result : {} {}", result, ok);
}

pub fn addgen<T: std::ops::Add<Output = T> >(i: T,j: T) -> T {
    return i + j;
}

pub fn add_with_lifetimes<'a, 'b> (i: &'a i32, j: &'b i32) -> i32 {
    return *i + *j
}


pub fn references(){
    // basic
    let a:i8 = 40;
    let r:&i8 = &a; // refference created.
    let b:i8 = a + *r; // dereferencing.
    println!("Addition After deref : {}", b);
}

pub fn functions(first:i32, second: i32 ) ->( i32 , bool){
    println!("First : {} Second : {} ", first, second);
    return( first + second , true)
}

pub fn pattern_matching(){
    let item = 1;
        match item {
            0 => {
            }, // single match
            10 ..= 20 => {}, // inclusive range
            40 | 80 => {}, // Either side of it.
            _ => {} // every value
    }

    let _needle = 42;
    let haystack = [ 1,1,2,5,14,42,132,429,1430,4862 ];
    const HIT: &str = "Hit!";
    for item in &haystack {
        let result = match item {
            42 | 132 => HIT,
            _ => "Miss",
        };
        if result == HIT {
            println!("{}: {}",item,result);
        };
    }
}

pub fn break_returns(){
    let number = loop {
        break 123;
    };
    println!("Berak Returns : {}", number);
}

pub fn match_this(){
    let n = 122;
    let description = match is_even(n){
        true => "EVEN",
        false => "ODD"
    };
    println!("Using Match : {}", description);
}

pub fn expr(){
    let n = 123456;
    let description = if is_even(n) {
        "Even"
    } else {
        "Odd"
    };

    println!("After Evaluation : {}", description);
}

pub fn whileloop(){
    let mut count: i32 = 0;
    let time_limit:Duration = Duration::new(1, 0);
    let start:Instant = Instant::now();

    while ( Instant::now() - start ) < time_limit {
        count += 1;
    }
    println!("{}", count)
}

pub fn forloop(){
    let a = [1,2,3];
    // Ownership
    for item in a { // equivalent to IntiIterator::into_iter(a)
        println!("Ownership : {}", item + 1);
    }
    println!("{:?}",a);

    let b = [ 4,5,6];
    for item in &b { // equivalent to b.iter()
        println!("Read-Only : {}",item +1)
    }
    println!("{:?}",b);

    let mut c = [7,8,9];
    for item in &mut c { // equivalent to c.iter_mut()
        println!("Read-Write : {}", *item + 1 );
    }
    println!("{:?}",c);

    for _ in 1..3{
        println!("Iterating ...")
    }

    let collection = [10,20,30,40,50];
    for index in 0..collection.len(){
        let item = collection[index];
        println!("Index : {} , Current Item : {}",index, item);
    }
}


pub fn other_numbers(){
    // ration numbers
    // complex numbers
    let a = Complex{
        re:2.1,
        im: -1.2
    };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;
    println!("{} + {}i", result.re, result.im);
}

pub fn comparing_numbers(){
    println!("Comparing Numbers Function Execution Started ");
    // < , > , == , != , <= , >=
    // cannot compare diff types
    let a: i32 = 10;
    let b:u16 = 100;

    // if a < ( b as i32)  { // promotion
    //     println!("Ten is less than one hundred. ")
    // }

    let b_ = b.try_into().unwrap();
    if a < b_ {
        println!("Ten is less than one hundred. ")
    }
}

pub fn num_operations(){
    // Operations on numbers use infix notation
    // Numeric expressions are simmilar to other programming lang.

    let twenty = 20;
    let twenty_one: i32 = 21;
    let twenty_two = 22i32;

    let addition = twenty + twenty_one + twenty_two;
    println!("{} + {} + {} = {}", twenty, twenty_one,twenty_two, addition);

    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    let float_array: [f32;3] = [ 42.00, 43f32,42.0_f32 ];
    println!("{:02} ", float_array[0]);

    let three = 0b11; // binary base 2
    let thirty = 0o36; // 0o indicates octal (base 8) numerals
    let three_hundred = 0x12C; // hexadecimal ( base 16 )
    println!("{} {} {}",three, thirty,three_hundred);

    println!("base 10 : {}-{}-{}",three,thirty,three_hundred );
    println!("base 2  : {:b}-{:b}-{:b}",three,thirty,three_hundred);
    println!("base 8  : {:o}-{:o}-{:o}",three,thirty,three_hundred);
    println!("base 16 : {:x}-{:x}-{:x}", three,thirty,three_hundred);

    // signed integers
    let signed_integers:[&str;4] = [ "i8", "i16", "i32","i64" ];
    for number in signed_integers.iter() {
        println!("Signed Number Type : {}", number);
    }
    // unsigned integers
    let unsigned_integers:[&str;4] = ["u8","u16","u32","u64"];
    for unumber in unsigned_integers.iter(){
        println!("Unsigned Number Type : {}", unumber);
    }
    // float numbers
    let float_types:[&str;2] = ["f32", "f64"];
    for fnumber in float_types.iter(){
        println!("F Number Type : {}", fnumber);
    }

    let inumbers = [ "isize","usize" ];
    for inumber in inumbers.iter(){
        println!("I Number Type : {}", inumber);
    }
}


pub fn numbers(){
    // let to declare variable bindings
    // immutable by default. meaning read only
    // stmt are delimited with smicolons ( ; )
    let a = 10; // type of the value will be inferred.
    let b: i32 = 20; // self declaration or by programmer
    let c = 30i32; //Numeric type call include a type annotation in their literal form.
    let d = 30_i32; // can include _ for readability
    let e = add(add(a,b),add(c,d));
    println!( "(a +b ) + (c + d ) = {}", e );

    let x = 1_00_000;
    println!("Number included with _ : {}",x);
}

fn add(i: i32,j: i32) -> i32 { // function type declaration
    i + j // return the last experation in the function.
    // return keyword not required.
}

pub fn is_even(number: i32) -> bool {
    return number % 2 == 0;
}