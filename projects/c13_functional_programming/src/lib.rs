
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

#[cfg(test)]
mod iterator_test {
    #[test]
    fn イテレータのテスト(){
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn イテレータを使って合計値を算出する() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let total: i32 = v1_iter.sum();
        assert_eq!(total, 6);
    }

    #[test]
    fn mapとcollectを使って配列に処理を行う() {
        let v1 = vec![1, 2, 3];
        let result: Vec<i32> = v1.iter().map(|x| x + 1).collect();
        assert_eq!(result, vec![2, 3, 4], "{:?}", result);
    }

    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String
    }

    fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|x|shoe_size == x.size).collect()
    }

    #[test]
    fn 靴の配列から自分のサイズのものだけ取り出す() {
        let shoes = vec![
            Shoe { size: 10, style: String::from("sneaker")},
            Shoe { size: 13, style: String::from("sandal")},
            Shoe { size: 10, style: String::from("boot")},
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);

        assert_eq!(in_my_size, [
            Shoe { size: 10, style: String::from("sneaker")},
            Shoe { size: 10, style: String::from("boot")},
        ]);
    }

    #[derive(Debug)]
    struct Counter { count: i32
    }

    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }
    
    impl Iterator for Counter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;
            if self.count < 6 {
                Some(self.count)
            } else {
                None
            }
        }
    }
    

    #[test]
    fn 独自のイテレータで5まで数える() {
        let mut counter = Counter::new();
        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn 別のイテレータを使う() {
        let sum: i32 = Counter::new().zip(Counter::new().skip(1)).map(|(x,y)|x * y).filter(|x|x % 3 == 0).sum();
        assert_eq!(18, sum);
    }
}

