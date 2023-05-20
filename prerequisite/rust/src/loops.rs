// Loops are used to iterate until a condition is met

pub fn run() {
    let mut count = 0;

    // Infinity Loop

    /*     loop
       {
           count +=1;
           println!("Number:{}", count);

           if count == 10 {
               break;
           }
       }
    */

    // While loop (fizzbuzz/challenge loop through 0 to 100 if the number divisble by 3 u print fizz if its divisible by buzz, and if both print fizzbuzz )

/*    while count <= 100 {
    if count % 15 == 0 {
     println!("fizzbuzz");
   } else if count % 3 == 0 {
      println!("fizz");
    } else if count % 5 == 0 {
    println!("buzz")
   } else {
      println!("{}", count);
    }

 // Inc
  count += 1;


} */

  // For Range
  for x in 0..100 {
    if x % 15 == 0 {
      println!("fizzbuzz");
    } else if x % 3 == 0 {
      println!("fizz");
    } else if x % 5 == 0 {
      println!("buzz")
    } else {
      println!("{}", x);
    }
  }
}