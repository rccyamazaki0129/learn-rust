use rand::Rng;

pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub fn add_random(x: i32) -> i32 {
    let mut rng = rand::thread_rng();
    let random_number: i32 = rng.gen_range(1,100);
    x + random_number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_one() {
        assert_eq!(add_one(1), 2);
        assert_eq!(add_one(0), 1);
        assert_eq!(add_one(-1), 0);
    }

    #[test]
    fn test_add_random() {
        let result = add_random(1);
        assert!(result > 1 && result < 101);
    }
}