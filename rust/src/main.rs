fn main() {
    println!("Hello, world!");
    let a = add(30);
    println!("{}",a);
}

fn add(x: i32) -> i32{
    return x+1
}