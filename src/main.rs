#![allow(dead_code)]
use std::io;
//mod currency_converter;
mod triangle;

fn input_int()->i32{
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Error");
    
    let n :i32 = s.trim().parse().expect("Ivalid Input");
    n
    
}

fn main() {
    let n = input_int();

    // Factorial
    //println!("Factorial: {}",factorial(n));

    // Currency converter
    // println!("Dolar to pen: {}",currency_converter::converter_usd_pen(n));
    // println!("Dolar to pen: {}",currency_converter::converter_pen_usd(n));

    // triangle::triangle(n);
    // triangle::triangle_inv(n);
    triangle::triangle_inv_odd(n);
    
}

