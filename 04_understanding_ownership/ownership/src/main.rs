fn main() {
    /*
      Ownership
    */
    let s = "hello"; //literals are immutable
    let st = String::from("hello"); //Strings are mutable

    let x = 5;
    let y = x; //both values are pushed onto stack

    let s1 = String::from("hello");
    let s2 = s1; //copy pointer, length, and capacity that are on the stack
    //println!({}, s1) //s1 is moved to s2, thus invalid

    let s1 = String::from("hello");
    let s2 = s1.clone(); //heap data gets copied
    
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1); //can return in tuples

    /*
      References
    */
    let s1 = String::from("hello");
    let s = &s1; //referencing

    let mut s1 = String::from("hello");
    let mut s = &s1; //mutable referencing
    //let mut r = &s1; //second mutable referencing will cause error

    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so the new reference has no problems
    let r2 = &mut s;

    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point
    let r3 = &mut s; // no problem
    println!("{}", r3);

    //let reference_to_nothing = dangle(); //dangling reference
    let reference_to_something = no_dangle();

    /*
      Slice type
    */
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    let s = String::from("hello");
    let slice = &s[0..2];
    let slice = &s[..2]; //same as above

    let s = String::from("hello");
    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..]; //same as above

    let s = String::from("hello");
    let len = s.len();
    let slice = &s[0..len];
    let slice = &s[..]; //same as above

    let s = String::from("hello");
    let word = first_word(&s);
    //s.clear(); // error!
    
    let my_string = String::from("hello world");
    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);
    let my_string_literal = "hello world";
    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);
    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

//fn dangle() -> &String {
//    let s = String::from("hello");
//
//    &s
//}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
