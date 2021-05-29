#[no_mangle]
extern fn add(x: i32, y: i32) -> i32 {
    return x.wrapping_add(y);
}

#[no_mangle]
extern fn sub(x: i32, y: i32) -> i32 {
    return x.wrapping_sub(y);
}

#[no_mangle]
extern fn mul(x: i32, y: i32) -> i32 {
    return x.wrapping_mul(y);
}

#[no_mangle]
extern fn div(x: i32, y: i32) -> i32 {
    return match y==0 {
        true => 0,
        false => (x/y)
    }
}
