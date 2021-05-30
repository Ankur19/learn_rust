fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 7;
    println!("The value of x is: {}", x);

    let tup: (i32, u32, f64) = (1, 2, 1.99);
    let arr: [i32; 5] = [0, 1, 2, 3, 4];
    let arr1: [u32; 10] = [5; 10];
}
