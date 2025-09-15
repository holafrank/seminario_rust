pub fn ejercicio1(){
    /*
    1- Escribir un programa que defina una variable de tipo flotante con algún valor,
    y luego permita al usuario ingresar un número decimal por teclado para
    multiplicar, dividir, sumar y restar su valor. Se deben imprimir los resultados. 
    */

        println!(" \n Ejercicio 1 \n");

        println!("Ingrese el primer operando");
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).expect("Error");
        let numero1:f32 = buf.trim().parse().expect("No es f32");

        println!("ingrese un operador");
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).expect("Error");
        
        let operador:char = buf.trim().parse().expect("No es char");

        println!("Ingrese el segundo operando");
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).expect("Error");
        let numero2:f32 = buf.trim().parse().expect("No es f32");

        match operador {
            '+' => println!("{}", numero1 + numero2),
            '-' => println!("{}", numero1 - numero2),
            '*' => println!("{}", numero1 * numero2),
            '/' => println!("{}", numero1 / numero2),
            _ => print!("No es un operador valido -- intentar con +, -, * o /")
        }

        println!(" \n --------- \n");
        
}
