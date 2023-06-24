fn main() {
    let name = name();
    let sur_name = sur_name();
    println!("My full name is {} {}", name, sur_name);
}


fn name() -> &'static str {
    let name = "Daniel";
    name
}

fn sur_name() -> &'static str {
let sur_name = "Efremov";
sur_name
}