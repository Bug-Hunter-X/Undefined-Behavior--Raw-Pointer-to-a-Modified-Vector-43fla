fn main() {
    let mut v = vec![1, 2, 3];
    let ptr = v.as_mut_ptr();
    unsafe {
        *ptr = 4; // Modify the first element
    }
    println!("{:?}", v); // v is now [4, 2, 3]

    // The following line causes undefined behavior:
    // because the memory is freed but still in use by ptr
    v = vec![4,5,6];
    println!("{:?}", v); // v is now [4, 5, 6]
}