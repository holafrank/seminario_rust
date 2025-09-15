/*
    10- Definir la funcion llamada cantidad_de_cadenas_mayor_a que recibe como parametros
    un arreglo de String y un entero llamado limite. Esta funcion retorna la cantidad de
    Strings del arreglo que son de longitud mayor al parametro limite.
*/

fn cantidad_de_cadenas_mayor_a(arreglo:&[String;7], lim:&usize)-> i32{
    let mut cant:i32 = 0;
    for item in arreglo{
        if &item.chars().count() > lim {
            println!("{} tiene {} caracteres", item, item.chars().count());
            cant += 1;
        }
    }
    return cant
}

pub fn resuelve_ej10(){
    
    println!( "\n Ejercicio 10 \n");
    
    let arreglo_de_strings: [String; 7] = ["Ciao".to_string(), "Hola".to_string(), "Hello".to_string(),
    "大家好".to_string(), "Bonjour".to_string(), "Bonna sera".to_string(), "Boa noite".to_string()];
    
    let limite:usize= 5;
    println!("La cantidad de cadenas con mas de {} caracteres es: {}", limite, 
    cantidad_de_cadenas_mayor_a(&arreglo_de_strings, &limite));
}
    
//Tests ~ Ejercicio 10
#[test]
fn test_cantidad_de_cadenas_mayor_a_ej10(){
    let a: [String;7] = ["A".to_string(), "Ab".to_string(), "Abc".to_string(), "Abcd".to_string(),
    "Abcde".to_string(), "Abcdef".to_string(), "Abcdefg".to_string()];
    assert_eq!(cantidad_de_cadenas_mayor_a(&a, &2), 5);
}