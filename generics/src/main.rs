use rand::Rng;

pub struct User {
    pub username: String,
    pub email: String,
}

impl Login for User {
    fn login(&self) -> Option<i32> {
        let login_id = rand::thread_rng().gen_range(1..=1000);
        if login_id < 500 {
            None
        } else {
            Some(login_id)
        }
    }
}

pub struct Member {
    pub level: i32,
    pub username: String,
    pub email: String,
}

impl Login for Member {}

pub trait Login {
    fn login(&self) -> Option<i32> {
        Some(999)
    }
}

fn copy<T: Clone>(t: T) -> T {    
    t.clone()
}

fn main() {
    let user = User{ username: "jon".to_string(), email: "jon@com".to_string() };
    let member = Member{ level: 22, username: "jon".to_string(), email: "jon@com".to_string() };

    let user_res = user.login().unwrap_or(-1);
    let member_res = member.login().unwrap_or(-1);

    println!("user: {}, member: {}", user_res, member_res);

    let s1: String = String::from("Hello, World!");
    let s2 = copy(&s1);

    println!("{}", s1);
    println!("{}", s2);
}
