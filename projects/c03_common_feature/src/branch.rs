// branch.rs
fn main() {
    let condition = false;
    let number = if condition {
        let x = 4;
        x + 1
    } else {
        let y = 6;
        y + 1
    };
    // numberの値は, {}です
    println!("The value of number is: {number}");
}