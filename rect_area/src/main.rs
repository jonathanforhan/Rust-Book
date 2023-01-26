#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rect) -> bool {
        if self.width > other.width && self.height > other.height {
            true
        } else {
            false
        }
    }
}

fn main() {

    let s = String::from("Hello");

    let rect1 = Rect {
        width: 30,
        height: 50
    };
    let rect2 = Rect {
        width: 10,
        height: 10
    };
    let rect3 = Rect {
        width: 50,
        height: 50
    };

    println!("Rect1 can hold Rect2 is {}", rect1.can_hold(&rect2));
    println!("Rect2 can hold Rect3 is {}", rect2.can_hold(&rect3));
}
