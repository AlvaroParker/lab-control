namespace ServiceTypes {
    export interface Usuario {
        nombre: string;
        apellido_1: string;
        apellido_2: string;
        rut: string;
        print_path: string;
        correo_uai: string;
        rol: string;
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
        };
        const missingProperties = Object.keys(schema)
            .filter(
                (key) =>
                    (input[key] === undefined || (input[key] as string).trim().length === 0) &&
                    key !== 'print_path'
            )
            .map((key) => key as keyof Usuario);

        return missingProperties.length === 0;
    };
    export const API_URL = 'http://192.168.68.112:3000';
}
export default ServiceTypes;
