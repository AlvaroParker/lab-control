use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};

// The json that we will respont with will have the following data:
#[derive(Serialize, Deserialize, Debug)]
pub struct RegistroAlumno {
    pub id: i32,
    pub nombre: String,
    pub apellido_1: String,
    pub apellido_2: String,
    pub rut: String,
    pub correo_uai: String,
    pub fecha: DateTimeWithTimeZone,
    pub salida: bool,
    pub rol: String,
    pub motivo: String,
}

// RegistroNew is the json body that we will receive when registering a new registro
#[derive(Serialize, Deserialize)]
pub struct RegistroNew {
    pub rut: String,
    pub salida: bool,
    pub motivo: String,
}
