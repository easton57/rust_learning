fn main() {
    println!("Hello, if statements!");

    let mut x = 16;

    if x < 32 {  // These can be run as a single line if simple enough. End with semi-colon
        println!("Too low loser");
    }
    else {
        println!("Nice");
    }

    loop { // Can assign to a variable! Wild!
        if x == 5 {
            break;
        }

        println!("Too high");

        x -= 1;
    }

    // While loops like python while {condition}
    // For loops like python as well, just use {} for i in variable { stuff }
    // for i in (1..4) { stuff }
    // for i in (1..4).rev() { stuff }
}
