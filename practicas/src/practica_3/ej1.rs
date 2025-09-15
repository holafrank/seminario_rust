/*
    1- Escribir un programa que defina una estructura Persona que tenga campos para el nombre, la edad
    y la direccion (que puede ser nulo al momento de la creacion de una persona). Para dicha estructura
    implemente los siguientes metodos:
        new: que pasando los parametros correspondientes crea una Persona y la retorna.
        imprimir: que imprime los datos de la persona sobre el mensaje ejecutado
        por ejemplo person.imprimir(), donde person es una variable del tipo Persona.
        obtener_edad: retorna la edad de la persona.
        actualizar_direccion(nueva_direccion)
*/
#[derive(Debug)]
    pub struct Persona{
        nombre: String,
        edad: u8,
        direccion: Option<String>,
    }
    impl Persona {
        fn new(nombre: String, edad: u8, dir_maybe: Option<String>) -> Persona {
            Persona{
                nombre,
                edad,
                direccion: dir_maybe,
            }
        }
        pub fn imprimir(&self){   //si no pongo & el metodo "consume al self" y lo vuelve su propiedad
            if let Some(dir) = &self.direccion {   //xq es un string, requiere &
                println!("{}, {}, {}", self.nombre, self.edad, dir);
            }else{
                println!("{}, {}, Sin direccion establecida.", self.nombre, self.edad);
            }
        }
        pub fn obtener_edad(&self) -> u8{
            return self.edad;
        }
        pub fn actualizar_direccion(&mut self, nueva_direccion:Option<String>){
            self.direccion = nueva_direccion;
        }
    }

    pub fn ejercicio1(){
        let pablo_grande = Persona::new("Pablo".to_string(), 38, Some("Calle 34 etre 117 y 118 numero 92 - La Plata".to_string()));
        let mut pablo_chiquito = Persona::new("Pablito".to_string(), 8, None);
        pablo_grande.imprimir();
        pablo_chiquito.imprimir();

        println!("La edad de Pablo es: {}", pablo_grande.obtener_edad());

        let nueva_direccion:String = "Callao 2890, Capital Federal".to_string();
        pablo_chiquito.actualizar_direccion(Some(nueva_direccion));
    }

    //Test ~ Ejercicio 1
    #[test]
    fn test_obtener_edad(){
        let pablo = Persona::new("Pablo".to_string(), 38, Some("Calle 34 etre 117 y 118 numero 92 - La Plata".to_string()));
        assert_eq!(pablo.obtener_edad(), 38);
    }

    #[test]
    fn test_actualizar_direccion(){
        let mut pablito = Persona::new("Pablito".to_string(), 8, None);
        assert_eq!(pablito.direccion, None);
        pablito.actualizar_direccion(Some("Callao 2890, Capital Federal".to_string()));
        assert_eq!(pablito.direccion, Some("Callao 2890, Capital Federal".to_string()));
    }