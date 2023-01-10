use std::io;
use nucleoguide::GuideDesign;

fn main() {
    loop {
        println!("Input test seqeunce:");
        let mut test_case = String::new();
        match io::stdin().read_line(&mut test_case) {
            Ok(n) => (),
            Err(n) => println!("Error with input sequence.")
        }
        let test_result = GuideDesign::new(test_case);
        break;
    }
}
