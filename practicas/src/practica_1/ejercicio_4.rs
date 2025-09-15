pub fn ejercicio4(){
    /*
    4- Escribir un programa que defina una tupla que contenga una cadena, un número entero
    con signo y un valor booleano, y luego imprima cada valor de la tupla
    */

    println!(" \n Ejercicio 4 \n");

    let tupla:(String, i32, bool) = ("Hola".to_string(), 5023, true);

    println! ("{:?}", tupla);
    println! ("{}", tupla.0);
    println! ("{}", tupla.1);
    println! ("{}", tupla.2);

    println!(" \n --------- \n");

}
