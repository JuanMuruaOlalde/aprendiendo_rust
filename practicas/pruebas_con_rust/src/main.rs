mod archivo_de_texto_plano;
mod divisiones;

fn main() {
    println!();
    println!("==============================================================================");

    let path_archivo = String::from("./resources/archivo_para_pruebas.txt");
    let personas = archivo_de_texto_plano::leer_datos_del_archivo(&path_archivo);

    println!("==============================================================================");
    println!("Las personas que se han leido son:");
    println!("----------------------------------");
    for persona in personas {
        println!("{}", persona);
        println!("--------------------------------------");
    }
    println!("==============================================================================");
}
