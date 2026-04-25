use jsgc_derive::Trace;

#[derive(Trace)]
struct A {
    t_unit: (),
    t_bool: bool,
    t_char: char,
    t_f32: f32,
    t_f64: f64,
    t_i8: i8,
    t_i16: i16,
    t_i32: i32,
    t_i64: i64,
    t_i128: i128,
    t_isize: isize,
    t_u8: u8,
    t_u16: u16,
    t_u32: u32,
    t_u64: u64,
    t_u128: u128,
    t_usize: usize,
}

fn main() {}
