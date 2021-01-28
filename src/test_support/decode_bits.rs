#![allow(dead_code)]

pub fn print_bits_f32(val: f32) {
    let mut s = String::new();
    let bits = val.to_bits();
    for b in 0..32 {
        if bits & (1 << (31 - b)) == 0 {
            s.push('0');
        } else {
            s.push('1');
        };
        if b == 0 || b == 8 {
            s.push(' ');
        }
    }
    println!("{}", s);
}

pub fn print_bits_f64(val: f64) {
    let mut s = String::new();
    let bits = val.to_bits();
    for b in 0..64 {
        if bits & (1 << (63 - b)) == 0 {
            s.push('0');
        } else {
            s.push('1');
        };
        if b == 0 || b == 11 {
            s.push(' ');
        }
    }
    println!("{}", s);
}

pub fn print_bits_u32(val: u32) {
    let mut s = String::new();
    for b in 0..32 {
        if val & (1 << (31 - b)) == 0 {
            s.push('0');
        } else {
            s.push('1');
        };
        if (b + 1) % 8 == 0 {
            s.push(' ');
        }
    }
    println!("{}", s);
}

pub fn print_bits_i32(val: i32) {
    let mut s = String::new();
    for b in 0..32 {
        if val & (1 << (31 - b)) == 0 {
            s.push('0');
        } else {
            s.push('1');
        };
        if (b + 1) % 8 == 0 {
            s.push(' ');
        }
    }
    println!("{}", s);
}
