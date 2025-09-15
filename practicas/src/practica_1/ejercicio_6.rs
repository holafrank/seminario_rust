pub fn ejercicio6(){
    /*
    6- Escribir un programa que defina una variable de tipo entero sin signo, y luego permita al
    usuario ingresar un número entero por teclado para sumarse con la variable definida. El
    programa debe imprimir el valor del número elevado al cuadrado.
    */

    println!(" \n Ejercicio 6 \n");

    let num1: u32 = 10;

    println!("Ingresar un número:");
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("Error");
    let num2:u32 = buf.trim().parse().expect("No es u32");

    let sum:u32 = num1 + num2;

    let square = sum*sum;
    println!("{}", square);

    println!(" \n --------- \n");

}
