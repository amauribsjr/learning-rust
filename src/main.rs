//para nada além de fins educacionais
fn main() {
    println!(" = Array and functions return = \n");
    let drinks = ["Coca-Cola", "Guaraná", "Monster", "Juice"];
    println!("This is the result of X + Y, considering X as 12.8 and Y 89: {:?}", add(12.8, 89f64));
    println!("The second drink within drinks' array is: {:?}", drinks[1]);

    let rep_three = [3; 5];
    let three_five = [3, 5];
    println!("Number three 5 times: {:?}", rep_three);
    println!("Now there's only {:?} but nothing is getting repeating.", three_five);

    println!("\n = Mutable vars and shadowing = \n");

    let mut x = change_x(2);
    println!("{x}");
    x = change_x(4);
    println!("And right after: {x}");
    let x = 89.2;
    println!("But now, X ({x}) is another variable.");
    let rand_name = "Amauri";
    let rand_name = rand_name.len();
    println!("rand_name values is: {rand_name}");

    println!("\n = Blocks manipulation = \n");

    const Y:i32 = 3;
    let z_example = 99;
    println!("Y and z_example values out of block: {Y}, {z_example}");
    {
        const Y:i32 = 5;
        let z_example = 19;
        println!("Y and z_example values inside the block: {Y}, {z_example}");
    }
    println!("Now, right out of the block, we have last Y values: {Y}\n... and also z_example: {z_example}");

    println!("\n= Tuples =\n");

    let tup1: (i32, f64, bool) = (12, 83.4, false);
    println!("tup1 values are: {:?}", tup1);

    let (x1, y1, z1) = tup1;
    println!("tup1 desconstructed values: {x1}, {y1}, {z1}");
    println!("Accessing the same values by index: {:?}, {:?}, {:?}", tup1.0, tup1.1, tup1.2);
}

//functions examples
fn add(x: f64, y: f64) -> f64 {
    x + y
}

fn change_x(x: i32) -> String {
    let y = x + 3;
    return format!("X was {x}, now it's {y}.");
}
