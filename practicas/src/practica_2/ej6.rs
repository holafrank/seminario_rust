
/*
    6- Definir la funcion llamada longitud_de_cadena que recibe un arreglo de String y retorna un
    arreglo con la longitud de las cadenas del parametro, correspondiendose en posicion del arreglo.
*/

fn longitud_de_cadena(c:&[String; 5]) -> [usize; 5]{
    let mut longitudes: [usize; 5] = [0;5];
    let mut i:usize = 0;
    for item in c{
        longitudes[i] = item.chars().count();
        i=i+1;
    }
    return longitudes;

}

pub fn resuelve_ej6(){
    
    println!( "\n Ejercicio 6 \n");

    let cadena: [String; 5] = ["Hola".to_string(), "🤯".to_string(), "大家好!".to_string(), "Y ahora,".to_string(), "...este es el final".to_string()];
    let long_de_cadenas = longitud_de_cadena(&cadena); //<-- con & se pasa como referencia!!
        
    println!("Arreglo de cadenas: {:?}", cadena);
    println!("La longitud de cada cadena es: {:?}", long_de_cadenas);
        
}


//Tests ~ Ejercicio 6
#[test]
fn test_longitud_de_cadena_ej6(){
    let cadena: [String; 5] = ["Uno".to_string(), "🤯".to_string(),
    "Dos".to_string(), "Tres".to_string(), "Cuatro".to_string()];
    assert_eq!( longitud_de_cadena(&cadena), [3,1,3,4,6] );
}
    