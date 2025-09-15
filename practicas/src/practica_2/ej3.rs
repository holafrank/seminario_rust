/*
    3- Definir la función llamada suma_pares que recibe como parámetro un
    arreglo de números enteros y retorna la suma de los números pares.
*/

fn es_par(num:&i32) -> bool{
    
    return (num % 2) < 1;
}

fn suma_pares(arreglo: &[i32]) -> i32{
    
    let mut total:i32 = 0;
    for elemento in arreglo {
        if es_par(&elemento){
            total+= elemento;
        }
    }    
    return total;
}

pub fn resuelve_ej3(){
    
    println!( "\n Ejercicio 3 \n");
    let arreglo = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    println!("Arreglo original: {:?}", arreglo);
    println!("Suma de los pares: {:?}", suma_pares(&arreglo));
    
}

//Test ~ Ejercicio 3
#[test]
fn test_suma_pares_ej_3(){
    assert_eq!(suma_pares(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 30);
}
    