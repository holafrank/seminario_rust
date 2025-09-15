pub fn ejercicio11(){
    /*
    11- Escribir un programa que defina un arreglo de 5 cadenas, y luego permita al usuario
    ingresar una cadena por teclado. El programa debe imprimir un mensaje si la cadena
    ingresada por el usuario se encuentra en el arreglo.
    */

    println!(" \n Ejercicio 11 \n");

    let arreglo:[String;5] = ["为".to_string(),"人".to_string(),"民".to_string(), "服".to_string(), "务".to_string()];
    
    println!("Ingresar una cadena:");
    let mut palabra = String::new();
    std::io::stdin().read_line(&mut palabra).expect("Error");
    
    for item in arreglo{
        if item.eq(&palabra){
            print!("Se encontró la palabra ingresada en el arreglo");
        }
    }

    println!("No se encontró la palabra ingresada en el arreglo");

    println!(" \n --------- \n");

}

