pub fn run() {
    //print to console
    println!("Hello from the print.rs file");

    //basic formatting
    /* println!("Number: {}", 1); */
    println!("{} is from {}", "beliven", "Portugal");

    //positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2    }",
        "beliven", "Portugal", "code"
    );

    //Named Arguments
    println!(
        "{name} likes to play {activity}",
        name = "beliveN",
        activity = "CS"
    );

    //Placeholder traits

    println!("Binary: {:b} Hex: {:x} Octal : {:o}", 10 , 10 , 10);


    //Placeholder for debug traits

    println!("{:?}", (12, true, "Hello"));


    //Basic Math
    println!("10+10={}", 10+10);

}
