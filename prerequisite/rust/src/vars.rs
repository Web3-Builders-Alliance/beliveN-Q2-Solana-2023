/* Variables hold primitive data or references to data
Variables are immutable by default
Rust is a block-scoped language
 */
pub fn run() {
    let name = "beliveN";
    let mut age = 28;
    println!("My name is {} and i am {}", name, age);
    age = 39;
    println!("My name is {} and i am {}", name, age);

    //Define constant

    const ID: i32 = 001;
    println!("ID:{}", ID);

    //Assign multiple vars

    let (my_name, my_age)=("Brad", 37);
    println!("{} is {}", my_name, my_age);

}
