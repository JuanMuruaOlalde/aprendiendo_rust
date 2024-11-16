use base64::Engine;
use chrono::TimeZone;
use json::object;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    println!("===========================================================");
    let client_id = match std::env::var_os("CLIENT_ID") {
        Some(id) => id.into_string().unwrap(),
        None => panic!("Por favor, necesito la variable de entorno CLIENT_ID"),
    };
    println!("El client id es [{client_id}].");
    let client_secret = match std::env::var_os("CLIENT_SECRET") {
        Some(id) => id.into_string().unwrap(),
        None => panic!("Por favor, necesito la variable de entorno CLIENT_SECRET"),
    };
    println!("El client secret es [{client_secret}].");
    let personal_access_token = match std::env::var_os("PAT") {
        Some(id) => id.into_string().unwrap(),
        None => panic!("Por favor, necesito la variable de entorno PAT"),
    };
    println!("El PAT (Autodesk's Personal Access Token) es [{personal_access_token}].");
    println!("===========================================================");

    let cliente_http = reqwest::Client::new();

    let token_de_acceso_2ledged: String =
        match obtener_un_token_de_acceso_2ledged(&cliente_http, &client_id, &client_secret).await {
            Ok(token) => token,
            Err(error) => {
                println!("Problemas al intentar obtener el token de acceso: {error}");
                String::from("error-en-token")
            }
        };
    println!("Token de acceso [{token_de_acceso_2ledged}]");
    println!("===========================================================");

    let identificador_de_contexto: String = match obtener_informacion_del_contexto(
        &cliente_http,
        &token_de_acceso_2ledged,
        &personal_access_token,
    )
    .await
    {
        Ok(contexto) => {
            println!("Contexto (Teams):");
            println!("{}", contexto.pretty(4));
            println!("===========================================================");
            let mut identificador: String = String::from("");
            for team in contexto.members() {
                if team["alias"] == "poner aquí el nombre del Team que se desea" {
                    identificador = team["contextId"].to_string();
                }
            }
            identificador
        }
        Err(error) => {
            println!("Problemas al intentar obtener el contexto: {error:?}");
            String::from("")
        }
    };

    let identificador_de_consulta: String = match realizar_una_consulta_de_uso_de_licencias(
        &cliente_http,
        &token_de_acceso_2ledged,
        &personal_access_token,
        &identificador_de_contexto,
    )
    .await
    {
        Ok(id) => id,
        Err(error) => {
            println!(
                "Problemas al intentar obtener información acerca del uso de licencias: {error:?}"
            );
            String::from("")
        }
    };

    match recuperar_resultado_de_una_consulta(
        &cliente_http,
        &token_de_acceso_2ledged,
        &personal_access_token,
        &identificador_de_consulta,
    )
    .await
    {
        Ok(informacion) => {
            println!("Uso de licencias:");
            println!("{}", informacion.pretty(4));
            println!("===========================================================");
        }
        Err(error) => {
            println!(
                "Problemas al intentar recuperar resultado de la consulta [{identificador_de_consulta}]: {error:?}"
            );
        }
    };

    let fecha_inicio = chrono::Utc.with_ymd_and_hms(2024, 9, 1, 0, 0, 0).unwrap();
    let fecha_fin = chrono::Utc.with_ymd_and_hms(2024, 9, 15, 0, 0, 0).unwrap();
    match solicitar_un_nuevo_informe_de_uso_de_licencias(
        TiposDeInforme::DatosAgregados,
        fecha_inicio,
        fecha_fin,
        &cliente_http,
        &token_de_acceso_2ledged,
        &personal_access_token,
    )
    .await
    {
        Ok(informacion) => {
            println!("Respuesta a la petición del informe:");
            println!("[{}]", informacion.pretty(4));
            println!("===========================================================");
        }
        Err(error) => {
            println!(
                "Problemas al intentar solicitar un nuevo informe de uso de licencias: {error:?}"
            );
        }
    };

    match obtener_lista_de_informes_solicitados_recientemente(
        &cliente_http,
        &token_de_acceso_2ledged,
        &personal_access_token,
    )
    .await
    {
        Ok(informacion) => {
            println!("Informes que hay descargables en este momento en el servidor de Autodesk:");
            println!("[{}]", informacion.pretty(4));
            println!("===========================================================");
        }
        Err(error) => {
            println!(
                "Problemas al intentar obtener la lista de informes solicitados recientemente: {error:?}"
            );
        }
    };

    Ok(())
}

async fn obtener_un_token_de_acceso_2ledged(
    cliente_http: &reqwest::Client,
    client_id: &str,
    client_secret: &str,
) -> Result<String, reqwest::Error> {
    let credenciales =
        base64::prelude::BASE64_STANDARD.encode(format!("{client_id}:{client_secret}"));
    let respuesta = cliente_http
        .post("https://developer.api.autodesk.com/authentication/v2/token")
        .header("Accept", "application/json")
        .header("Authorization", format!("Basic {credenciales}"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&[
            ("grant_type", "client_credentials"),
            ("scope", "data:read data:write"),
        ])
        .send()
        .await?;
    //println!("peticion de token: {respuesta:?}");
    let texto = respuesta.text().await?;
    //println!("contenido completo de la respuesta a la petición de token: {texto}");
    let json = match json::parse(&texto) {
        Ok(json) => json,
        Err(error) => panic!("La respuesta a la petición de token no se puede convertir a json. Se ha producido el error [{error}] intentando convertir el texto recibido: [{texto}]."),
    };
    Ok(json["access_token"].to_string())
}

async fn obtener_informacion_del_contexto(
    cliente_http: &reqwest::Client,
    token_de_acceso: &str,
    token_personal: &str,
) -> Result<json::JsonValue, reqwest::Error> {
    let respuesta = cliente_http
        .get("https://developer.api.autodesk.com/insights/v1/contexts")
        .bearer_auth(token_de_acceso)
        .header("ADSK-PAT", token_personal)
        .header("Content-Type", "application/json")
        .send()
        .await?;
    //println!("peticion de contexto: {respuesta:?}");
    let texto = respuesta.text().await?;
    //println!("contenido completo de la respuesta a la petición de contexto: {texto}");
    let json = match json::parse(&texto) {
        Ok(json) => json,
        Err(error) => panic!("La respuesta a la petición de contexto no se puede convertir a json. Se ha producido el error [{error}] intentando convertir el texto [{texto}]."),
    };
    Ok(json)
}

async fn realizar_una_consulta_de_uso_de_licencias(
    cliente_http: &reqwest::Client,
    token_de_acceso: &str,
    token_personal: &str,
    identificador_de_contexto: &str,
) -> Result<String, reqwest::Error> {
    let parametros_para_la_peticion = object! {
        fields: ["productName", "version"],
        metrics: ["uniqueUsers", "totalUniqueDays", "latestUsageDate"],
        where: "",
        orderBy: "",
        contextId: identificador_de_contexto,
    };
    // println!(
    //     "==>> parametros para la petición: {}",
    //     parametros_para_la_peticion.dump()
    // );
    let peticion = cliente_http
        .post("https://developer.api.autodesk.com/insights/v1/usage-queries")
        .bearer_auth(token_de_acceso)
        .header("ADSK-PAT", token_personal)
        .header("Content-Type", "application/json")
        .body(parametros_para_la_peticion.dump());
    // println!("==>> peticion de una consulta de uso de licencias: {peticion:?}");
    let respuesta = peticion.send().await?;
    // println!("==>> respuesta: {respuesta:?}");
    let texto = respuesta.text().await?;
    let json = match json::parse(&texto) {
        Ok(json) => json,
        Err(error) => panic!("La respuesta a la petición de una consulta de uso de licencias no se puede convertir a json. Se ha producido el error [{error}] intentando convertir el texto [{texto}]."),
    };
    // println!(
    //     "==>> contenido completo de la respuesta a la petición de una consulta de uso de licencias: {}",
    //     json.pretty(4)
    // );
    Ok(json["id"].to_string())
}

async fn recuperar_resultado_de_una_consulta(
    cliente_http: &reqwest::Client,
    token_de_acceso: &str,
    token_personal: &str,
    identificador_de_consulta: &str,
) -> Result<json::JsonValue, reqwest::Error> {
    let peticion = cliente_http
        .get(format!("https://developer.api.autodesk.com/insights/v1/usage-queries/{identificador_de_consulta}"))
        .bearer_auth(token_de_acceso)
        .header("ADSK-PAT", token_personal)
        .header("Content-Type", "application/json");
    // println!("==>> peticion de recuperacion de resultado de una consulta: {peticion:?}");
    let respuesta = peticion.send().await?;
    // println!("==>> respuesta: {respuesta:?}");
    let texto = respuesta.text().await?;
    let json = match json::parse(&texto) {
        Ok(json) => json,
        Err(error) => panic!("La recuperacion del resultado de la consulta {identificador_de_consulta} no se puede convertir a json. Se ha producido el error [{error}] intentando convertir el texto [{texto}]."),
    };
    Ok(json)
}

async fn obtener_lista_de_informes_solicitados_recientemente(
    cliente_http: &reqwest::Client,
    token_de_acceso: &str,
    token_personal: &str,
) -> Result<json::JsonValue, reqwest::Error> {
    let respuesta = cliente_http
        .get("https://developer.api.autodesk.com/insights/v1/exports")
        .bearer_auth(token_de_acceso)
        .header("ADSK-PAT", token_personal)
        .send()
        .await?;
    //println!("peticion de lista de informes de uso de licencias: {respuesta:?}");
    let texto = respuesta.text().await?;
    let json = match json::parse(&texto) {
        Ok(json) => json,
        Err(error) => panic!("La respuesta a la petición de lista de informes de uso de licencias no se puede convertir a json. Se ha producido el error [{error}] intentando convertir el texto [{texto}]."),
    };
    Ok(json)
}

enum TiposDeInforme {
    DatosAgregados,
    DatosDetallados,
}

async fn solicitar_un_nuevo_informe_de_uso_de_licencias(
    tipo_de_informe: TiposDeInforme,
    fecha_inicio: chrono::DateTime<chrono::Utc>,
    fecha_fin: chrono::DateTime<chrono::Utc>,
    cliente_http: &reqwest::Client,
    token_de_acceso: &str,
    token_personal: &str,
) -> Result<json::JsonValue, reqwest::Error> {
    // ejemplo de formato válido para startDate o endDate: "2024-09-23T00:00:00.000Z"
    let fecha_inicio = fecha_inicio.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let fecha_fin = fecha_fin.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let mut parametros_para_la_peticion = json::JsonValue::new_object();
    match tipo_de_informe {
        TiposDeInforme::DatosAgregados => {
            parametros_para_la_peticion = object! {
                //teamId: identificador_de_contexto, //If not specified, the export will include data from all teams managed by the requesting user.
                outputFormat: "EXCEL",
                reports: ["USAGE_REPORT"],
                usageReports: ["USAGE_REPORT_BY_PRODUCT"],
                startDate: fecha_inicio ,
                endDate: fecha_fin,
                filters: {},
            }
        }
        TiposDeInforme::DatosDetallados => {
            parametros_para_la_peticion = object! {
                //teamId: identificador_de_contexto, //If not specified, the export will include data from all teams managed by the requesting user.
                outputFormat: "EXCEL",
                reports: ["USAGE"],
                startDate: "2024-09-01T00:00:00.000Z",
                endDate: "2024-09-23T00:00:00.000Z"
            }
        }
    }
    // println!(
    //     "parametros para la petición: {}",
    //     parametros_para_la_peticion.dump()
    // );
    let peticion = cliente_http
        .post("https://developer.api.autodesk.com/insights/v1/exports")
        .bearer_auth(token_de_acceso)
        .header("ADSK-PAT", token_personal)
        .body(parametros_para_la_peticion.dump());
    //println!("==>> peticion de un informe: {peticion:?}");
    let respuesta = peticion.send().await?;
    //println!("==>> respuesta: {respuesta:?}");
    let texto = respuesta.text().await?;
    let json = match json::parse(&texto) {
        Ok(json) => json,
        Err(error) => panic!("La respuesta a la petición de un informe no se puede convertir a json. Se ha producido el error [{error}] intentando convertir el texto [{texto}]."),
    };
    Ok(json)
}

async fn recuperar_estado_y_url_para_descarga_de_un_informe(
    cliente_http: &reqwest::Client,
    token_de_acceso: &str,
    token_personal: &str,
    identificador_de_informe: &str,
) -> Result<json::JsonValue, reqwest::Error> {
    let peticion = cliente_http
        .get(format!(
            "https://developer.api.autodesk.com/insights/v1/exports/{identificador_de_informe}"
        ))
        .bearer_auth(token_de_acceso)
        .header("ADSK-PAT", token_personal)
        .header("Content-Type", "application/json");
    //println!("==>> peticion de recuperacion del estado de un infome: {peticion:?}");
    let respuesta = peticion.send().await?;
    //println!("==>> respuesta: {respuesta:?}");
    let texto = respuesta.text().await?;
    let json = match json::parse(&texto) {
        Ok(json) => json,
        Err(error) => panic!("La recuperacion del estado del informe [{identificador_de_informe}] no se puede convertir a json. Se ha producido el error [{error}] intentando convertir el texto [{texto}]."),
    };
    Ok(json)
}
