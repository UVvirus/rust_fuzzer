use std::process::Command;
use std::io::Error;
use input_generator::generate_random_number;
use input_generator::generate_random_word;

fn main() -> Result<(), Error>{
let random_number=generate_random_number(1,100);
println!("Random number: {}", random_number);

let random_word = generate_random_word(10);

let python_script = "/home/you_we/test.py";
println!("Running command: python3 {}", python_script);
let output = Command::new("python").arg(python_script)
.arg(random_word)
.output()?;
println!("Output: {}", String::from_utf8_lossy(&output.stdout));
eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));

Ok(())
}