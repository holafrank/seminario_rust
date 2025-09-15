pub fn ejercicio8(){
    /*
    8- Escribir un programa que defina una constante de tipo cadena, y luego imprima el
    número de veces que un caracter específico ingresado por el usuario aparece en la cadena.
    Se debe imprimir el resultado.
    */

    println!(" \n Ejercicio 8 \n");

    const CADENA:&str = "Y entre los libros de la buena memoria se queda oyendo como un ciego frente al mar";
    
    println!("Ingresar un caracter:");
    let mut caracter = String::new();
    std::io::stdin().read_line(&mut caracter).expect("Error");
    let letra:char = caracter.trim().parse().expect("¡Ups! Lo que usted ingresó no es un caracter.");
    
    let mut contador_letras:u8 = 0;
    let vector_letras: Vec<char> = CADENA.chars().collect();

    for l in vector_letras {
        if l == letra {
            contador_letras = contador_letras + 1;
        }
    }
    print!("La cantidad de veces que aparece el caracter {:?} en la cadena es: {:?}", letra, contador_letras);

    println!(" \n --------- \n");
        
}

