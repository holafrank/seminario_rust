pub fn ejercicio12(){
    /*
    12- Escribir un programa que defina una tupla que contenga una cadena y un arreglo de
    enteros, y luego imprima la cadena y la suma de los valores en el arreglo.
    */

    println!(" \n Ejercicio 12 \n");

    let tupla:(&str,[u8;3]) = ("Hola", [1,2,3]);
    let mut total:u8 = 0;
    for item in tupla.1 {
        total = total + item;
    }

    println!("Cadena: {:?}", tupla.0);
    println!("Total del arreglo: {:?}", total);

    println!(" \n --------- \n");

}


