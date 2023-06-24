fn main(){

let amazon_box = Box{
width: 1,
length: 2,
heigth: 3,
weight: 5,
color: Colors::Blue
};

Box::big_box().show_characteristics();
}

struct Box{
    width: i32,
    length: i32,
    heigth: i32,
    weight: i32,
    color: Colors
}

impl Box{

    fn big_box() -> Self{
Self{ width: 100,
    length: 200,
    heigth: 300,
    weight: 500,
    color: Colors::White}
    }

    fn show_characteristics(&self){
        println!("The characteristics are {:?} {:?} {:?} {:?} {:?}.",
        self.width, self.length, self.heigth, self.weight, self.color)
    }
}
#[derive(Debug)]
enum Colors{
    Green,
    Blue,
    Red,
    White
}