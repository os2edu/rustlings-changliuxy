// move_semantics1.rs
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand for a hint.


fn main() {
    let mut vec0 = Vec::new();
    let v = vec0.clone();
    let mut vec1 = fill_vec(&mut vec0);

    println!("{} has length {} content `{:?}`", "vec1", v.len(), v);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec1: &mut Vec<i32>) -> Vec<i32> {
    let mut vec = vec1;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec.to_vec()
}
