fn main() {
    let mut v = vec![1, 2, 3];
    {   //Use a scope to ensure proper resource management
        let ptr = v.as_mut_ptr();
        unsafe {
            *ptr = 4; // Modify the first element
        }
        println!("{:?}", v); // v is now [4, 2, 3]
    }

    //The vector is dropped at the end of the scope and is safe to re-use
    v = vec![4,5,6];
    println!("{:?}", v); // v is now [4, 5, 6]
} 