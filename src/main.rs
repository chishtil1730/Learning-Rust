
mod class2;
struct Point{
    x:i32,
    y:i32
}

fn main() {
    let mut x =5;
    x +=1;
    let  y = x;
    println!("Hello => {} ",x);
    coordinates(Point{x,y});

    for i in 0..5{
        print!("{} ",i+1);
    }

    println!("");
    class2::get_data();

    let y = class2::print_data(1.2,3);
    println!("Float sum: {}",y);

}

fn coordinates(Point{x,y}: Point){
    println!("x ={}, y ={}",x,y)
}

