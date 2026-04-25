use jsgc_derive::Trace;

#[derive(Trace)]
struct A<'a> {
    t_unit: &'a (),
    t_bool: &'a bool,
    t_char: &'a char,
    t_f32: &'a f32,
    t_f64: &'a f64,
    t_i8: &'a i8,
    t_i16: &'a i16,
    t_i32: &'a i32,
    t_i64: &'a i64,
    t_i128: &'a i128,
    t_isize: &'a isize,
    t_str: &'a str,
    t_u8: &'a u8,
    t_u16: &'a u16,
    t_u32: &'a u32,
    t_u64: &'a u64,
    t_u128: &'a u128,
    t_usize: &'a usize,
}

fn main() {}
