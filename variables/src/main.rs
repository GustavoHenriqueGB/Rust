/*
Integer types:

Length  | Signed | Unsigned
--------+--------+---------
8-bit	| i8	 | u8
16-bit	| i16	 | u16
32-bit	| i32	 | u32
64-bit	| i64	 | u64
128-bit	| i128	 | u128
arch	| isize  | usize

Iteger Literals

Number literals |Example
----------------+------------
Decimal	        |98_222
Hex	            |0xff
Octal	        |0o77
Binary	        |0b1111_0000
Byte (u8 only)	|b'A'
*/

fn main() {
    // Integers:
    let _i: u32 = 12; // u32 the _ is so that warnings don't occur
    let _j: i32 = -12; // i32 

    // Floating points:
    let _x = 2.0; // f64 
    let _y: f32 = 3.0; // f32 

    // Numeric Operations:

    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; // Results in -1
    
    // Bool:
    let _t = true;
    let _f: bool = false; // with explicit type annotation

    // Char: 
    let _c = 'z';
    let _z: char = 'ℤ'; // with explicit type annotation
    let _heart_eyed_cat = '😻';
}