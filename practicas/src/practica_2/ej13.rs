/*
    13- Definir una función llamada ordenar_nombres que recibe 
    un arreglo de String y los ordena en orden alfabético.
*/    

fn ordenar_nombres(nombres:&mut[String;7]){
    nombres.sort();
}

pub fn resuelve_ej13(){

    println!( "\n Ejercicio 13 \n");

    let mut arreglo_nombres:[String;7] = ["Héctor".to_string(), "Osvaldo".to_string(), "Gastón".to_string(),
    "Antonio".to_string(), "Quimey".to_string(), "Valentino".to_string(), "Bautista".to_string()];
    
    println!("Arreglo desordenado: {:?}", arreglo_nombres);
    ordenar_nombres(&mut arreglo_nombres);
    println!("Arreglo ordenado: {:?}", arreglo_nombres);
        
}

//Tests ~ Ejercicio 13
#[test]
fn test_ordenar_nombres_ej13(){
    
    let mut arreglo_nombres:[String;7] = ["Héctor".to_string(), "Osvaldo".to_string(), "Gastón".to_string(),
    "Antonio".to_string(), "Quimey".to_string(), "Valentino".to_string(), "Bautista".to_string()];
    
    ordenar_nombres(&mut arreglo_nombres);
    
    let arreglo_ordenado:[String; 7] =  ["Antonio".to_string(), "Bautista".to_string(), "Gastón".to_string(),
    "Héctor".to_string(), "Osvaldo".to_string(), "Quimey".to_string(), "Valentino".to_string()];
    
    assert_eq!(arreglo_nombres, arreglo_ordenado);

}