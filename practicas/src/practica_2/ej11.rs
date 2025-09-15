/*
    Definir la funcion llamada multiplicar_valores que recibe como parametro un arreglo
    de enteros y otro numero entero llamado factor. Esta funcion multiplica los valores
    del arreglo por el parametro factor modificandolo.
*/    
       
fn multiplicar_valores (arreglo:&mut[i32; 3], factor:&i32){
    for item in arreglo{
        *item = *item * factor;         //con * lo desreferencias para acceder al valor de la celda
    }
}
    
pub fn resuelve_ej11(){
    
    println!( "\n Ejercicio 11 \n");
    
    let mut arreglo_og: [i32; 3] = [1, 2, 3];
    let factor:i32 = 10;

    println!("Arreglo sin multiplicar {:?}", arreglo_og);
    multiplicar_valores(&mut arreglo_og, &factor);
    println!("Arreglo multiplicado por {:?}: {:?}", factor, arreglo_og);

}

//Tests ~ Ejercicio 11
#[test]
fn test_multiplicar_valores_ej11(){
    let mut arreglo_og: [i32; 3] = [1, 2, 3];
    multiplicar_valores(&mut arreglo_og, &2);
    assert_eq!(arreglo_og, [2,4,6]);
}
        
