use add_one;
use add_two;

fn main() {
    let num = 10;
    println!("Hello, world! {} plus one is {}", num, add_one::add_one(num));
    println!("Hello, world! {} plus random is {}", num, add_one::add_random(num));
    println!("Hello, world! {} plus random is {}", num, add_two::add_two(num));
}
