use std::io;
fn main() {
   // Make a variable to store the temp
   // has to be mutable to collect input

   let mut temp = String::new();
   let mut temp_type = String::new();

   // Collects the type input
   println!("Are you going to enter C or F?");
   io::stdin()
       .read_line(&mut temp_type) // Reads in the line of input
       .expect("Failed to read line"); // Runs if Result enum is Err

   // Converts it from string to char
   let temp_type: char = temp_type
       .trim()
       .parse()
       .expect("Not a character");

   println!("Enter a temperature value");
   io::stdin()
       .read_line(&mut temp) 
       .expect("Failed to read line");

   // Converts it from string to f64 (signed float)
   let temp: f64 = temp
       .trim()
       .parse()
       .expect("Not a number");



   if temp_type == 'c' || temp_type == 'C' {
       println!("The temperature in Celcius is: {}", format!("{:.2}", temp));
       let temp: f64 = celc_to_faren(temp);
       println!("The temperature in Farenheit is: {}", format!("{:.2}", temp));
   } else if temp_type =='f' || temp_type == 'F'{
       println!("The temperature in Farenheit is: {}", format!("{:.2}", temp));
       let temp: f64 = faren_to_celc(temp);
       println!("The temperature in Celcius is: {}", format!("{:.2}", temp));
   } else {
       println!("You did not enter a valid temp_type!");
   }
}

// Farenheit = (9/5)C + 32
// C = (Farenheit - 32) * (5/9)

// By omitting the ';' on an expression
// that denotes to return that value
fn faren_to_celc(temp: f64) -> f64 {
    (temp-32.0)*(5.0/9.0)
}

fn celc_to_faren(temp: f64) -> f64 {
    (9.0/5.0)*temp + 32.0
}
