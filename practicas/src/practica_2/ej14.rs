/*
    14- Definir una función llamada incrementar que recibe como parámetro
    un número flotante e incrementa en 1 su valor.
*/

fn incrementar(numeritos:&mut[f64;5]){
    for item in numeritos{
        *item += 1.0;
    }
}
    
pub fn resuelve_ej14(){

    println!( "\n Ejercicio 14 \n");
    
    let mut arreglo_de_numeritos:[f64;5] = [1.9, 3.14165797263, 9.99, 1.34, 198.2];

    println!("Arreglo original: {:?}", arreglo_de_numeritos);
    incrementar(&mut arreglo_de_numeritos);
    println!("Arreglo incremenmtado: {:?}", arreglo_de_numeritos);
    
}

//Tests ~ Ejercicio 14
#[test]
fn test_incrementar_ej14(){
    let mut arreglo:[f64;5] = [0.9, 0.14165797263, 0.99, 0.34, 0.2];
    incrementar(&mut arreglo);
    assert_eq!(arreglo, [1.9, 1.14165797263, 1.99, 1.34, 1.2])

}
    