use std::str::Chars;
use std::collections::HashMap;
use std::time::Instant;
//fragment specifiers:
/*
| Specifier  | What it Matches                     | Example                     |
| ---------- | ----------------------------------- | --------------------------- |
| `ident`    | Identifier (variable/function name) | `x`, `sum`, `my_var`        |
| `expr`     | Expression                          | `a + b`, `5`, `foo()`       |
| `stmt`     | Statement                           | `let x = 5;`                |
| `block`    | Block of code                       | `{ x + 1 }`                 |
| `pat`      | Pattern (used in match)             | `Some(x)`, `x`, `_`         |
| `ty`       | Type                                | `i32`, `String`, `Vec<i32>` |
| `path`     | Path                                | `std::io::Read`             |
| `meta`     | Attribute metadata                  | `derive(Debug)`             |
| `tt`       | Token tree (any tokens)             | `+`, `=>`, `{}`             |
| `item`     | Item                                | `fn`, `struct`, `enum`      |
| `lifetime` | Lifetime                            | `'a`, `'static`             |
| `vis`      | Visibility                          | `pub`, `pub(crate)`         |
| `literal`  | Literal values                      | `"hi"`, `42`, `true`        |
 */

mod strings;
mod arrays;

macro_rules! new {
    ($a:expr, $b:expr)/*signature*/ => {
        //Code that will be executed during run time
        println!("Hey, from new macro{}",($a+$b))
    };
}

//creating a variable
macro_rules! create_var {
    ($var_name:ident) => {
        let $var_name = 42;
    };
}

//Macro with 1 or more argument like method overloading
macro_rules! overload_macro {
    ($name:expr) => {println!("{}",$name)};

    ($name:expr,$message:expr) => {println!("{},{}",$message,$name)};
}

//creating a hashmap and initialize it.{import hashmaps first form collection}
//Using repetition
//They are hygienic => meaning they do not conflict with
// variable names form the surroundings
/*
Example :
Even if user writes:

let h_map = 100;
let map = create_hashmap! { "a" => 1 };

There is no collision, because macro variables are hygienic.
 */
macro_rules! create_hashmap {
    ($($key:expr => $value:expr),* $(,)?) => {
        {
            let mut h_map = HashMap::new();
            $(h_map.insert($key,$value);)* h_map
        }
    };
}
// $(..),* => for repeating things.
// $(,)? => allows one optional comma (",")
/*
//It makes
a => b,
c => d,

a valid syntax
 */
macro_rules! init_hmap {
    ($hamp:expr, $($key:expr => $val:expr),* $(,)*) => {{
        $(
            $hamp.insert($key, $val);
        )*
        $hamp
    }};
}

//macro to calculate execution time:
macro_rules! benchmark {
    ($name:expr, $code_block:block) => {
        {
            let start = Instant::now();
            let result = $code_block;
            let duration = start.elapsed();
            println!("{} took {:?}", $name, duration);
            result
        }
    };
}

//initializing using macros
macro_rules! init_vec {
    ( keyword $vec:expr,$($val:expr),*) => {
        $(
        $vec.push($val);
        )*
    };

    ($($val:expr),* $(,)*) => {
        {
            let mut vec = Vec::new();
            $(vec.push($val);)*
            vec
        }
    };
}

//Using custom keywords:
macro_rules! vec_init_overload {
    ($($val:expr),* $(,)*) => {
        {
            let mut vec =Vec::new();
            $(vec.push($val);)*
            vec
        }
    };
    (keyword $vec:expr, $($val:expr),*) => {
        $(
          $vec.push($val);
        )*
    };
}

macro_rules! init_vec2 {
    ($($val:expr),* $(,)*) => {
        {
            let mut vec = Vec::new();
            $(vec.push($val);)*
            vec
        }
    };
}


//Structs
struct Person{
    first_name: String,
    last_name: String,
}

impl Person {
    fn print_details(&self) {
        println!("Hello!! {}, {}", self.last_name, self.first_name)
    }
}

enum Store{
    ExitTime,
    Manager(Person),
}



fn main() {
    let st = Instant::now();
    //basics_two();
    //rusty_problems();
    //macros();

    //Structs:
    let guy = Person{first_name: String::from("Chishti"),last_name:String::from("Shaik")};
    guy.print_details();
    let et = st.elapsed();

    //Enums
    let guy = Store::Manager(Person{first_name:String::from("Chishti"),last_name:String::from("Shaik")});
    let time = Store::ExitTime;

    process_store(guy);
    process_store(time);



    println!("Elapsed time {:?}\u{03BC}s",et.as_micros());
}

fn process_store(p:Store){
    match p {
        Store::ExitTime => {println!("Exiting");},
        Store::Manager(person) => { println!("{}",person.last_name);}
    }
}

pub fn macros() {
    //Macros
    create_var!(new_string);
    println!("This is my new_string {}", new_string);
    overload_macro!("Chishti");
    overload_macro!("chishti","my message");

    let map = create_hashmap! {
        "k" => 1,
        "k2" => 2
    };

    benchmark!("calculate_et",{
        new!(1,25);
    });
    let mut vec = Vec::new();
    init_vec!(keyword vec,12,3,48,1,4,67);
    println!("vec from keyword: {:?}", vec);

    vec.sort();
    println!("{:?}", vec);

    let vec2 = init_vec2!(1,2,3,4);
    println!("{:?}", vec2);

    let vec3 = init_vec!(1,2,3,4);
    println!("vec3: {:?}", vec3);


    let vector2 = vec_init_overload!(1,2,3,4,5);
    println!("vector2: {:?}", vector2);

    let mut vector3 = Vec::new();
    vec_init_overload!(keyword vector3,241,241424);
    println!("vector3: {:?}", vector3);


    let mut hmap = HashMap::new();
    init_hmap!(hmap, "id" => 1, "score" => 100);

    println!("This is my map {:?}", map);
}

pub fn basics_two() {
    println!("From data types cargo");

    basics_stuff();

    strings::strings();

    let arr = arrays::arrays();

    let arr2: [&str; 2] = ["abcd", "efgh"];

    arrays::print_array(arr);
    println!("printed array:{:?} of length:{}", arr2, arr2.len());

    let _s: &str = "hello world";

    let mut name = String::from("Chishti");
    name.push_str("Shaik");

    println!("{name}");

    let function_returning_str = strings();
    println!("Returned from function: {:?}", function_returning_str);

    let str1 = "Sambar";
    let str2 = " Vada";

    let append = append(&str1, &str2);
    println!("{append}");

    let f_name = &mut String::from("Sixty");
    add_string(f_name, "boy");
    println!("{f_name}");
}

pub fn rusty_problems() {
    let t = ([1, 2, 3], [4, 5, 6]);

    // Modify this line only, don't use `_s`
    for i in [t.0, t.1] {
        for j in 0..3 {
            print!("{} ", i[j])
        }
        println!();
    }

    let (ref s1, ref s2) = t;

    println!("{:?} {:?} {:?}", s1, s2, t);
    print!("{}\n", s1[0]);

    let string = String::from("Chutney");

    for i in string.chars() {
        print!("{}", i);
    }

    let arr = [1, 2, 3];
    let s1: &[i32] = &arr[0..2];

    let s2: &str = "hello, world";

    println!("\nSuccess! {:?}{:?}", s1, s2);

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // Fill the blanks to make the code work
    //let slice:__ = __;
    let slice: &[i32] = &arr[1..4];
    assert_eq!(slice, &[2, 3, 4]);
    println!("{:?}", slice);

    println!("Success!");


    /*

fn main() {
    let s = "你好，世界";
    // Modify this line to make the code work
    let slice = &s[0..2];

    assert!(slice == "你");

    println!("Success!");
}
     */

    let s = "你好，世界";

    // "你" is 3 bytes long in UTF-8
    let slice = &s[0..3];

    assert!(slice == "你");

    println!("Success!");

    let sr = String::from("A Normal String");

    get_ref(&sr);
    println!("Success!");
    let var2 = no_ref(sr);

    println!("{}", var2);
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

fn get_ref(var:&String){
    println!("{}",var);
}

fn no_ref(var:String)->String{
    println!("{}",var);
    var
}