use c13_functional_programming as MyModule;
fn main() {
    if false {
        let simulated_user_specified_value = 10;
        let simulated_random_number = 7;

        MyModule::generate_workout(
            simulated_user_specified_value,
            simulated_random_number
        );
    }

    {
        // Closures capture their environment
        let x = vec![1, 2, 3];
        let equal_to_x = move |z| z == x;

        let y  = vec![1, 2, 3];
        assert!(equal_to_x(y));
    }

    {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        for val in v1_iter {
            println!("{}, ", val);
        }
    }
}