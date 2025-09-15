pub fn ejercicio9(){
    /*
    9- Escribir un programa que defina un arreglo de 5 números enteros, y luego imprima la
    suma de los valores del arreglo.
    */

    println!(" \n Ejercicio 9 \n");

    let mut total:u8 = 0;
    let arreglo:[u8;5]=[1,2,3,4,5];

    for numero in arreglo {
        total = total + numero;
    }

    println!("{:?}", total);

    println!(" \n --------- \n");

}

