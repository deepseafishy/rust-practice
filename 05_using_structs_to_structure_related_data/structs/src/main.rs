#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn can_hold2(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    /*
      Structs
    */
    let user1 = User {
        email: String::from("example@eg.com"),
        username: String::from("example"),
        active: true,
        sign_in_count: 1
    };
    
    let mut user1 = User {
        email: String::from("example@eg.com"),
        username: String::from("example"),
        active: true,
        sign_in_count: 1
    };

    user1.email = String::from("another_example@eg.com");

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    
    //Same as above
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    println!("user2 is {:?}", user2);
    println!("user2 is {:#?}", user2);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
