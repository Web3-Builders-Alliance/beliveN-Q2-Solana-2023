/*
Primitive string = immutable fixed-lenght string somwhere in memory
String = Growable, heap allocated data structure - Use when you need to modify or own data
 */
pub fn run() {
    let mut hello = String::from("hello \u{1f600} ");

    // Get lenght

    println!("Lenght:{}", hello.len());

    // Push a char

    hello.push('W');

    // Push a String

    hello.push_str("orld!");

    // Capacity in bytes

    println!("Capacity: {}", hello.capacity());

    // Check if empty

    println!("Is empty:{}", hello.is_empty());

    // Check if contain substring

    println!("Contains 'World' {}", hello.contains("World"));

    // Replace
    println!("Replace: {}", hello.replace("World", "There"));

    // Loop through string by whitespace

    for word in hello.split_whitespace() {
        println!("{}", word)
    }

    // Create String with capacity

    let mut s = String::with_capacity(10);

    s.push('a');
    s.push('b');

    // Assertion testing

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    /* println!("{}", hello); */
}
