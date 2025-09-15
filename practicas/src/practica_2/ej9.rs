/*
    9- Definir la funcion llamada cantidad_en_rango que recibe 3 parametros:
    1 arreglo de enteros, un numero entero llamado inferior y otro numero entero
    llamado superior. Esta funcion retorna la cantidad de numeros del arreglo
    que estan entre el rango de los parametros inferior y superior inclusive.   
*/

fn es_valido(inf: &i32, sup: &i32) -> bool {
    return inf<sup;
}


fn cantidad_en_rango(inf: &i32, sup: &i32, arreglo:&[i32;10]) -> i32{
    if es_valido(inf, sup){
        let mut cant:i32 = 0;
        for item in arreglo{
            if item >= inf && item <= sup{
                cant=cant+1;
            }
        }
        return cant;
    }else{
        return -1;
    }
}
pub fn resuelve_ej9(){
    
    println!( "\n Ejercicio 9 \n");

    let sup:i32 = 10;
    let inf:i32 = 5;

    let arreglo_de_enteros:[i32;10] = [2,3,4,5,6,7,8,9,10,11];
    //los numeros tal que (5 >= X <= 10) son (5,6,7,8,9,10) --> 6 en total
    println!("Cantidad de numeros del arreglo que estan entre {} y {} inclusive es: {}"
    , inf, sup, cantidad_en_rango(&inf, &sup, &arreglo_de_enteros));
}

//Tests ~ Ejercicio 9
#[test]
fn test_cantidad_en_rango_ej9(){
    let superior :i32 = 9;
    let inferior :i32 = 5;
    let arreglo:[i32;10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    assert_eq!(cantidad_en_rango(&inferior, &superior, &arreglo), 5 );
    assert_eq!(cantidad_en_rango(&superior, &inferior, &arreglo), -1);
    assert_eq!(cantidad_en_rango(&11, &20, &arreglo), 0);
}