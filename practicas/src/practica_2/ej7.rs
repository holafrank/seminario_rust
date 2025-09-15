/*
    7- Definir la función llamada cantidad_de_mayores que recibe como parámetro
    un arreglo de números enteros y un número entero llamado límite. 
    Esta función retorna la cantidad de números mayores al límite que tiene el arreglo.
*/    

fn es_mayor(num1:&i32, num2:&i32) -> bool {
            
    return num1>num2;
}
    
fn cantidad_de_mayores(arreglo:&[i32; 5], limite:&i32) -> i32{  
    let mut contador:i32 = 0;

    for item in arreglo{
        if es_mayor(item, limite){
                contador += 1;
        }
    }

    return contador;
}

pub fn resuelve_ej7(){

    println!( "\n Ejercicio 7 \n");
        
    let arreglo_fijo:[i32; 5] = [1, 3, 98, 34, 9]; 
    let limite:i32= 6;
    
    println!("Mi límite es: {:?}, mi arreglo fijo es: {:?}", limite, arreglo_fijo);
    println!("La cantidad de elementos mayores al límite es: {:?}", cantidad_de_mayores(&arreglo_fijo, &limite))
        
}


//Tests ~ Ejercicio 7
#[test]
fn test_es_mayor_ej7(){
    assert_eq!(es_mayor(&5, &1), true);
    assert_eq!(es_mayor(&1, &5), false);
}
#[test]
fn test_cantidad_de_mayores_ej7(){
    let arreglo:[i32; 5] = [1, 3, 98, 99, 100]; 
    let limite:i32= 6;
    assert_eq!(cantidad_de_mayores(&arreglo, &limite), 3);
}
    
    
    