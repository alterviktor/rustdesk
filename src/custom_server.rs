use hbb_common::{
    bail,
    base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _},
    sodiumoxide::crypto::sign,
    ResultType,
};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, Clone)]
pub struct CustomServer {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub api: String,
    #[serde(default)]
    pub relay: String,
}

fn get_custom_server_from_config_string(_s: &str) -> ResultType<CustomServer> {
    // Vždy vrátíme natvrdo definovaný server, ignorujeme vstup
    Ok(CustomServer {
        host: "podpora.moravec.eu".to_owned(),      // ZDE NAPIŠTE SVOU DOMÉNU / IP SYNOLOGY
        key: "POIclmEVbykh6szaJV9RqsAj1ueA+OJxvWrc4KF1Wtc=".to_owned(),             // ZDE VLOŽTE OBSAH VAŠEHO .pub KLÍČE
        api: "".to_owned(),
        relay: "podpora.moravec.eu".to_owned(),
    })
}

pub fn get_custom_server_from_string(_s: &str) -> ResultType<CustomServer> {
    // Kompletně obcházíme parsování názvu souboru.
    // Ať už uživatel spustí soubor s jakýmkoliv názvem, aplikace dostane vaši konfiguraci.
    Ok(CustomServer {
        host: "podpora.moravec.eu".to_owned(),      // ZDE NAPIŠTE SVOU DOMÉNU / IP SYNOLOGY
        key: "POIclmEVbykh6szaJV9RqsAj1ueA+OJxvWrc4KF1Wtc=".to_owned(),             // ZDE VLOŽTE OBSAH VAŠEHO .pub KLÍČE
        api: "".to_owned(),
        relay: "podpora.moravec.eu".to_owned(),
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_filename_license_string() {
        // Testy upraveny tak, aby procházely s natvrdo zadaným serverem
        let res = get_custom_server_from_string("any_filename.exe").unwrap();
        assert_eq!(res.host, "vaseserver.cz".to_owned());
    }
}
