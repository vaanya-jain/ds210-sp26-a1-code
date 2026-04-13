fn ptr() {
    let x = String::from("hello");
    let ptr = &x as *const String;
    drop(x);

    let x1: u64 = 1;
    let x2: u64 = 3;
    let x3: u64 = x1 + x2;

    unsafe {
        println!("{}", *ptr);
    }
}


fn main() {
    ptr();
}









/*
fn reference() {
    let x = String::from("hello");
    let ref_to_x = &x;
    drop(x);
    println!("{}", ref_to_x);
}
*/