/*
    1- Escriba una función que reciba un vector de números enteros y retorna la cantidad de números
    primos. Cree un trait para la determinación del número primo e impleméntelo según corresponda.
    Utilice la función iter sobre el vector y aplique un closure para resolverlo.
*/

extern crate is_prime;
use is_prime::*;

fn cantidad_primos(vector: &Vec<u32>) -> u32 {
    let cantidad_primos:usize= vector.iter().filter(|&v| v.es_primo()).count();
    return cantidad_primos as u32;
}
pub trait EsPrimo {
    fn es_primo(&self) -> bool;
}
impl EsPrimo for u32{
    fn es_primo(&self) -> bool{    
        return is_prime(& self.to_string());
    }
}

pub fn ejercicio1(){
    let vector: Vec<u32> = vec![10,11,12,13,14,15];
    println!("Cantidad de numero primos: {:?}", cantidad_primos(&vector));
}

#[test]
fn test_cantidad_primos(){
    let vector: Vec<u32> = vec![10,11,12,13,14,15];
    assert_eq!(cantidad_primos(&vector), 2);
}
#[test]
fn test_cantidad_primos_cero(){
    let vector: Vec<u32> = vec![10,12,14,15];
    assert_eq!(cantidad_primos(&vector), 0);
}
#[test]
fn test_cantidad_primos_vacio(){
    let vector: Vec<u32> = vec![];
    assert_eq!(cantidad_primos(&vector), 0);
}