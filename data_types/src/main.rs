fn main() {
    println!("Basically, rust can infer the type or you can declare it. Look below and explicitly declared types!");

    // Int's, this goes up to u128, base 2 numbers obviously, 8, 16, 32, 64, 128. signed int's use
    // an i as a prefix instead of a u
    let num: u8 = 5;

    println!("\nHere's you u8 number: {num}\nHere are some other number literals:\nDecimal: 98_111\nHex: 0xff\nOctal: 0o77\nBinary: 0b1111_0000\nByte (u8 only): b'A'");

    // Other basic types exist, standard math operations, floats, etc. Here's a tuple instead since
    // you typically forget about how to do those.
    let tup: (i64, f32, u8) = (500, 3.2, 1);
    let (x, y, z) = tup;

    println!("{z}");

    // Arrays are just like python
    let arr = [1, 2, 3, 4, 5];
    let item = arr[1]; // This seems to be necessary. Assign out of the array or tuple for use
    println!("{item}");
}
