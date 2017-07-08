#![allow(unused_variables)]
fn main(){
    println!("3.2 Data Types");

    // two subsets of types, scalar and compound

    // most types can be inferred, but if unclear must be annotated
    // .e.g. parsing a Number from a String
    let guess: u32 = "42".parse().expect("not a number");

    // a scalar type represents a single value
    // primary scalar types: integer, floating-point numbers, booleans, and characters

    /*Integer*/
    // 8 integer types, 4 main with signed/unsigned variants
    // Rust defaults to i32
    // Each signed variant can store numbers from -(2^n-1) to 2^n-1 - 1 inclusive
    // Unsigned variants can store numbers from 0 to 2^n - 1
    // 8-bit have a range of 255
    let neg8bit: i8 = -1;
    let u8bit: u8 = 1;

    // 16-bit have a range of 65536
    let neg16bit: i16 = -1000;
    let u16bit: u16 = 1000;

    // 32-bit have a range of 4,294,967,296
    let neg32bit: i32 = -0xABCDEF;
    let u32bit: u32 = 0xABCDEF;

    // 64-bit have a range of 18,446,744,073,709,552,000
    let neg64bit: i64 = -0x7ffffffff;
    let u64bit: u64 = 0x7ffffffff;

    // the arch integer types, viz isize & usize, are based on architecture


    /*Floating-point*/
    // f32 and f64
    // defaults to f64 b/c speed is close to f32 but with greater precision
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    /*Numeric operations*/
    // +,-,*,/,%  enough said

    /*Boolean*/
    // type is `bool`, values can only be `true` or `false`
    let t = true;
    let f: bool = false; // with explicit type annotation

    /*Character*/
    // denoted by single quotes, is a Unicode Scalar Value, which range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive
    let c = 'a';
    let d: char = 'b';


    /* Compound types */
    // collections of other types
    // primitive compound types: tuples & arrays

    /*Tuple*/
    let tup: (i32, char, f64) = (500, 'y', 6.4);

    // retrieving values from a tuple using destructuring and 0-based index
    let (num, yes, float) = tup;
    assert!(num == tup.0);
    assert!(yes == tup.1);
    assert!(float == tup.2);

    /*Array*/
    // elements of same-type
    // fixed length
    let a = [1,2,3,4,5];
    // accessed by index using square brackets instead of dots
    assert!(a[0] == 1);
    // most of the time you will use a vector for the arrays you'd use in other languages

    // out of bounds index access will compile but panic! at run time
}