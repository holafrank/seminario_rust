extern crate is_prime;
use is_prime::*;

/*
    2- Definir la función llamada es_primo que recibe un número entero
    positivo mayor a 1 y retorna true si es primo, false caso contrario.
*/

fn es_primo(&num: &u32) -> bool {
    return is_prime(&num.to_string());
}

pub fn resuelve_ej2() {
    println!("\n Ejercicio 2 \n");

    println!("Ingresar un numero");
    let mut numero = String::new();
    std::io::stdin().read_line(&mut numero).expect("Error");
    let numero: u32 = numero
        .trim()
        .parse()
        .expect("No es un número entero positivo");

    println!("¿Es primo?");
    println!("{}", es_primo(&numero));
}

//Test ~ Ejercicio 2
#[test]
fn test_es_primo_ej_2() {
    assert_eq!(es_primo(&13), true);
    assert_eq!(es_primo(&14), false);
}
