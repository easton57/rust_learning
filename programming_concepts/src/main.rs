fn main() {
    variables_and_mutability();

    constants();

    shadowing();
}

fn variables_and_mutability() {
    let mut x = 5;
    println!("Here is x: {x}");
    x = 7;
    println!("Here is x now: {x}");
}

fn constants() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("Here is my const value: {THREE_HOURS_IN_SECONDS}");
}

fn shadowing() {
    let x = 5;

    println!("Here is my x outside of the lower scope: {x}");

    {
        let x = 16;
        println!("Here is x in the lower scope/shadow: {x}");
    }

    println!("Here we are back outside of the scope: {x}");
}
