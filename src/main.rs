use std::f32::consts::E;

fn main() {
    //Integer
    let entero: i8 = 23;
    let entero2: u8 = 40;
    let entero3: i8 = -40;


    //Integer literals

    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;

    //Floating point

    let float1 = 5.0;
    let float32: f32 = 12.432;

    //Boolean

    let verdadero = true;
    let falso = false;

    //Character

    let caracter = 'a';
    let emoji = '😻';

    // Compound types

    //Tuple

    let tupla = ('h', 23, -45, 0.222);
    let tupla2: (char, u64, i32, f64) = ('h', 23, -45, 0.222);
    let (x,y,z,w) = tupla;
    println!("El segundo valor de la tupla es: {}", tupla.1);


    //Array

    let arreglo = [1,2,3,4,5];
    println!("El segundo valor del arreglo es: {}", arreglo[1]);

    let arreglo2: [i128; 5] = [1,2,3,4,5];

    // Strings

    let nombr = "Steep Salvador";
}
