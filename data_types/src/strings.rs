pub fn strings() {
    let name: &str = "Chishti";
    let character : char = name.chars().nth(0).unwrap();

    for letter in name.chars(){
        print!("{ }",letter)
    }
    println!();

    let number : &str ="1234";

    println!("Initial: {}",character);

    let x: i32  = number.parse().unwrap();
    println!("{}",x);

}
