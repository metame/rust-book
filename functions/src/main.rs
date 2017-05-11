
fn main() {
    println!("3.2 Functions");

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

}


