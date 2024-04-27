export enum Status {
    OK = 200,
    CREATED = 201,
    NO_CONTENT = 204,
    BAD_REQUEST = 400,
    UNAUTHORIZED = 401,
    FORBIDDEN = 403,
    NOT_FOUND = 404,
    CONFLICT = 409,
    INTERNAL_SERVER_ERROR = 500,
    UNKNOWN = 0,
}
export interface Usuario {
    nombre: string
    apellido_1: string
    apellido_2: string
    rut: string
    print_path: string
    correo_uai: string
    rol: string
    last_registro: Registro | null
}
export interface Motivo {
    id: number
    motivo: string
}
export interface Rol {
    id: number
    rol: string
}
export interface Registro {
    id: number
    nombre: string
    apellido_1: string
    apellido_2: string
    correo_uai: string
    rut: string
    fecha: string
    salida: boolean
    rol: string
    motivo: string
}
export interface Admin {
    nombre: string
    apellido_1: string
    apellido_2: string
    email: string
    token: string
}
export interface Registro {
    id: number
    fecha: string
    motivo: string
    rut: string
    salida: boolean
}
export interface AdminRegistro {
    nombre: string
    apellido_1: string
    apellido_2: string
    email: string
    pswd: string
}
export interface AdminGeneric {
    nombre: string
    apellido_1: string
    apellido_2: string
    email: string
}
// Check if object has all the properties of Usuario (except for print_path)
export const isUsuario = (input: Usuario): boolean => {
    if (
        input &&
        typeof input.nombre === 'string' &&
        typeof input.apellido_1 === 'string' &&
        typeof input.apellido_2 === 'string' &&
        typeof input.rut === 'string' &&
        typeof input.correo_uai === 'string' &&
        typeof input.rol === 'string' &&
        input.nombre &&
        input.apellido_1 &&
        input.apellido_2 &&
        input.rut &&
        input.correo_uai &&
        input.rol
    )
        return true

    return false
}
