
fn main() {
    println!("3.3 Functions");

    // every rust program has one function and you're in it

    // simplest fn declaration
    fn foo() {
        // this throws a warning "never used"
    }

    // with argument, type annotations required
    fn print_number(x: i32) {
        println!("x is: {}", x);
    }
    print_number(5);

    // with two arguments
    fn sum(x: i32, y: i32){
        println!("the sum of x and y is: {}", x + y);
    }
    sum(1,2);

    // returns last line & must be lacking semicolon
    fn add_one(x: i32) -> i32 {
        x + 1
    }
    println!("Add one to 5 to get: {}", add_one(5));

    // There's a difference between an expression and a statement
    // Expressions return a value and statements do not
    // There are only two kinds of statements in rust: declaration statements and expression statements

    // Declaration statements (e.g. variable declaration)
    // js/ruby-like syntax of `let x = y = 5` aren't valid in Rust 
    // let x = (let y = 5); // throws Expected Identifier, found keyword 'let'
    // since a statement doesn't return anything the valid syntax below doesn't do what you'd think it would coming from js/ruby
    let mut y = 5;
    let x = y = 6;
    assert!(x == ()); // an empty struct
    assert!(y == 6);

    // Expression statements (e.g. implied return )
    // the `add_one` fn above uses this, it makes the `x+1` expression into a return statement by omitting the semi-colon

    // Early returns can be done using `return`
    fn foo2(x: i32) -> i32 {
        return x;

        // dead code
        x + 1
    }


    // Function pointers, binding variables to fn's
    let f: fn(i32) -> i32;
    


    // Diverging functions (functions that don't return)
    // the type `!` is read 'diverges'
    fn diverges() -> ! {
        panic!("This function never returns");
    }
    diverges(); //thread 'main' panicked at 'This function never returns'




}


