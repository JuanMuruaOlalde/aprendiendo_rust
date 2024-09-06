pub fn pruebas_con_divisiones() {
    let resultado = dividir(5.0, 0.0);
    match resultado {
        Ok(resultado) => println!("El resultado de la división decimal es: {}", resultado),
        Err(mensaje) => println!(
            "Se ha producido un error en la división decimal: {}",
            mensaje
        ),
    }
    let resultado = dividir_enteros(5, 0);
    match resultado {
        Ok(resultado) => println!("El resultado de la división entera es: {}", resultado),
        Err(mensaje) => println!(
            "Se ha producido un error en la división entera: {}",
            mensaje
        ),
    }
}

fn dividir(a: f32, b: f32) -> Result<f32, String> {
    Ok(a / b)
}

fn dividir_enteros(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err(String::from("No se puede dividir por cero"));
    }
    Ok(a / b)
}
