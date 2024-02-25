fn main() {

    // // variables and mutability
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");
    //
    //
    // // constants
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // println!("{THREE_HOURS_IN_SECONDS}");

    // shadowing
    let x = 5;
    let x = x + 1;
    
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // let spaces = "    ";
    // spaces = spaces.len(); // não podemos alterar o tipo de uma variável
    // println!("The value of spaces is {spaces}");
}