mod strings;
mod arrays;

fn main() {
    println!("From data types cargo");

    let int =1;

    let decimal = 23f64;

    let name: &str = "Chishti";

    let string: &str = &int.to_string();

    for i in 0..2{
        if i==0{
            print!("Numero: {}, ",i+1);
        }else {
            print!("{}, ",i+1);
        }
    }
    println!();

    println!("Name: {}",name);

    println!("Integer division {:.3}", 232f32/32f32);

    println!("Float division: {} ", 1f32/2f32);

    println!("Type casted into decimal: {}",decimal);

    println!("{}", string);

    strings::strings();

    let arr =arrays::arrays();

    let arr2 : [&str;2] = ["abcd","efgh"];

    arrays::print_array(arr);
    println!("printed array:{:?} of length:{}",arr2,arr2.len());
}
