fn main() {
    // Creating raw pointers from references
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // Creating a raw pointer to an arbitrary memory address
    let address = 0x012345usize;
    let r = address as *const i32;

    // Would work but give a Segmentation fault
    /*
    unsafe {
        println!("r is: {}", *r);
    }
    */
}
