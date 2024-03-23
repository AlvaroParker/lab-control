namespace ServiceTypes {
    export interface Usuario {
        nombre: string;
        apellido_1: string;
        apellido_2: string;
        rut: string;
        print_path: string;
        correo_uai: string;
        rol: string;
        last_registro: Registro | null;
    }
    export interface Motivo {
        id: number;
        motivo: string;
    }
    export interface Registro {
        id: number;
        nombre: string;
        apellido_1: string;
        apellido_2: string;
        correo_uai: string;
        rut: string;
        fecha: string;
        salida: boolean;
        rol: string;
        motivo: string;
    }
    export interface Admin {
        nombre: string;
        apellido_1: string;
        apellido_2: string;
        email: string;
        token: string;
    }
    export interface Registro {
        id: number;
        fecha: string;
        motivo: string;
        rut: string;
        salida: boolean;
    }
    export interface AdminRegistro {
        nombre: string,
        apellido_1: string,
        apellido_2: string,
        email: string,
        pswd: string
    }
    export interface AdminGeneric {
        nombre: string,
        apellido_1: string,
        apellido_2: string,
        email: string
    }
    // Check if object has all the properties of Usuario (except for print_path)
    export const isUsuario = (input: any): input is Usuario => {
        const schema: Record<keyof Usuario, string> = {
            nombre: 'string',
            apellido_1: 'string',
            apellido_2: 'string',
            rut: 'string',
            print_path: 'string',
            correo_uai: 'string',
            rol: 'string',
            last_registro: 'string',
        };
        const missingProperties = Object.keys(schema)
            .filter(
                (key) =>
                    (input[key] === undefined || (input[key] as string).trim().length === 0) &&
                    key !== 'print_path' &&
                    key !== 'last_registro'
            )
            .map((key) => key as keyof Usuario);

        return missingProperties.length === 0;
    };
    // This is defined when deploying the server
    const IP = import.meta.env.VITE_BACKEND_IP as string;
    const PORT = import.meta.env.VITE_BACKEND_PORT as string;
    export const API_URL = `https://${IP}:${PORT}/api`;
    export const WS_URI = `wss://${IP}:${PORT}/api`;
}
export default ServiceTypes;
