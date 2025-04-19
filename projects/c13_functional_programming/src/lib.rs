
use std::thread;
use std::time::Duration;
use std::collections::HashMap;

struct Cacher<T, U, V> 
where 
    T: Fn(U) -> V,
    U: Eq + std::hash::Hash + Clone,
    V: Clone
{
    calculation: T,
    hashmap: HashMap<U, V>
}

impl<T, U, V> Cacher<T, U, V> 
where 
    T: Fn(U) -> V,
    U: Eq + std::hash::Hash + Clone,
    V: Clone
{
    fn new(calculation: T) -> Cacher<T, U, V> {
        Cacher { calculation, hashmap: HashMap::new() }
    }

    fn value(&mut self, arg: U) -> V {
        self.hashmap
            .entry(arg.clone())
            .or_insert_with(|| (self.calculation)(arg))
            .clone()
    }
}

pub fn generate_workout(intensity: i32, random_number: i32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            )
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Cacher;

    #[test]
    fn キャッシュした値1を取り出せる() {
        let mut cacher = Cacher::new(|x|x);
        let v1 = cacher.value(1);
        assert_eq!(v1, 1);
    }

    #[test]
    #[allow(unused_variables)]
    fn _2回目にキャッシュした値2を取り出せる() {
        let mut cacher = Cacher::new(|x|x);
        let v1 = cacher.value(1);
        let v2 = cacher.value(2);
        assert_eq!(v2, 2);
    }

    #[test]
    fn 文字列を渡して取り出せる() {
        let mut cacher = Cacher::new(|x|x);
        let v1 = cacher.value("Hello");
        assert_eq!(v1, "Hello");
    }
}