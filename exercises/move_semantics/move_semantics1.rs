// move_semantics1.rs
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand for a hint.

fn main() {
    let vec0: Vec<i32> = Vec::new();

    println!("pointer1 {:p}", &vec0);

    let mut vec1 = fill_vec(vec0);
    println!("pointer4 {:p}", &vec1);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    println!("pointer2 {:p}", &vec);
    let mut vec2 = vec;
    println!("pointer3 {:p}", &vec2);
    vec2.push(22);
    vec2.push(44);
    vec2.push(66);

    vec2
}
