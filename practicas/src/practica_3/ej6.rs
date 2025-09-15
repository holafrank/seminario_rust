/*  6- Escribir un programa que defina una estructura Estudiante que tenga campos para el
    nombre, el número de identificación y las calificaciones de exámenes. De cada Examen se
    conoce el nombre de la materia y la nota. Para dichas estructuras implemente los siguientes
    métodos:   
    ❖Examen:
    ➢ new: que pasando los parámetros correspondientes, crea un Examen y lo retorna.
    ❖Estudiante:
    ➢ new: que pasando los parámetros correspondientes, crea un Estudiante y lo retorna.
    ➢ obtener_promedio: retorna el promedio de las notas.
    ➢ obtener_calificacion_mas_alta: retorna la nota más alta.
    ➢ obtener_calificacion_mas_baja: retorna la nota más baja.

    Nota: Tenga en cuenta que el Estudiante puede tener entre 0 y n notas de examen.
*/
struct Examen{
    materia: String,
    nota: u8,
}
impl Examen{
    pub fn new(materia:String, nota:u8) -> Examen{
        Examen{
            materia,
            nota,
        }
    }
    pub fn get_nota(&self) -> u8 {
        return self.nota;
    }
}
struct Estudiante {
    nombre: String,
    legajo: u16,
    calificaciones: Vec<Examen>,
}
impl Estudiante{
    pub fn new(nombre: String, legajo: u16, calificaciones: Vec<Examen>,) -> Estudiante{
        Estudiante{
            nombre,
            legajo,
            calificaciones,
        }
    }
    pub fn obtener_promedio(&self) -> Option<u8> {
        if self.calificaciones.is_empty(){
            return None;
        }else{
            let mut total:u8=0;
            for examen in &self.calificaciones {
                total += examen.get_nota();
            }
            return Some(total / self.calificaciones.len() as u8);
        }
    }
    pub fn obtener_calificacion_mas_alta(&self) -> Option<u8> {
        if self.calificaciones.is_empty(){
            return None;
        }else{
            let mut max:u8=0;
            for examen in &self.calificaciones {
                if examen.get_nota() > max{
                    max = examen.get_nota();
                }
            }
            return Some(max);
        }
    }
    pub fn obtener_calificacion_mas_baja(&self) -> Option<u8> {
        if self.calificaciones.is_empty(){
            return None;
        }else{
            let mut min:u8=10;
            for examen in &self.calificaciones {
                if examen.get_nota() < min{
                    min = examen.get_nota();
                }
            }
            return Some(min);
        }
    }
}
pub fn ejercicio_6(){
    let mut examenes:Vec<Examen> = Vec::new();
    let examen_a:Examen = Examen::new("Servicio de concha".to_string(), 9);
    let examen_b:Examen = Examen::new("Devoración de toda".to_string(), 10);
    let examen_c:Examen = Examen::new("Historia Queer".to_string(), 8);
    println!("{}",examen_b.materia);
    examenes.push(examen_a); examenes.push(examen_b); examenes.push(examen_c);
    let sixsex: Estudiante = Estudiante::new("SixSex".to_string(), 6969, examenes);
    println!("{:?},{:?}", sixsex.nombre, sixsex.legajo);
    println!("{:?},{:?},{:?}", sixsex.obtener_promedio(), sixsex.obtener_calificacion_mas_alta(), sixsex.obtener_calificacion_mas_baja());
}

#[test]
fn test_obtener_promedio(){
    let mut examenes:Vec<Examen> = Vec::new();
    let examen_a:Examen = Examen::new("Servicio de concha".to_string(), 9);
    let examen_b:Examen = Examen::new("Devoración de toda".to_string(), 10);
    let examen_c:Examen = Examen::new("Historia Queer".to_string(), 8);
    examenes.push(examen_a); examenes.push(examen_b); examenes.push(examen_c);
    let est: Estudiante = Estudiante::new("SixSex".to_string(), 6969, examenes);
   
    assert_eq!(est.obtener_promedio(), Some(9));
}
#[test]
fn test_obtener_calificacion_mas_alta(){
    let mut examenes:Vec<Examen> = Vec::new();
    let examen_a:Examen = Examen::new("Servicio de concha".to_string(), 9);
    let examen_b:Examen = Examen::new("Devoración de toda".to_string(), 10);
    let examen_c:Examen = Examen::new("Historia Queer".to_string(), 8);
    examenes.push(examen_a); examenes.push(examen_b); examenes.push(examen_c);
    let est: Estudiante = Estudiante::new("SixSex".to_string(), 6969, examenes);
   
    assert_eq!(est.obtener_calificacion_mas_alta(), Some(10));
}
#[test]
fn test_obtener_calificacion_mas_baja(){
    let mut examenes:Vec<Examen> = Vec::new();
    let examen_a:Examen = Examen::new("Servicio de concha".to_string(), 9);
    let examen_b:Examen = Examen::new("Devoración de toda".to_string(), 10);
    let examen_c:Examen = Examen::new("Historia Queer".to_string(), 8);
    examenes.push(examen_a); examenes.push(examen_b); examenes.push(examen_c);
    let est: Estudiante = Estudiante::new("SixSex".to_string(), 6969, examenes);
   
    assert_eq!(est.obtener_calificacion_mas_baja(), Some(8));
}
#[test]
fn test_obtener_promedio_sin_examenes(){
    let est: Estudiante = Estudiante::new("SixSex".to_string(), 6969, Vec::new());
    assert_eq!(est.obtener_promedio(), None);
}
#[test]
fn test_obtener_calificacion_mas_alta_sin_examenes(){
    let est: Estudiante = Estudiante::new("SixSex".to_string(), 6969, Vec::new());
    assert_eq!(est.obtener_calificacion_mas_alta(), None);
}
#[test]
fn test_obtener_calificacion_mas_baja_sin_examenes(){
    let est: Estudiante = Estudiante::new("SixSex".to_string(), 6969, Vec::new());
    assert_eq!(est.obtener_calificacion_mas_baja(), None);
}