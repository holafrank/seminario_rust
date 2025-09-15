pub fn ejercicio7(){
    /*
    7- Escribir un programa que defina una variable de tipo arreglo que contenga seis números
    enteros, y luego multiplique cada valor del arreglo por un valor constante definido,
    modificando el contenido del arreglo.
    */

    println!(" \n Ejercicio 7 \n");
    const FACTOR:u8 = 10;
    let mut arreglo:[u8;5]=[5,7,2,9,0];

    println!("Array Original: {:?}", arreglo);

    for i in 0..arreglo.len() {
        arreglo[i] = arreglo[i] * FACTOR;
    }

    println!("Array Multiplicado: {:?}", arreglo);

    println!(" \n --------- \n");

}
