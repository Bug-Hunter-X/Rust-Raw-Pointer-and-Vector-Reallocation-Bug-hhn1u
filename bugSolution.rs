fn main() {
    let mut v = vec![1, 2, 3];
    // Avoid raw pointers. Instead, use safe methods
    v[0] = 10; 
    println!("v: {:?}", v);
}