pub fn ejercicio3(){
    /*
    3- Escribir un programa que defina una variable de tipo booleano, y luego permita al usuario
    ingresar un valor booleano por teclado para actualizar su valor haciendo las operaciones
    and y or. Se deben imprimir ambos resultados.
    */

    println!( "\n Ejercicio 3 \n");

    println!("Ingrese un operador booleano:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Error");
    let input_bool:bool= input.trim().parse().expect("Se ingreso algo distinto a un valor booleano!");

    let valor_bool:bool = false;

    println!("Se ingresó: {:?}, el valor de la variable es: {:?}", input_bool, valor_bool);
    println!("and: {:?}, or: {:?}", input_bool && valor_bool, input_bool || valor_bool);

    println!(" \n --------- \n");

}

