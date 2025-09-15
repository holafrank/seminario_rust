pub fn ejercicio5(){
    /*
    5- Escribir un programa que defina una variable de tipo cadena, y luego permita al usuario
    ingresar una cadena por teclado para concatenar su valor. El programa debe imprimir la
    cadena en mayúsculas.
    */

    println!(" \n Ejercicio 5 \n");

    println!("Ingresar un String:");
    let mut cadena = String::new();
    std::io::stdin().read_line(&mut cadena).expect("Error");

    let mayusc: String = cadena.to_uppercase();
    println!("{}", mayusc);

    println!(" \n --------- \n");

}

