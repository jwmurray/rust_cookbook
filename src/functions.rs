fn greeting(salutation: &str, name: &str) {
    println!("{}, {}", salutation, name);
}

fn add(addend1: i32, addend2: i32) -> i32 {
    addend1 + addend2
}

pub fn run() {
    greeting("Hello", "Jane");

    // Bind function values to variables
    let get_sum = add(5, 5);
    println!("Sum: {}", get_sum);

    // Closure
    let n3: i32 = 11;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;

    println!("C Sum: {}", add_nums(3, 3));
}
