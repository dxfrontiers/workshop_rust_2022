fn main() {

    let mut fib = vec![0, 1, 1, 2, 3, 5];

    print_last_fib(&x);

    fib.push(next_fib_iter(&fib));

    print_last_fib(&fib);

    fib.push(next_fib_pattern(&fib));

    print_last_fib(&fib);

}

fn next_fib_iter(fib: &[i32]) -> i32 {
    todo!()
}

fn next_fib_pattern(fib: &[i32]) -> i32 {
    todo!()
}

fn print_last_fib(fib: &[i32]) {
    if let Some(x) = fib.last() {
        println!("newested last: {}", x)
    }
}