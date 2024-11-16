pub struct Usuario {
    pub nombre: Nombre,
    pub contraseña: Contraseña,
}
impl std::fmt::Display for Usuario {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let texto_nombre = format!(
            "Soy {} {} {} {}",
            self.nombre.nombre, self.nombre.letra, self.nombre.apellido1, self.nombre.apellido2
        );
        let texto_alias = format!(", también conocido como {}", self.nombre.alias);
        if self.nombre.alias.is_empty() {
            write!(f, "{texto_nombre}")
        } else {
            write!(f, "{texto_nombre}{texto_alias}")
        }
    }
}

pub struct Nombre {
    pub nombre: String,
    pub letra: char,
    pub apellido1: String,
    pub apellido2: String,
    pub alias: String,
}
impl Default for Nombre {
    fn default() -> Self {
        Self {
            nombre: Default::default(),
            letra: Default::default(),
            apellido1: Default::default(),
            apellido2: Default::default(),
            alias: Default::default(),
        }
    }
}

pub struct Contraseña(String);
impl Contraseña {
    pub fn parse(contraseña_propuesta: &str) -> Result<Contraseña, &'static str> {
        let patron_de_validacion = regex::Regex::new("^*([a-z]?[0-9])*([a-z]?[A-Z])*$").unwrap();
        if contraseña_propuesta.len() >= 12 && patron_de_validacion.is_match(&contraseña_propuesta)
        {
            Ok(Contraseña(String::from(contraseña_propuesta)))
        } else {
            Err("La contraseña ha de ser de mínimo 12 caracteres y ha de contener alguna minúscula, alguna mayúscula y algún número.")
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub fn intentar_dejar_a_un_usuario_sin_contraseña() {
    if let Ok(una_contraseña) = Contraseña::parse("asdfasdfasdfdfasdfA5") {
        let un_usuario = Usuario {
            nombre: Nombre {
                nombre: String::from("Benzirpi"),
                apellido1: String::from("Mirvento"),
                ..Default::default()
            },
            contraseña: una_contraseña,
        };
        println!("{}", un_usuario);
    } else {
        println!("La contraseña no era válida.");
    }
}
