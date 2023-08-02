fn main() {
    println!("Shadowing!");
    let x = 5;
    let x = x + 1;
    {   
        let x = x * 5;
        println!("The value of x in scope: {x}");
    } 
    println!("The value of x outside scope: {x}");
}
