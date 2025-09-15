/*
    3- Escribir un programa que defina una estructura Fecha que tenga campos para
    el día, el mes y el año. Para dicha estructura implemente los siguientes métodos:
        ➢ new: que pasando los parámetros correspondientes, crea una Fecha y la retorna.
        ➢ es_fecha_valida: retorna true si es una fecha valida, false caso contrario.
        tenga en cuenta los años bisiestos también.
        ➢ es_bisiesto: retorna true si el año de la fecha pertenece a un año bisiesto.
        ➢ sumar_dias(dias): suma la cantidad de días a la fecha, modificándose
        ➢ restar_dias(dias): resta la cantidad de días a la fecha, modificándose
        ➢ es_mayor(una_fecha): que retorna true si la fecha que recibe el mensaje es
        mayor a la fecha pasada por parámetro..

        crate recomendada https://crates.io/crates/date_time
        Yo acá hice todo sin valerme de ella.
*/

    #[derive(Debug)]
    pub enum Año{
        EsBisiesto(u16),
        NoBisiesto(u16),
    }
    impl Año{
        pub fn new(año:u16) -> Año {
            if año %4 !=0 {
                return Año::NoBisiesto(año);
            }else{
                if año %4 == 0 && año % 100 != 0{
                    return Año::EsBisiesto(año);
                }else{
                    if año % 4 == 0 && año % 100 == 0 && año % 400 != 0{
                        return Año::NoBisiesto(año);
                    }else{
                        if año % 4 == 0 && año % 100 == 0 && año % 400 == 0 {
                            return Año::EsBisiesto(año);
                        }else{
                            return Año::NoBisiesto(año);
                        } 
                    }
                }
            }
        }
        pub fn actual_year(&self) -> u16 {
            match self{
                Año::EsBisiesto(valor) => return  *valor,
                Año::NoBisiesto(valor) => return *valor,
            }
        }
        pub fn año_valido(&self) -> bool {
            return self.actual_year() > 0 && self.actual_year() <= 3000;
        }
        pub fn es_bisiesto(&self) -> bool{
            match self {
                Año::EsBisiesto(valor) => return true,
                Año::NoBisiesto(valor) => return false,
            }
        }

    }

    #[derive(Debug)]
    pub enum Mes{
        TerminaEn31(u8),        //enero 1, marzo 3, mayo 5, julio 7, agosto 8, octubre 10 y diciembre 12
        TerminaEn30(u8),        //abril 4, junio 6, septiembre 9 y noviembre 11.
        FebreroBisiesto(u8),
        Febrero(u8),
    }
    impl Mes{
        pub fn new(mes_num:u8, bisiesto:bool) -> Mes {
            match mes_num{
                1 => Mes::TerminaEn31(1),
                2 => {
                    if bisiesto{
                        Mes::FebreroBisiesto(2)
                    }else{
                        Mes::Febrero(2)
                    }
                },
                3 => Mes::TerminaEn31(3),
                4 => Mes::TerminaEn30(4),
                5 => Mes::TerminaEn31(5),
                6 => Mes::TerminaEn30(6),
                7 => Mes::TerminaEn31(7),
                8 => Mes::TerminaEn31(8),
                9 => Mes::TerminaEn30(9),
                10 => Mes::TerminaEn31(10),
                11 => Mes::TerminaEn30(11),
                12 => Mes::TerminaEn31(12),
                _ => Mes::TerminaEn30(0),
            }
        }
        pub fn actual_month(&self) -> u8 {
            match self{
                Mes::TerminaEn31(valor) => return  *valor,
                Mes::TerminaEn30(valor) => return *valor,
                Mes::Febrero(valor) => return *valor,
                Mes::FebreroBisiesto(valor) => *valor,
            }
        }
        pub fn mes_valido(&self) -> bool {
            return self.actual_month() > 0 && self.actual_month() <= 12;
        }    
        pub fn supera_limite(&self, dia:u8) -> bool {
            match self {
                Mes::Febrero(valor) => if dia < 28 { 
                    return false
                }else{
                    return true; //si llega la fecha 28 de febrero, para aumentar un dia primero hay que aumentar el mes
                },
                Mes::FebreroBisiesto(valor) => if dia < 29 {
                    return false
                }else{
                    return true;
                },
                Mes::TerminaEn31(valor) => if dia < 31 {
                    return false
                }else{
                    return true;
                },
                Mes::TerminaEn30(valor) => if dia < 30 {
                    return false
                }else{
                    return true;
                },
            }
        }

    }

    #[derive(Debug)]
    pub struct Fecha {
        dia: u8,
        mes: Mes,
        año: Año, 
    }
    impl Fecha{
        pub fn new (dia: u8, mes_num: u8, año_num: u16) -> Fecha {      //Recibe numeros, no enums
            let año:Año = Año::new(año_num);
            if año.es_bisiesto(){
                let mes: Mes = Mes::new(mes_num, true);
                Fecha {
                    dia, 
                    mes,
                    año,
                }
            }else{
                let mes: Mes = Mes::new(mes_num, false);
                Fecha {
                    dia, 
                    mes,
                    año,
                }
            }

        }    
        pub fn dia_valido(&self, bisiesto:bool) -> bool {
            match bisiesto{
                true => {
                    match self.mes{
                        Mes::Febrero(valor) => return false, //no puede ser febrero normal en año bisiesto
                        Mes::FebreroBisiesto(valor) => return self.dia >0 && self.dia < 30,
                        Mes::TerminaEn31(valor) => return  self.dia > 0 && self.dia < 32,
                        Mes::TerminaEn30(valor) => return self.dia > 0 && self.dia < 31,
                    }
                }
                false => {
                    match self.mes{
                        Mes::FebreroBisiesto(valor) => return false, //no puede ser febrero bisiesto en año normal
                        Mes::Febrero(valor) => return self.dia >0 && self.dia < 29,
                        Mes::TerminaEn31(valor) => return  self.dia > 0 && self.dia < 32,
                        Mes::TerminaEn30(valor) => return self.dia > 0 && self.dia < 31,                   
                    }
                }
            }
        }
        pub fn es_fecha_valida (&self) -> bool {
            if self.año.año_valido() && self.mes.mes_valido(){
                return self.dia_valido(self.año.es_bisiesto());
            }else{
                return false;
            }
        }
        pub fn siguiente_mes(&mut self, bisiesto:bool) -> bool{
            match self.mes {
                Mes::TerminaEn30(1) =>
                if bisiesto{
                    self.mes = Mes::FebreroBisiesto(2);
                }else{
                    self.mes = Mes::Febrero(2);
                },
                Mes::Febrero(2) => self.mes = Mes::TerminaEn31(3),
                Mes::FebreroBisiesto(2) => self.mes = Mes::TerminaEn31(3),
                Mes::TerminaEn31(3) => self.mes = Mes::TerminaEn30(4),
                Mes::TerminaEn30(4) => self.mes = Mes::TerminaEn31(5),
                Mes::TerminaEn31(5) => self.mes = Mes::TerminaEn30(6),
                Mes::TerminaEn30(6) => self.mes = Mes::TerminaEn31(7),
                Mes::TerminaEn31(7) => self.mes = Mes::TerminaEn31(8),
                Mes::TerminaEn31(8) => self.mes = Mes::TerminaEn30(9),
                Mes::TerminaEn30(9) => self.mes = Mes::TerminaEn31(10),
                Mes::TerminaEn31(10) => self.mes = Mes::TerminaEn30(11),
                Mes::TerminaEn31(11) => self.mes = Mes::TerminaEn31(12),
                Mes::TerminaEn31(12) => {
                self.mes = Mes::TerminaEn31(1);
                return true;
                },
                _ => return false,
            }
            return false;
        }
        pub fn siguiente_año(&mut self){
            let año_siguiente:u16 = self.año.actual_year()+1;
            if año_siguiente %4 !=0 {
                self.año = Año::NoBisiesto(año_siguiente);
            }else{
                if año_siguiente %4 == 0 && año_siguiente % 100 != 0{
                    self.año = Año::EsBisiesto(año_siguiente);
                }else{
                    if año_siguiente % 4 == 0 && año_siguiente % 100 == 0 && año_siguiente % 400 != 0{
                        self.año = Año::NoBisiesto(año_siguiente);
                    }else{
                        if año_siguiente % 4 == 0 && año_siguiente % 100 == 0 && año_siguiente % 400 == 0 {
                            self.año = Año::EsBisiesto(año_siguiente);
                        }else{
                            self.año = Año::NoBisiesto(año_siguiente);
                        } 
                    }
                }
            }

        }
        pub fn sumar_dias(&mut self, dias:u8) -> Fecha{
            let mut dias_cont:u8 = dias;
            while dias_cont > 0 {
                if self.mes.supera_limite(self.dia){
                    if self.siguiente_mes(self.año.es_bisiesto()){ //si el mes cambia de dic a ene cambia de año
                        self.siguiente_año()
                    }
                    self.dia=1;
                    dias_cont = dias_cont -1;
                }else{
                    self.dia = self.dia+1;
                    dias_cont = dias_cont -1;
                }
            }
            return self.devolver_fecha();
        }
        pub fn devolver_fecha(&self) -> Fecha {
            let dia:u8= self.dia;
            let mes:u8= self.mes.actual_month();
            let año:u16= self.año.actual_year();
            return Fecha::new(dia, mes, año);
        }
        pub fn año_anterior(&mut self){
            let año_anterior:u16 = self.año.actual_year()-1;
            if año_anterior %4 !=0 {
                self.año = Año::NoBisiesto(año_anterior);
            }else{
                if año_anterior %4 == 0 && año_anterior % 100 != 0{
                    self.año = Año::EsBisiesto(año_anterior);
                }else{
                    if año_anterior % 4 == 0 && año_anterior % 100 == 0 && año_anterior % 400 != 0{
                        self.año = Año::NoBisiesto(año_anterior);
                    }else{
                        if año_anterior % 4 == 0 && año_anterior % 100 == 0 && año_anterior % 400 == 0 {
                            self.año = Año::EsBisiesto(año_anterior);
                        }else{
                            self.año = Año::NoBisiesto(año_anterior);
                        } 
                    }
                }
            }
        }
        pub fn mes_anterior(&mut self, bisiesto:bool) -> bool{
            match self.mes {
                Mes::TerminaEn31(1) => {self.mes = Mes::TerminaEn31(12);
                return true;
                },
                Mes::Febrero(2) => self.mes = Mes::TerminaEn31(1),
                Mes::FebreroBisiesto(2) => self.mes = Mes::TerminaEn30(1),
                Mes::TerminaEn31(3) => {
                    if bisiesto{
                        self.mes = Mes::FebreroBisiesto(2);
                    }else{
                        self.mes = Mes::Febrero(2);
                    }
                },
                Mes::TerminaEn30(4) => self.mes = Mes::TerminaEn31(3),
                Mes::TerminaEn31(5) => self.mes = Mes::TerminaEn30(4),
                Mes::TerminaEn30(6) => self.mes = Mes::TerminaEn31(5),
                Mes::TerminaEn31(7) => self.mes = Mes::TerminaEn30(6),
                Mes::TerminaEn31(8) => self.mes = Mes::TerminaEn31(7),
                Mes::TerminaEn30(9) => self.mes = Mes::TerminaEn31(8),
                Mes::TerminaEn31(10) => self.mes = Mes::TerminaEn30(9),
                Mes::TerminaEn31(11) => self.mes = Mes::TerminaEn31(10),
                Mes::TerminaEn31(12) => self.mes = Mes::TerminaEn31(11),
                _ => return false,
            }
            return false;
        }           
        pub fn restar_dias(&mut self, dias:u8) {
            let mut dias_cont:u8 = dias;
            while dias_cont > 0 {
                if self.dia == 1{
                    if self.mes_anterior(self.año.es_bisiesto()){ //si el mes cambia de dic a ene cambia de año
                        self.año_anterior();
                    }
                    match self.mes {
                        Mes::Febrero(x) => self.dia = 28,
                        Mes::FebreroBisiesto(x) => self.dia = 29,
                        Mes::TerminaEn30(x) => self.dia = 30,
                        Mes::TerminaEn31(x) => self.dia = 31,
                    }
                    dias_cont = dias_cont -1;
                }else{
                    self.dia = self.dia-1;
                    dias_cont = dias_cont -1;
                }
            }
        }
        pub fn es_mayor(&self, una_fecha:&Fecha) -> bool {
            if self.año.actual_year() > una_fecha.año.actual_year(){
                return true;
            }else{
                if self.mes.actual_month() > una_fecha.mes.actual_month(){
                    return true;
                }else{
                    if self.dia > una_fecha.dia{
                        return true;
                    }else{
                        return false;
                    }
                }    
            }
        }              
        pub fn es_bisiesto(&self) -> bool {
            return self.año.es_bisiesto();
        }
        pub fn imprimir_fecha(&self) -> String {
            let año:String= self.año.actual_year().to_string();
            let mes:String= self.mes.actual_month().to_string();
            let dia:String= self.dia.to_string();
            let fecha:String= format!("{}/{}/{}", dia, mes, año);
            return fecha;
        }
    }
                
    pub fn ejercicio3(){
        let año:u16= 1999;
        let mes:u8=3;
        let dia:u8=15;     

        let mut fecha_1: Fecha = Fecha::new(dia, mes, año);

        println!("La fecha es: {:?}", fecha_1.imprimir_fecha());
        println!("Es fecha valida? --> {}", fecha_1.es_fecha_valida());
        println!("Es bisiesto? --> {}", fecha_1.es_bisiesto()); 

        let dias_sumar:u8 = 3;
        fecha_1.sumar_dias(dias_sumar);
        println!("Sumar {} dias --> {:?}", dias_sumar, fecha_1.imprimir_fecha()); 

        let dias_restar:u8 = 4;
        fecha_1.restar_dias(dias_restar);
        println!("Restar {} dias --> {:?}", dias_restar, fecha_1.imprimir_fecha()); 

        let fecha_2: Fecha = Fecha::new(27, 12, 2002);
        println!("{:?} es mayor que {:?} ? ---> {}", fecha_1.imprimir_fecha(), fecha_2.imprimir_fecha(), fecha_1.es_mayor(&fecha_2));

    }

    #[test]
    fn test_es_fecha_valida(){
        let fecha_valida: Fecha = Fecha::new(10, 10, 2010);
        let fecha_invalida_1: Fecha = Fecha::new(29, 2, 2010);
        let fecha_invalida_2: Fecha = Fecha::new(29, 13, 2010);
        let fecha_invalida_3: Fecha = Fecha::new(31, 6, 2010);

        assert_eq!(fecha_invalida_1.es_fecha_valida(), false);
        assert_eq!(fecha_invalida_2.es_fecha_valida(), false);
        assert_eq!(fecha_invalida_3.es_fecha_valida(), false);
        assert_eq!(fecha_valida.es_fecha_valida(), true);
    }

    #[test]
    fn test_es_bisiesto(){
        let fecha_bisiesta_1: Fecha = Fecha::new(10, 10, 1920);
        let fecha_bisiesta_2: Fecha = Fecha::new(10, 10, 2000);
        let fecha_no_bisiesta_1: Fecha = Fecha::new(10, 10, 1900);
        let fecha_no_bisiesta_2: Fecha = Fecha::new(10, 10, 2007);

        assert_eq!(fecha_bisiesta_1.es_bisiesto(), true);
        assert_eq!(fecha_bisiesta_2.es_bisiesto(), true);
        assert_eq!(fecha_no_bisiesta_1.es_bisiesto(), false);
        assert_eq!(fecha_no_bisiesta_2.es_bisiesto(), false);
    
    }

    #[test]
    fn test_sumar_dias(){

        let mut fecha_1: Fecha = Fecha::new(30, 12, 2003);
        fecha_1.sumar_dias(3);
        let fecha_result:Fecha = Fecha::new(2, 1, 2004);

        assert_eq!(fecha_1.dia, fecha_result.dia);
        assert_eq!(fecha_1.mes.actual_month(), fecha_result.mes.actual_month());
        assert_eq!(fecha_1.año.actual_year(), fecha_result.año.actual_year());

        assert_eq!(fecha_1.es_bisiesto(), true);

    }

    #[test]
    fn test_restar_dias(){

        let mut fecha_1: Fecha = Fecha::new(1, 1, 2005);
        fecha_1.restar_dias(1);
        let fecha_result:Fecha = Fecha::new(31, 12, 2004);
        
        assert_eq!(fecha_1.dia, fecha_result.dia);
        assert_eq!(fecha_1.mes.actual_month(), fecha_result.mes.actual_month());
        assert_eq!(fecha_1.año.actual_year(), fecha_result.año.actual_year());

        assert_eq!(fecha_1.es_bisiesto(), true);

    }

    #[test]
    fn test_es_mayor(){
        let fecha_2005: Fecha = Fecha::new(1, 1, 2005);
        let fecha_2015: Fecha = Fecha::new(1, 1, 2015);

        assert_eq!(fecha_2005.es_mayor(&fecha_2015), false);

        let fecha_1: Fecha = Fecha::new(1, 1, 2025);
        let fecha_2: Fecha = Fecha::new(2, 1, 2025);
        assert_eq!(fecha_2.es_mayor(&fecha_1), true);
    }