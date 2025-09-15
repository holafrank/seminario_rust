/*
    1- Definir la función llamada es_par que recibe como parámetro un
    número entero y retorna true si el número es par, false caso contrario.
*/

pub fn es_par(num:i32) -> bool{
    return (num % 2) < 1;
}

pub fn resuelve_ej1(){

    println!("Ingresar un numero");
    let mut numero = String::new();
    std::io::stdin().read_line(&mut numero).expect("Error");
    let numero:i32 = numero.trim().parse().expect("No es i32");

    println!("¿Es par?");
    println!("{}", es_par(numero));
}

//Test ~ Ejercicio 1
#[test]
fn test_es_par_ej1(){
    assert_eq!(es_par(10), true);
    assert_eq!(es_par(11), false);
}


