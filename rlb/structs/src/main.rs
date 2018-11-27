fn main() {
    let user1 = User {
        email: String::from("rahul.bhatia.k@gmail.com"),
        username: String::from("rahul.bhatia.k"),
        active: true,
        sign_in_count: 1
    };

    println!("First user email: {}", user1.email);

    let user2 = build_user(String::from("test_email@gmail.com"), 
            String::from("test_username"));

    println!("Second user email: {}", user2.email);

    let user3 = User {
        email: String::from("another@example.com"), 
        username: String::from("anotheruser"),
        ..user1
    };
    println!("Third user email: {}", user3.email);

    let black = Color(0,0,0);
    let origin = Point(0,0,0);
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// there is an intentional use of String not &str
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn build_user(email: String, username: String) -> User {
    User {
        email, 
        username, 
        active: true,
        sign_in_count: 1
    }
}

//structure 
struct Rectangle {
    width: u32,
    height: u32,
}

//structure methods
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn square(edge: u32) -> Rectangle {
        Rectangle {
            width: edge,
            height: edge,
        }
    }
}
