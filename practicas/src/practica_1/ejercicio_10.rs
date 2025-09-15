pub fn ejercicio10(){
    /*
    10- Escribir un programa que defina dos arreglos de 5 números enteros cada uno, y luego
    cree un tercer arreglo que contenga la suma de los elementos de los dos arreglos
    originales.
    */

    println!(" \n Ejercicio 10 \n");

    let arreglo:[u8;5]=[1,2,3,4,5];
    let arreglo2:[u8;5]=[5,4,3,2,1];
    let mut arreglo3:[u8;5]=[0;5];

    for i in 0..5 {
        arreglo3[i] = arreglo[i] + arreglo2[i];
    }
    
    println!("{:?}", arreglo3);

    println!(" \n --------- \n");

}

