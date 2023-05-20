/* Conditionals are used to check the condition of something and act on the result */

pub fn run() {
    let age: u8 = 32;
    let check_id: bool = true;
    let knows_person_of_age = true;
  
    // If/Else
    if age >= 21 && check_id || knows_person_of_age {
      println!("Zoo Man: What animal would you like to see?");
    } else if age < 21 && check_id {
      println!("Zoo Man: Sorry, lions to dangerous for your age");
    } else {
      println!("Zoo Man: I'll need to see your ID");
    }
  
    // Shorthand If
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is Of Age: {}", is_of_age)
  }