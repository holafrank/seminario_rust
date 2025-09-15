/*
    8- Definir la funcion llamada sumar_arreglos que recibe 2 arreglos  del mismo tamaño de numeros
    flotantes y retorna un nuevo arreglo que contiene la suma de los elementos de los arreglos pasados
    por parametro, correspondiendose el resultado con cada posicion de los arreglos pasados por parametro.
*/

fn sumar_arreglos(a1:&[f32; 5], a2: &[f32; 5]) -> [f32; 5]{
    let mut totales: [f32; 5] = [0.0;5];
    let mut i:usize =0;
    loop {
        totales[i] = a1[i] + a2[i];
        if i==a1.len()-1{
            break;
        }
        i=i+1;
    }
    return totales;
}
    
pub fn resuelve_ej8(){

    println!( "\n Ejercicio 8 \n");
    
    let arreglo1:[f32; 5] = [9.0001, 2.1, 3.14, 50.1, 9999.999];
    let arreglo2:[f32; 5] = [0.0009, 7.9, 6.86, 49.9, 0.001];

    println!("Arreglo 1: {:?}", arreglo1);
    println!("Arreglo 2: {:?}", arreglo2);
    println!("Arreglo resultante: {:?}", sumar_arreglos(&arreglo1, &arreglo2));
        
}

//Tests ~ Ejercicio 8
#[test]
fn test_sumar_arreglos_ej8(){
    let arreglo1:[f32; 5] = [1.1, 2.0, 3.3, 4.0, 5.5];
    let arreglo2:[f32; 5] = [5.5, 4.0, 3.3, 2.0, 1.1];
    assert_eq!(sumar_arreglos(&arreglo1, &arreglo2), [6.6, 6.0, 6.6, 6.0, 6.6] );
}
    
        