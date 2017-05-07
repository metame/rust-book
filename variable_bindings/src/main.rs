fn main() {
    println!("3.1 Variable Bindings");

    let x = 5; //basic variable binding with type inference
    assert!(x == 5);

    let (x, y) = (1, 2); // left-hand assignment is a pattern, not a name
    assert!(x == 5 && y == 2);

    let x: i32 = 5; //binding with type annotation
    assert!(x == 5);

    /*
    let x = 5;
    x = x + 5; causes re-assignment err
    */
    
    let mut x = 5;
    x = x + 5;
    assert!(x == 10);

    let x: i32; // rust allows undeclared values
    /*
    println!("The value of x is: {}", x);
    but doesn't allow you to use them
    this throws err: possibly uninitialized value
    */

    // bindings are block-scoped
    let x = 5;
    {
        assert!(x == 5);
        let x = 10;
        assert!(x == 10);
    }
    assert!(x == 5);

    // shadowing
    let x = 5;
    assert!(x == 5);
    let x = "yay!";
    assert!(x == "yay!");
    // it's what I've done in this whole terrible program
    // all of these values that have been assigned to x still exist even though I can only access the last value x has been assigned to
}
