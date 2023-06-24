fn main(){

    let my_numbers = vec![10,20,30,40];

    for my_number in &my_numbers {
        match my_number {
            30 => println!("thirty"),
            _ => println!("{:?}", my_number)
        }
    }
    println!("The number of elements is {:?}", my_numbers.len())
}