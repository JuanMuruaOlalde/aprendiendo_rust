use chrono::NaiveDate;
use regex::Regex;
use std::fmt;
use std::path::Path;
use std::{
    fs,
    io::{BufRead, BufReader},
};

pub struct Persona {
    nombre: String,
    apellidos: String,
    nacimiento: NaiveDate,
    peso_kg: f32,
    altura_cm: f32,
    fecha_medicion: NaiveDate,
}
impl fmt::Display for Persona {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {}\nFecha de nacimiento: {}\nAltura: {} cm\nPeso: {} kg (medido el {})",
            self.nombre,
            self.apellidos,
            self.nacimiento,
            self.altura_cm,
            self.peso_kg,
            self.fecha_medicion
        )
    }
}

pub fn leer_datos_del_archivo<T: AsRef<Path>>(path_archivo: T) -> Vec<Persona> {
    let mut personas: Vec<Persona> = Vec::new();
    match fs::File::open(path_archivo.as_ref()) {
        Err(error) => {
            println!(
                "ERROR al abrir el archivo {}: {}",
                path_archivo.as_ref().display(),
                error
            );
        }
        Ok(archivo) => {
            let lector = BufReader::new(archivo);
            for (num_linea, linea) in lector.lines().into_iter().enumerate() {
                match linea {
                    Err(error) => {
                        println!("ERROR al leer la linea [{}]: {}", num_linea, error);
                    }
                    Ok(linea) => match num_linea {
                        0 => {
                            println!("==============================================================================");
                            println!("Se ignora la primera linea de cabecera:");
                            println!("{}", linea);
                            println!("==============================================================================");
                        }
                        _ => match procesar_linea(&linea) {
                            Err(error) => {
                                println!("ERROR al procesar la linea [{}]: {}", num_linea, error);
                            }
                            Ok(persona) => {
                                personas.push(persona);
                            }
                        },
                    },
                }
            }
        }
    }
    personas
}

fn procesar_linea(linea: &str) -> Result<Persona, String> {
    let separador = Regex::new(r"\s+:\s+").unwrap();
    let datos: Vec<&str> = separador.split(&linea).collect();
    if datos.len() == 6 {
        let nombre = String::from(datos[0]);
        let apellidos = String::from(datos[1]);
        let nacimiento = match NaiveDate::parse_from_str(datos[2], "%d/%m/%Y") {
            Ok(fecha) => fecha,
            Err(error) => {
                return Err(std::fmt::format(format_args!(
                    "Fallo al procesar la fecha de nacimiento [{}]: {}",
                    datos[2], error
                )))
            }
        };
        let peso_kg: f32 = match datos[3].parse() {
            Ok(peso) => peso,
            Err(error) => {
                return Err(std::fmt::format(format_args!(
                    "Fallo al procesar el peso [{}]: {}",
                    datos[3], error
                )))
            }
        };
        let altura_cm: f32 = match datos[4].parse() {
            Ok(altura) => altura,
            Err(error) => {
                return Err(std::fmt::format(format_args!(
                    "Fallo al procesar la altura [{}]: {}",
                    datos[4], error
                )))
            }
        };
        let fecha_medicion = match NaiveDate::parse_from_str(datos[5], "%d/%m/%Y") {
            Ok(fecha) => fecha,
            Err(error) => {
                return Err(std::fmt::format(format_args!(
                    "Fallo al procesar la fecha de la medición [{}]: {}",
                    datos[5], error
                )))
            }
        };
        Ok(Persona {
            nombre,
            apellidos,
            nacimiento,
            peso_kg,
            altura_cm,
            fecha_medicion,
        })
    } else {
        return Err(std::fmt::format(format_args!(
            "Se esperaban 6 campos separados por ' : '; 
             pero en esta linea se han encontrado {} campos:\n[{}]",
            datos.len(),
            linea,
        )));
    }
}

#[cfg(test)]
#[test]
fn devuelve_una_lista_vacia_si_no_puede_abrir_el_archivo_a_leer() {
    let path_archivo = "./resources/archivo_que_no_existe.txt";
    let resultado = leer_datos_del_archivo(path_archivo);
    assert!(resultado.len() == 0);
}
#[test]
fn avisa_si_una_linea_tiene_menos_de_6_campos() {
    let linea = "Benzirpi : Mirvento : 01/01/2000 : 70 : 170";
    let resultado = procesar_linea(linea);
    assert!(resultado.is_err());
}
#[test]
fn avisa_si_una_linea_tiene_mas_de_6_campos() {
    let linea = "Benzirpi : Mirvento : 01/01/2000 : 70 : 170 : 01/01/2020 : 80";
    let resultado = procesar_linea(linea);
    assert!(resultado.is_err());
}
#[test]
fn avisa_si_una_linea_esta_vacia() {
    let linea = "";
    let resultado = procesar_linea(linea);
    assert!(resultado.is_err());
}
#[test]
fn avisa_si_falla_al_procesar_fecha_de_nacimiento() {
    let linea = "Benzirpi : Mirvento : 01/01 : 70 : 170 : 01/01/2020";
    let resultado = procesar_linea(linea);
    assert!(resultado.is_err());
}
#[test]
fn avisa_si_falla_al_procesar_peso() {
    let linea = "Benzirpi : Mirvento : 01/01/2000 : 7pes0 : 170 : 01/01/2020";
    let resultado = procesar_linea(linea);
    assert!(resultado.is_err());
}
#[test]
fn avisa_si_falla_al_procesar_altura() {
    let linea = "Benzirpi : Mirvento : 01/01/2000 : 70 : 1altur0 : 01/01/2020";
    let resultado = procesar_linea(linea);
    assert!(resultado.is_err());
}
#[test]
fn avisa_si_falla_al_procesar_fecha_de_medicion() {
    let linea = "Benzirpi : Mirvento : 01/01/2000 : 70 : 170 : 01/01";
    let resultado = procesar_linea(linea);
    assert!(resultado.is_err());
}
