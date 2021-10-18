use std::io;

fn main() {
    // the type here is specified so will run
    let guess: u32 = "42".parse().expect("Not a number!");
    
    // INTEGER TYPES:
    // i - signed
    // u - unsigned
    // (number) - size in bits: 8, 16, 32, 64, 128
    //     depends on computer: for 64 bit machine use 64 etc
    
    //  INTEGER LITERALS:
    // Decimal - 98_222
    // Hex - 0xff
    // Octal - 0o77
    // Bin - 0b1111_0000
    // Byte (for u8 only) - b'A'

    // the type here is NOT specified so will NOT run
    // compiler needs more information
    // let guess = "42".parse().expect("Not a number!");
    println!("{}", guess);

    //floating point types:
    let _x = 2.0; //defaults to f64
    let _y: f32 = 3.0;

    //numeric operations
    //add
    let _sum = 5 + 10;

    //sub
    let _diff = 95.5 - 4.3;

    //mult
    let _prod = 4 * 30;

    //quotient
    let _qot = 56.7 / 32.2;

    //remainder
    let _rem = 43 % 5;

    // BOOLEANS
    let _t = true;
    let f: bool = false; //explicit type annotation
    println!("{}", f);

    // CHAR
    let _c = 'z';
    let _z = 'Z';

    //// COMPOUND TYPES:
    // TUPLE
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // println!("{}", tup) won't work so:
    let (_x, _y, z) = tup;
    println!("{}", z);

    //OR:
    println!("{}", tup.0);

    // ARRAYS
    let arr = [1, 2, 3, 4, 5];
    let mos = ["Jan", "Feb"];
    let arr2: [i32; 5] = [1, 2, 3, 4, 5];
    let arr3 = [3; 5];

    println!("{}", arr[0]);
    println!("{}", mos[0]);
    println!("{}", arr2[0]);
    println!("{}", arr3[4]);

    // try invalid access:
    println!("Please enter array index.");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    
    let index: usize = index //will default to my system size
        .trim()
        .parse()
        .expect("Index entered not a number");
    
    let elem = arr[index];

    println!(
        "The value of the elem at index {} is: {}",
        index, elem
    );


}
