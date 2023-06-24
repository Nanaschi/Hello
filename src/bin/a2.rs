fn main() {
    show_sum(sum(2,9));
}

fn sum(a: i32, b: i32) -> i32{
a+b
}

fn show_sum(result: i32){
    println!("{:?}", result)
    }
