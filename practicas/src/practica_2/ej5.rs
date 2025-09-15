/*
    5- Defina la funcion duplicar_valores que recibe un arreglo de numeros flotantes
    y retorna un arreglo nuevo con los valores duplicados del parametro.
*/

/* 
    ¿Y por qué no podríamos devolver un array slice en una función?
    
    "There is no trick and you can't. A slice is a form of borrow, by definition a slice
    refers to memory owned by some other collection (static memory, a vector, an array, ...).
    This means like every other borrow it can't be returned if it borrows data from the local
    scope, that would result in a dangling pointer (as the actual owner will get destroyed when
    the scope ends). The correct thing to do is to just return a Vec."
    https://stackoverflow.com/questions/75394760/returning-an-array-slice-of-u32-from-a-function

    Puedo en cambio sí, usar una referencia mutable de un slice como parámetro....

*/

fn duplicar_valores(a: &[f32], result: &mut[f32]) {

    if  a.len() == result.len() {

        let mut i = 0;
    
        for item  in a  {
            result[i] = item*2.0;
            i = i+1;
        }

    } else {
        println!("Los arreglos deberían ser de la misma longitud...");
    }
}

/*
    Si yo quisiera que la función *retorne* el arreglo nuevo, ya ahí debería empezar a jugar con
    lifetimes y otras cosas que se escapan a nuestros conocimientos actuales para esta práctica;
    y de todas formas, para este tipo de situaciones siempre usaríamos un Vec...

    Joaco: "En los momentos en que tenés que devolver un slice solo se puede devolver un array,
    en ese caso la única opción es usar tamaños genéricos N. 
    
    fn ejemplo_array_tamano_generico<const N: usize>(arr1: &[u8;N]) -> [u8;N] {
    arr1.clone()
    }
*/

fn duplicar_valores_fijo(a:&[f32; 5]) -> [f32; 5]{
    
    let mut i = 0;
    let mut nuevo: [f32; 5] = [0.0; 5];
    
    for item in a  {
        nuevo[i] = item * 2.0;
        i += 1;
    }

    return nuevo;

}

pub fn resuelve_ej5(){
    
    println!( "\n Ejercicio 5 \n");

    let arreglo: [f32; 5] = [1.0, 2.8, 3.14, 4.4, 5.999];
    let mut res: [f32;5] = [0.0; 5];
    duplicar_valores(&arreglo, &mut res);
    println!("Arreglo: {:?}", arreglo);
    println!("Arreglo duplicado: {:?}", res);
    println!("{}", "🤯");
    println!("Y ahora con la función que retorna arreglos de longitud fija 5:");
    let arreglo: [f32; 5] = [1.0, 2.8, 3.14, 4.4, 5.999];
    let arreglo_duplicado = duplicar_valores_fijo(&arreglo);
    println!("Arreglo de longitud fija: {:?}", arreglo);
    println!("Arreglo duplicado: {:?}", arreglo_duplicado);
}

//Tests ~ Ejercicio 5
#[test]
fn test_duplicar_valores_ej_5(){
    let arreglo: [f32; 5] = [1.0, 2.2, 3.3, 4.4, 5.1];
    let mut result: [f32; 5] = [0.1; 5];

    duplicar_valores(&arreglo, &mut result);

    assert_eq!(result, [2.0, 4.4, 6.6, 8.8, 10.2]);
}

#[test]
fn test_duplicar_valores_diff_len(){
    let arreglo: [f32; 5] = [1.0, 2.2, 3.3, 4.4, 5.1];
    let mut result: [f32; 6] = [0.3; 6];

    duplicar_valores(&arreglo, &mut result);

    assert_eq!(result, [0.3, 0.3, 0.3, 0.3, 0.3, 0.3]);
}

#[test]
fn test_duplicar_valores_fijo(){
    let arreglo: [f32; 5] = [1.0, 2.2, 3.3, 4.4, 5.1];
    let mut result: [f32; 5] = [0.1; 5];

    duplicar_valores(&arreglo, &mut result);

    assert_eq!(duplicar_valores_fijo(&arreglo), [2.0, 4.4, 6.6, 8.8, 10.2]);
}
