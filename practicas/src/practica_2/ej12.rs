/*
    12- Definir una función llamada reemplazar_pares que recibe un
    arreglo de enteros y reemplaza todos los números pares por -1.
*/
        
fn reemplazar_pares (arreglo:&mut[i32; 10]){
    for item in arreglo{
        if *item % 2 == 0{
            *item = -1;
        }
    }
}

pub fn resuelve_ej12(){
    
    println!( "\n Ejercicio 12 \n");
        
    let mut arreglo:[i32;10] = [0,1,200,3,400,5,600,7,800,9];

    println!("Arreglo original: {:?}", arreglo);
    reemplazar_pares(&mut arreglo);
    println!("Reemplazando pares: {:?}", arreglo);

}

//Tests ~ Ejercicio 12
#[test]
fn test_reemplazar_pares_ej12(){
    let mut arreglo:[i32;10] = [ 0, 1, 200, 3, 400, 5, 600, 7, 800, 9 ];
    reemplazar_pares(&mut arreglo);
    assert_eq!(arreglo, [-1, 1, -1, 3, -1, 5, -1, 7, -1, 9]);
}