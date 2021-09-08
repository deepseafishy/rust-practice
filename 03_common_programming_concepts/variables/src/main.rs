fn five() -> i32 {
    5
}

fn main() {
    /*
      Immutable and mutable
    */
    let x = 5;
    println!("The value of x is: {}", x);
    //x = 6; //This doesn't work for immutable variables
    
    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 6; //This works as y is mutable
    println!("The value of y is: {}", y);

    /*
      Constants
    */
    const MAX_POINTS: u32 = 100_000;
    
    /*
      Shadowing
    */
    let z = 5;
    let z = z + 1;
    let z = z * 2;
    println!("The value of z is: {}", z);

    let spaces = "   ";
    let spaces = spaces.len();

    let spaces = "   ";
    //spaces = spaces.len(); //Cannot mutate a variable's type

    /*
      Floating Points
    */
    let x = 2.0; //f64
    let y: f32 = 3.0; //f32

    /*
      Numeric Operations
    */
    let sum = 5 + 1;
    let difference = 95.5 - 4.3;
    let product = 4 * 40;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    /*
      Boolean
    */
    let t = true;
    let f: bool = false; //With explicit type annotation

    /*
      Characters
    */
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    /*
      Tuple
    */
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup;
    println!("The value of b is : {}", b);
    println!("The value of b is : {}", tup.1);

    /*
      Array
    */
    let arr = [1, 2, 3, 4, 5];
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let arr = [3; 5];
    let first = arr[0];

    /*
      Functions
    */
    let x = five();

    /*
      if Expressions
    */
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    /*
      Loops
    */
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    /*
      while Loops
    */
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }
    println!("LIFTOFF!!!");

    /*
      for Loops
    */
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}































