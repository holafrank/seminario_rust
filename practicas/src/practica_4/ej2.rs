#![allow(dead_code)]

/*
    2- Dado el siguiente struct:
        
    struct Persona<'a>{
        nombre:&'a str,
        apellido:&'a str,
        direccion:&'a str,
        ciudad:&'a str,
        salario:f64,
        edad:u8,
    }
        
    a- Escriba una función que reciba un vector de personas y otro parámetro que indica un
    salario y retorna un listado de personas donde el salario es mayor al parámetro recibido.
        
    b- Escriba una función que reciba un vector de personas, edad y el nombre de una ciudad, y
    retorna las personas mayores al parámetro edad y que viven en el valor del parámetro ciudad.
    
    c- Escriba una función que reciba un vector de personas y un nombre de una ciudad y retorna
    true si todas las personas viven en la ciudad pasada por parámetro, false caso contrario.
        
    d- Escriba una función que reciba un vector de personas y un nombre de una ciudad y retorna
    true si al menos vive una persona en la ciudad pasada por parámetro,, false caso contrario.
        
    e- Escriba una función que reciba un arreglo de personas y una persona y retorna true si la
    persona existe en el arreglo, false caso contrario
        
    f -Escriba una función que reciba un arreglo de personas y retorna un arreglo con las
    edades de las personas.
        
    g - Escriba una función que reciba un arreglo de personas y retorna la persona con el menor
    salario y la persona con el mayor salario, en caso de que haya más de una persona en cada
    categoría desempatar por la edad más grande.
        
    Nota: Implemente todos los métodos y traits que considere para resolver los ejercicios.
    Todos los ejercicios deben resolverse con iterator y closure.

*/

#[derive(Debug, PartialEq)]
struct Persona<'a>{
    nombre:&'a str,
    apellido:&'a str,
    direccion:&'a str,
    ciudad:&'a str,
    salario:f64,
    edad:u8,
}

impl<'a> Persona<'a>{

    pub fn sueldo_es_mayor_que(&self, s:&f64) -> bool {
        return self.salario > *s;
    }
    
    pub fn edad_mayor_que(&self, edad:&u8) -> bool {
        return self.edad > *edad;
    }
    
    pub fn misma_ciudad(&self, city:&str) -> bool {
        return self.ciudad == city;
    }
    
    pub fn mi_edad(&self) -> u8 {
        return self.edad.clone();
    }
    
    pub fn mi_sueldo(&self) -> f64 {
        return self.salario.clone();
    }
}
/*
impl<'a> PartialEq for Persona<'a>{
    fn eq(&self, other: &Self) -> bool {
        return self.salario == other.salario &&
        self.nombre == other.nombre &&
        self.apellido == other.apellido &&
        self.ciudad == other.ciudad &&
        self.direccion ==other.direccion &&
        self.edad == other.edad;
    }
}
 */
/// Función que recibe un vector de personas y otro parámetro que indica un salario y retorna un listado de personas donde el salario es mayor al parámetro recibido.
fn salarios_mayores_que<'a> (personas:&Vec<&'a Persona<'a>>, otro_salario:&f64) -> Vec<&'a Persona<'a>> {
    personas.iter()
        .filter(|&x| x.sueldo_es_mayor_que(otro_salario))
        .map(|&x| x)
        .collect()

}

/// Función que reciba un vector de personas, edad y el nombre de una ciudad, y retorna las personas mayores al parámetro edad y que viven en el valor del parámetro ciudad.
fn mayor_edad_misma_ciudad <'a> (personas:&Vec<&'a Persona<'a>>, edad:&u8, ciudad:&str) -> Vec<&'a Persona<'a>> {
    personas.iter()
        .filter(|&x| x.misma_ciudad(ciudad) && x.edad_mayor_que(&edad))
        .map(|&x| x)
        .collect()
}
    
/// Función que recibe un vector de personas y un nombre de una ciudad y retorna true si todas las personas viven en la ciudad pasada por parámetro, false caso contrario.
fn todos_viven_en <'a>(personas:&Vec<&'a Persona<'a>>, city:&str) -> bool {
    return personas.iter().all(|p| p.misma_ciudad(city));
}
    
/// Función que recibe un vector de personas y un nombre de una ciudad y retorna true si al menos vive una persona en la ciudad pasada por parámetro, false caso contrario.
fn alguno_vive_en<'a>(personas:&Vec<&'a Persona<'a>>, city:&str) -> bool {
    return personas.iter().any(|p| p.misma_ciudad(city));
}
    
/// Función que recibe un arreglo de personas y una persona y retorna true si la persona existe en el arreglo, false caso contrario.
fn existe_en<'a>(personas: &[Persona], person:&Persona) -> bool {
    return personas.iter().any(|p|p.eq(person));
}

/// Función que recibe una colección de personas y retorna una colección con las edades de las personas.
fn arreglo_de_edades<'a>(personas: &'a[Persona]) -> Vec<u8> {
    if personas.is_empty(){
        panic!("Arreglo vacío!")
    }else{
        personas.iter()
        .map(|p| p.mi_edad())
        .collect()
    }
}

/// Función que recibe un arreglo de personas y retorna la persona con el menor salario y la persona con el mayor salario, en caso de que haya más de una persona en cada categoría desempatar por la edad más grande.
fn mejor_y_peor<'a>(personas: &'a[Persona]) -> Vec<&'a Persona<'a>>{

    if personas.is_empty(){
        panic!("Arreglo vacío!")

    }else{

        let mut vector_result: Vec<&'a Persona<'a>> = Vec::new();

        if let Some(max) = el_mejor(personas){
            vector_result.push(max);
        }else{
            panic!("Error! No hay máximo!")
        }

        if let Some(min) = el_peor(personas){
            vector_result.push(min);
        }else{
            panic!("Error! No hay minimo!")
        }

        return vector_result;
    }
}
    
fn el_mejor<'a>(personas: &'a[Persona]) -> Option<&'a Persona<'a>> {

    let mut maximo  =
        personas.iter()
        .max_by(|a, b| a.salario.total_cmp(&b.salario));

    if maximo.is_some(){

        let salario_max = maximo.unwrap().salario;

        let empates_max:Vec<&'a Persona<'a>> = personas.iter()
            .filter(|max| max.salario == salario_max)
            .collect();
        if empates_max.len() > 1 {
            maximo =
            empates_max.iter()
            .map(|p| &**p).max_by(|a, b| a.mi_edad().cmp(&b.mi_edad()));
            
            /*  Este .map está por cuestiones de tipos. Si no lo pongo pasa esto:
            mismatched types
            expected enum `Option<&practica_4::ej2::Persona<'_>>`
            found enum `Option<&&practica_4::ej2::Persona<'_>>`rustcClick for full compiler diagnostic
            ej2.rs(157, 9): expected due to the type of this binding
            ej2.rs(158, 9): expected due to this value
            ej2.rs(171, 58): try using `.map(|v| &**v)` to convert `Option<&&practica_4::ej2::Persona<'_>>`
            to `Option<&practica_4::ej2::Persona<'_>>`: `.map(|v| &**v)`  */

        }    

    }

    return maximo;
}

fn el_peor<'a>(personas: &'a[Persona]) -> Option<&'a Persona<'a>> {

    let mut minimo =
        personas.iter()
        .min_by(|a, b| a.salario.total_cmp(&b.salario));

    if minimo.is_some(){

        let salario_min = minimo.unwrap().salario;

        let empates_min:Vec<&'a Persona<'a>> = personas.iter()
            .filter(|min| min.salario == salario_min)
            .collect();
        if empates_min.len() > 1 {
            minimo =
            empates_min.iter()
            .map(|p| &**p).max_by(|a, b| a.mi_edad().cmp(&b.mi_edad()));
        }    

    }

    return minimo;
}


pub fn ejercicio2(){



    // Función que recibe un vector de personas y otro parámetro que indica un salario y retorna un listado
    //de personas donde el salario es mayor al parámetro recibido.
    

}
#[test]
fn test_salarios_mayores_que(){

    let p1:Persona = Persona { nombre: "Pablo", apellido: "Zappa", direccion: "Dirección1", ciudad: "La Plata", salario: 1000.0, edad: 30 };
    let p2:Persona = Persona { nombre: "Leandro", apellido: "Arispe", direccion: "Dirección1", ciudad: "La Plata", salario: 1500.0, edad: 33 };
    let p3:Persona = Persona { nombre: "Laura", apellido: "Tapia", direccion: "Dirección2", ciudad: "La Plata", salario: 2000.0, edad: 30 };
    
    let personas = vec![&p1, &p2, &p3];

    //Comparación &Vec<&'a Persona<'a>> !!
    assert_eq!(salarios_mayores_que(&personas, &1499.0), vec![&p2, &p3]);
}

#[test]
fn test_salarios_mayores_que_vacio(){

    let p1:Persona = Persona { nombre: "Pablo", apellido: "Zappa", direccion: "Dirección1", ciudad: "La Plata", salario: 1000.0, edad: 30 };
    let p2:Persona = Persona { nombre: "Leandro", apellido: "Arispe", direccion: "Dirección1", ciudad: "La Plata", salario: 1500.0, edad: 33 };
    let p3:Persona = Persona { nombre: "Laura", apellido: "Tapia", direccion: "Dirección2", ciudad: "La Plata", salario: 2000.0, edad: 30 };
    
    let personas = vec![&p1, &p2, &p3];
    let vacio: Vec<&Persona<'_>> = Vec::new();

    assert_eq!(salarios_mayores_que(&personas, &2000.0), vacio);
}

#[test]
fn test_mayor_edad_misma_ciudad(){

    let p1:Persona = Persona { nombre: "Pablo", apellido: "Zappa", direccion: "Dirección1", ciudad: "La Plata", salario: 1000.0, edad: 30 };
    let p2:Persona = Persona { nombre: "Leandro", apellido: "Arispe", direccion: "Dirección1", ciudad: "La Plata", salario: 1500.0, edad: 33 };
    let p3:Persona = Persona { nombre: "Laura", apellido: "Tapia", direccion: "Dirección2", ciudad: "La Plata", salario: 2000.0, edad: 31 };
    
    let personas = vec![&p1, &p2, &p3];

    assert_eq!(mayor_edad_misma_ciudad(&personas, &30, "La Plata"), vec![&p2, &p3]);
}

#[test]
fn test_mayor_edad_misma_ciudad_vacio(){

    let p1:Persona = Persona { nombre: "Pablo", apellido: "Zappa", direccion: "Dirección1", ciudad: "La Plata", salario: 1000.0, edad: 30 };
    let p2:Persona = Persona { nombre: "Leandro", apellido: "Arispe", direccion: "Dirección1", ciudad: "La Plata", salario: 1500.0, edad: 33 };
    let p3:Persona = Persona { nombre: "Laura", apellido: "Tapia", direccion: "Dirección2", ciudad: "La Plata", salario: 2000.0, edad: 30 };
    
    let personas = vec![&p1, &p2, &p3];
    let vacio: Vec<&Persona<'_>> = Vec::new();

    assert_eq!(mayor_edad_misma_ciudad(&personas, &30, "CABA"), vacio);
    assert_eq!(mayor_edad_misma_ciudad(&personas, &40, "La Plata"), vacio);
}

#[test]
fn test_todos_viven_en(){

    let p1:Persona = Persona { nombre: "Pablo", apellido: "Zappa", direccion: "Dirección1", ciudad: "La Plata", salario: 1000.0, edad: 30 };
    let p2:Persona = Persona { nombre: "Leandro", apellido: "Arispe", direccion: "Dirección1", ciudad: "La Plata", salario: 1500.0, edad: 33 };
    let p3:Persona = Persona { nombre: "Laura", apellido: "Tapia", direccion: "Dirección2", ciudad: "La Plata", salario: 2000.0, edad: 31 };
    
    let personas = vec![&p1, &p2, &p3];

    assert_eq!(todos_viven_en(&personas, "La Plata"), true);
    assert_eq!(todos_viven_en(&personas, "CABA"), false);

}

#[test]
fn test_alguno_viven_en(){
  
    let p1:Persona = Persona { nombre: "Pablo", apellido: "Zappa", direccion: "Dirección1", ciudad: "La Plata", salario: 1000.0, edad: 30 };
    let p2:Persona = Persona { nombre: "Leandro", apellido: "Arispe", direccion: "Dirección1", ciudad: "La Plata", salario: 1500.0, edad: 33 };
    let p3:Persona = Persona { nombre: "Laura", apellido: "Tapia", direccion: "Dirección2", ciudad: "CABA", salario: 2000.0, edad: 31 };
      
    let personas = vec![&p1, &p2, &p3];
  
    assert_eq!(alguno_vive_en(&personas, "CABA"), true);
    assert_eq!(alguno_vive_en(&personas, "La Plata"), true);
    assert_eq!(alguno_vive_en(&personas, "Córdoba"), false);
  
}

#[test]
fn test_existe_en(){
     
    let p1:Persona = Persona { nombre: "Pablo", apellido: "Zappa", direccion: "Dirección1", ciudad: "La Plata", salario: 1000.0, edad: 30 };
    let p2:Persona = Persona { nombre: "Leandro", apellido: "Arispe", direccion: "Dirección1", ciudad: "La Plata", salario: 1500.0, edad: 33 };
    let p3:Persona = Persona { nombre: "Laura", apellido: "Tapia", direccion: "Dirección2", ciudad: "CABA", salario: 2000.0, edad: 31 };
         
    let personas = [p1, p2, p3];
     
    assert_eq!(existe_en(&personas, &Persona { nombre: "Laura", apellido: "Tapia", direccion: "Dirección2", ciudad: "CABA", salario: 2000.0, edad: 31 }), true);
    assert_eq!(existe_en(&personas, &Persona { nombre: "María", apellido: "Pérez", direccion: "Dirección5", ciudad: "La Plata", salario: 2500.0, edad: 32 }), false);
     
}

#[test]
fn test_arreglo_de_edades(){
     
    let p1:Persona = Persona { nombre: "Pablo", apellido: "Zappa", direccion: "Dirección1", ciudad: "La Plata", salario: 1000.0, edad: 30 };
    let p2:Persona = Persona { nombre: "Leandro", apellido: "Arispe", direccion: "Dirección1", ciudad: "La Plata", salario: 1500.0, edad: 33 };
    let p3:Persona = Persona { nombre: "Laura", apellido: "Tapia", direccion: "Dirección2", ciudad: "CABA", salario: 2000.0, edad: 31 };
         
    let personas = [p1, p2, p3];
     
    assert_eq!(arreglo_de_edades(&personas), [30, 33, 31]);

}

#[test]
#[should_panic(expected = "Arreglo vacío!")]
fn test_arreglo_de_edades_vacio(){
    
    let personas = [];
    arreglo_de_edades(&personas);

}

#[test]
fn test_mejor_y_peor_por_salario(){
         
    let p1:Persona = Persona { nombre: "Pablo", apellido: "Zappa", direccion: "Dirección1", ciudad: "La Plata", salario: 1000.0, edad: 30 };
    let p2:Persona = Persona { nombre: "Leandro", apellido: "Arispe", direccion: "Dirección1", ciudad: "La Plata", salario: 1500.0, edad: 33 };
    let p3:Persona = Persona { nombre: "Laura", apellido: "Tapia", direccion: "Dirección2", ciudad: "CABA", salario: 2000.0, edad: 31 };
             
    let personas = [p1, p2, p3];

    let vec_result = vec![
        &Persona { nombre: "Laura", apellido: "Tapia", direccion: "Dirección2", ciudad: "CABA", salario: 2000.0, edad: 31 }, 
        &Persona { nombre: "Pablo", apellido: "Zappa", direccion: "Dirección1", ciudad: "La Plata", salario: 1000.0, edad: 30 }];
         
    assert_eq!(mejor_y_peor(&personas), vec_result);
}

#[test]
fn test_mejor_y_peor_por_edad(){
         
    let p1:Persona = Persona { nombre: "Pablo", apellido: "Zappa", direccion: "Dirección1", ciudad: "La Plata", salario: 1000.0, edad: 30 };
    let p2:Persona = Persona { nombre: "Leandro", apellido: "Arispe", direccion: "Dirección1", ciudad: "La Plata", salario: 2000.0, edad: 33 };
    let p3:Persona = Persona { nombre: "Laura", apellido: "Tapia", direccion: "Dirección2", ciudad: "CABA", salario: 2000.0, edad: 31 };
    let p4:Persona = Persona { nombre: "Carolina", apellido: "Justo", direccion: "Dirección10", ciudad: "CABA", salario: 1000.0, edad: 21 };
             
    let personas = [p1, p2, p3, p4];

    let vec_result = vec![
        &Persona { nombre: "Leandro", apellido: "Arispe", direccion: "Dirección1", ciudad: "La Plata", salario: 2000.0, edad: 33 },
        &Persona { nombre: "Pablo", apellido: "Zappa", direccion: "Dirección1", ciudad: "La Plata", salario: 1000.0, edad: 30 }];

    assert_eq!(mejor_y_peor(&personas), vec_result);
}

#[test]
#[should_panic(expected = "Arreglo vacío!")]
fn test_panic_mejor_y_peor(){
    
    let personas = [];
    mejor_y_peor(&personas);

}