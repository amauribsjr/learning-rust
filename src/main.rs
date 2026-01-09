//para nada além de fins educacionais
fn main() {
    let drinks = ["Coca-Cola", "Guaraná", "Monster", "Juice"];
    println!("This is the result of X + Y, considering X as 12.8 and Y 89.0: {:?}", add(12.8, 89.0));
    println!("The second drink within drinks' array is: {:?}", drinks[1]);

    println!("");

    let mut x = change_x(2);
    println!("{x}");
    x = change_x(4);
    println!("And right after: {x}");
    let x = 89.2;
    println!("But now, X ({x}) is another variable.");

    println!("");

    let rand_name = "Amauri";
    let rand_name = rand_name.len();
    println!("rand_name values is: {rand_name}");

    println!("");

    const Y:i32 = 3;
    let z_example = 99;
    println!("Y and z_example values out of block: {Y}, {z_example}");
    {
        const Y:i32 = 5;
        let z_example = 19;
        println!("Y and z_example values inside the block: {Y}, {z_example}");
    }
    println!("Now, right out of the block, we have last Y values: {Y}\n... and also z_example: {z_example}");
}

fn add(x: f64, y: f64) -> f64 {
    x + y
}

fn change_x(x: i32) -> String {
    let y = x + 3;
    return format!("X was {x}, now it's {y}.");
}
