use serde::{Deserialize, Serialize};

// The POST json body takes `EditPersona` parameters as arguments.
#[derive(Serialize, Deserialize)]
pub struct EditPersona {
    pub nombre: Option<String>,
    pub apellido_1: Option<String>,
    pub apellido_2: Option<String>,
    pub correo_uai: Option<String>,
    pub rut: Option<String>,
    pub rol: Option<String>,
}

// `[enroll.rs]`
// The result of the enrollment is a JSON, this struct is used to deserialize the JSON
// if result is true, the enrollment was successful and the path is the path to the fingerprint
#[derive(serde::Deserialize)]
pub struct Sucess<'a> {
    pub result: bool,
    pub path: Option<&'a str>,
}

// `[reroll.rs]`
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Rut {
    pub rut: String,
}

// NewPersona struct used to on `enroll.rs` to enroll a new Persona
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NewPersona {
    pub nombre: String,
    pub apellido_1: String,
    pub apellido_2: String,
    pub rut: String,
    pub correo_uai: String,
    pub is_disabled: bool,
    pub rol: String,
}

// We added the print_path so we a can then deserialize Outer and Serialize it again to an `personas::Model`
#[derive(Debug, Serialize)]
pub struct Outer<'a> {
    #[serde(flatten)]
    pub field_1: NewPersona,
    pub print_path: &'a str,
}

// Create a new Outer instance with a `NewPersona` struct and a `print_path` String
impl<'a> Outer<'a> {
    pub fn new(field_1: NewPersona, print_path: &'a str) -> Self {
        Self {
            field_1,
            print_path,
        }
    }
}
// This struct will be Deserialized into JSON and passed in the body of a socket request
// to the fingerprint sensor. It carries the fingerprint file path to check for a match.
#[derive(Serialize, Deserialize, Debug)]
pub struct Body {
    pub action: Action,
    pub paths: Vec<String>,
}

// Action, either `verify` to Verify a print or `Enroll` to enroll a new print
#[derive(Serialize, Deserialize, Debug)]
pub enum Action {
    #[serde(alias = "verify")]
    Verify,
    #[serde(alias = "enroll")]
    Enroll,
}
