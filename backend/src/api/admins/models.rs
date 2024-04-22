use serde::{Deserialize, Serialize};

// Struct representing the JSON that user will send to the webserver
#[derive(Serialize, Deserialize)]
pub struct RequestAdmin {
    pub nombre: String,
    pub apellido_1: String,
    pub apellido_2: String,
    pub email: String,
    pub pswd: String,
}
// Struct representing JSON that the server will response
#[derive(Serialize, Deserialize, Clone)]
pub struct ResponseAdmin {
    pub nombre: String,
    pub apellido_1: String,
    pub apellido_2: String,
    pub email: String,
    pub cookie: Option<String>,
}

// Login struct, the client must provide this field in order to login
#[derive(Serialize, Deserialize)]
pub struct LoginAdmin {
    pub email: String,
    pub pswd: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AdminReq {
    pub nombre: String,
    pub apellido_1: String,
    pub apellido_2: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Email {
    pub email: String,
}
