use nucleoguide::GuideDesign;

fn main() {
    println!("testing lib!");
    let test_struct = match GuideDesign::new(String::from("AGCTAGCTAGCT"), String::from("TAG"), 0) {
        Ok(n) => n,
        Err(n) => panic!("{n}")
    };
    match GuideDesign::idgrnas(&test_struct) {
        Ok(n) => println!("{n}"),
        Err(n) => panic!("{n}")
    }
}
