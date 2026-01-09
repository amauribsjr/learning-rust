fn main() {
    let drinks = ["Coca-Cola", "Guaraná", "Monster", "Juice"];
    println!("This is the result of X + Y, considering X as 12.8 and Y 89.0: {:?}", add(12.8, 89.0));
    println!("The second drink within drinks' array is: {:?}", drinks[1]);
}

fn add(x: f64, y: f64) -> f64 {
    x + y
}
