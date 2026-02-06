use std::str::Chars;

mod strings;
mod arrays;

fn main() {
    println!("From data types cargo");

    basics_stuff();

    strings::strings();

    let arr =arrays::arrays();

    let arr2 : [&str;2] = ["abcd","efgh"];

    arrays::print_array(arr);
    println!("printed array:{:?} of length:{}",arr2,arr2.len());

    let _s : &str = "hello world";

    let mut name = String::from("Chishti");
    name.push_str("Shaik");

    println!("{name}");

    let function_returning_str = strings();
    println!("Returned from function: {:?}", function_returning_str);

    let str1  = "Sambar";
    let str2 = " Vada";

    let append = append(&str1,&str2);
    println!("{append}");

    let f_name = &mut String::from("Sixty");
    add_string(f_name,"boy");
    println!("{f_name}");

    let t = ([1,2,3],[4,5,6]);

    // Modify this line only, don't use `_s`
    for i in [t.0,t.1]{
        for j in 0..3{
            print!("{} ",i[j])
        }
        println!();
    }

    let (ref s1, ref s2) = t;

    println!("{:?} {:?} {:?}",s1,s2,t);
    print!("{}\n",s1[0]);

    let string = String::from("Chutney");

    for i in string.chars(){
        print!("{}",i);
    }

}

fn basics_stuff() {
    let int = 1;

    let decimal = 23f64;

    let name: &str = "Chishti";

    let string: &str = &int.to_string();

    for i in 0..2 {
        if i == 0 {
            print!("Numero: {}, ", i + 1);
        } else {
            print!("{}, ", i + 1);
        }
    }
    println!();

    println!("Name: {}", name);

    println!("Integer division {:.3}", 232f32 / 32f32);

    println!("Float division: {} ", 1f32 / 2f32);

    println!("Type casted into decimal: {}", decimal);

    println!("{}", string);
}

//chars is borrowed, so it's lifetime ends at the end of function
//To remove this, we can add ownership by returning a vector by collect() method.
fn strings() -> Vec<char>{
    let mut name = String::from("Chishti");
    name.push_str(" Shaik");
    let chars:Chars = name.chars();

    let ref_name : &str = &name;

    println!("{:?}",chars);
    println!("{}", ref_name);

    chars.collect()
}

fn append(x:&str, y:&str) -> String{
    let mut s = String::from("");
    s.push_str(x);
    s.push_str(y);
    s
}

fn add_string(x:&mut String,y:&str){
    x.push_str(y);
}

