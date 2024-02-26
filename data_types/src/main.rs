use std::io;

fn main() {
    // integer

    // deixando explicito o tipo pro rust ser capaz de intender q precisa ser um inteiro, nesse
    // caso ocorre uma conversão, mas se caso não for numero ele respondera com error
    // case error: let n1: u32 = "a".parse().expect("Not a number!");
    // let n1: u32 = "423".parse().expect("Not a number!");

    // deixando o proprio rust entender o tipo informado
    // tipo padrão para a deteccao do rust é i32
    // let n2 = 2147483647;
    //
    // let n3: i8 = 1;

    // rust pede para sinalizar um underscore, pra um variavel q tu tem interesse de declarar sem usar
    // let _n4: i16;
    //
    // println!("n1: {n1}\nn2: {n2}\nn3: {n3}");

    // Nao funciona exceed o valor maximo do tipo overflow
    // let vl_over_flow: i8 = 128;
    // println!("{vl_over_flow}")

    // float
    // let x = 2.0; // f64
    // let y: f32 = 3.0; // f32

    // addition
    // let sum = 5 + 10;
    // println!("{sum}");

    // // subtraction
    // let difference = 95.5 - 4.3;
    // println!("subtraction: {difference}");
    //
    // // multiplication
    // let product = 4 * 30;
    // println!("multiplication: {product}");
    //
    // // division
    // let quotient = 56.7 / 32.2;
    // println!("division 01: {quotient}");
    // let truncated = -5 / 3; // Results in -1
    // println!("division 02: {truncated}");
    //
    // // remainder
    // let remainder = 43 % 5;
    // println!("remainder: {remainder}");

    // boolean
    // let t = true;
    // let f: bool = false; // with explicit type annotation
    // println!("t: {t}, f: {f}");

    // let c = 'z';
    // let z: char = 'ℤ'; // with explicit type annotation
    // let heart_eyed_cat = '😻';
    // println!("c: {c}, z: {z}, emoji: {heart_eyed_cat}");

    // tupla
    // let _tup: (i32, f64, u8) = (500, 200.32, 1);

    // let tup = (500, 6.4, 1);
    // let (_x, y, _z) = tup;
    // println!("The value of y is: {y}");

    // let x: (i32, f64, u8) = (500, 6.4, 1);
    // let five_hundred = x.0;
    // let six_point_four = x.1;
    // let one = x.2;

    // println!("0: {five_hundred}\n1: {six_point_four}\n2: {one}")

    // array
    // let a = [1, 2, 3, 4, 5];

    // let months = ["January", "February", "March", "April", "May", "June", "July",
    //           "August", "September", "October", "November", "December"];

    // let a: [i32; 5] = [1, 2, 3, 4, 5];

    // uma declaracao curta disso let a = [3; 5]; para isso let a = [3, 3, 3, 3, 3];

    // let a = [1, 2, 3, 4, 5];
    //
    // let first = a[0];
    // let second = a[1];
    // println!("{first}");
    // println!("{second}");

    // Acesso inválido ao elemento da matriz
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
