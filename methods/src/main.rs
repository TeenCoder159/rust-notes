#[allow(dead_code)]
struct _Rectangle {
    width: u32,
    height: u32,
}
impl _Rectangle {
    fn can_hold(&self, other: &_Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
fn main() {
    let rect1 = _Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = _Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = _Rectangle {
        width: 60,
        height: 40,
    };
    println!("Can rect1 hold rect2 ? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3 ? {}", rect1.can_hold(&rect3));
}
