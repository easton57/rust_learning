fn main() {
   let what = print_int(12); 

    println!("Here's your bool: {what}");
}

fn print_int(x: u32) -> bool {  // Declare the types for input and return
    println!("Here's your int: {x}");
    return false;
}
