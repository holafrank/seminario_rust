/*
    4- Definir la funcion llamada cantidad_impares que recibe como parametro
   un arreglo de numeros enteros y retorna la cantidad de numeros impares
*/
   
fn es_impar(num:&i32) -> bool{
   
    return num%2==1;
}

fn cantidad_impares(a: &[i32]) -> i32 {
    let mut contador = 0;
    for item  in a  {
        if es_impar(&item){
            contador += 1;
        }
    }
    return contador;
}

pub fn resuelve_ej4(){

    println!( "\n Ejercicio 4 \n");

    let arreglo: [i32; 5] = [1,2,3,4,5];
   
    println!("Arreglo: {:?}", arreglo);
    println!("Cantidad de impares: {}", cantidad_impares(&arreglo));
   
}

//Tests ~ Ejercicio 4
#[test]
fn test_cant_impares_ej_4(){
    assert_eq!(cantidad_impares(&[1,2,3,4,5]), 3);
}
#[test]
fn test_es_impar_ej_4(){
    assert_eq!(es_impar(&1), true);
}

   
   