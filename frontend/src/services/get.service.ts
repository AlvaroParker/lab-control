import ServiceTypes, { Status } from './types';
import axios, { isAxiosError } from 'axios';

axios.defaults.withCredentials = true;

export const getUsuarios = async (): Promise<[Array<ServiceTypes.Usuario>, Status]> => {
    try {
        const res = await axios.get(ServiceTypes.API_URL + '/usuarios');
        switch (res.status) {
            case 200:
                return [res.data, Status.OK];
            case 401:
                return [[], Status.UNAUTHORIZED];
            case 500:
                return [[], Status.INTERNAL_SERVER_ERROR];
            default:
                return [[], Status.UNKNOWN];
        
        }
    } catch (err) {
        if (isAxiosError(err)) {
            switch (err.response?.status) {
                case 401:
                    return [[], Status.UNAUTHORIZED];
                case 500:
                    return [[], Status.INTERNAL_SERVER_ERROR];
                case 404:
                    return [[], Status.NOT_FOUND];
                default:
                    return [[], Status.UNKNOWN];
            }
        }
    }
    return [[], Status.UNKNOWN];
};

export const getUsuarioByRut = async (rut: string): Promise<[ServiceTypes.Usuario | null, Status]> => {
    try {
        const res = await axios.get(ServiceTypes.API_URL + `/usuarios/${rut}`);
        switch (res.status) {
            case 200:
                return [res.data, Status.OK];
            case 404:
                return [null, Status.NOT_FOUND];
            case 401:
                return [null, Status.UNAUTHORIZED];
            case 500:
                return [null, Status.INTERNAL_SERVER_ERROR];
            default:
                return [null, Status.UNKNOWN];

        }
    } catch (err) {
        if (isAxiosError(err)) {
            switch (err.response?.status) {
                case 401:
                    return [null, Status.UNAUTHORIZED];
                case 500:
                    return [null, Status.INTERNAL_SERVER_ERROR];
                case 404:
                    return [null, Status.NOT_FOUND];
                default:
                    return [null, Status.UNKNOWN];
            }

        }
    }
    return [null, Status.UNKNOWN];
};

export const verifyUsuario = async (
    salida: boolean,
    motivo: string
): Promise<[ServiceTypes.Usuario | null, Status]> => {
    try {
        const res = await axios.post(ServiceTypes.API_URL + '/usuarios/verify', { salida, motivo });
        switch (res.status) {
            case 200:
                return [res.data, Status.OK];
            case 401:
                return [null, Status.UNAUTHORIZED];
            case 404:
                return [null, Status.NOT_FOUND];
            case 500:
                return [null, Status.INTERNAL_SERVER_ERROR];
            default:
                return [null, Status.UNKNOWN];
        }
    } catch (error) {
        if (isAxiosError(error)) {
            switch (error.response?.status) {
                case 401:
                    return [null, Status.UNAUTHORIZED];
                case 404:
                    return [null, Status.NOT_FOUND];
                case 500:
                    return [null, Status.INTERNAL_SERVER_ERROR];
                default:
                    return [null, Status.UNKNOWN];
            }
        }
    }
    return [null, Status.UNKNOWN];
};

export const getMotivos = async (): Promise<[Array<ServiceTypes.Motivo> | null, Status]> => {
    try {
        const res = await axios.get(ServiceTypes.API_URL + '/metadata/motivos');
        switch (res.status) {
            case 200:
                return [res.data, Status.OK];
            case 404:
                return [null, Status.NOT_FOUND];
            case 401:
                return [null, Status.UNAUTHORIZED];
            case 500:
                return [null, Status.INTERNAL_SERVER_ERROR];
        }
    } catch (error) {
        if (isAxiosError(error)) {
            switch (error.response?.status) {
                case 401:
                    return [null, Status.UNAUTHORIZED];
                case 500:
                    return [null, Status.INTERNAL_SERVER_ERROR];
                case 404:
                    return [null, Status.NOT_FOUND];
                default:
                    return [null, Status.UNKNOWN];
            }

        }
    }
    return [null, Status.UNKNOWN];
}

export const getRoles = async(): Promise<[Array<ServiceTypes.Rol> | null, Status]> => {
    try {
        const res = await axios.get(ServiceTypes.API_URL + '/metadata/roles');
        switch (res.status) {
            case 200:
                return [res.data, Status.OK];
            case 404:
                return [null, Status.NOT_FOUND];
            case 401:
                return [null, Status.UNAUTHORIZED];
            case 500:
                return [null, Status.INTERNAL_SERVER_ERROR];
        }
    } catch (error) {
        if (isAxiosError(error)) {
            switch (error.response?.status) {
                case 401:
                    return [null, Status.UNAUTHORIZED];
                case 500:
                    return [null, Status.INTERNAL_SERVER_ERROR];
                case 404:
                    return [null, Status.NOT_FOUND];
                default:
                    return [null, Status.UNKNOWN];
            }
        }
    }
    return [null, Status.UNKNOWN];
}

export const getRegistros = async (
    offset: number,
    limit: number
): Promise<[Array<ServiceTypes.Registro> | null, Status]> => {
    try {
        const res = await axios.get(
            ServiceTypes.API_URL + `/registros?limit=${limit}&offset=${offset}`
        );
        switch (res.status) {
            case 200:
                return [res.data, Status.OK];
            case 401:
                return [null, Status.UNAUTHORIZED];
            case 500:
                return [null, Status.INTERNAL_SERVER_ERROR];
        }
    } catch (error) {
        if (isAxiosError(error)) {
            switch (error.response?.status) {
                case 401:
                    return [null, Status.UNAUTHORIZED];
                case 500:
                    return [null, Status.INTERNAL_SERVER_ERROR];
            }
        }
    }
    return [null, Status.UNKNOWN];
};

export const getAdmins = async(): Promise<[Array<ServiceTypes.AdminGeneric> |null, Status]> => {
    try {
        const res = await axios.get(ServiceTypes.API_URL + '/admin/all');
        switch (res.status) {
            case 200:
                return [res.data, Status.OK];
            case 404:
                return [null, Status.NOT_FOUND];
            case 401:
                return [null, Status.UNAUTHORIZED];
            case 500:
                return [null, Status.INTERNAL_SERVER_ERROR];
        }
    } catch (error) {
        if (isAxiosError(error)) {
            switch (error.response?.status) {
                case 401:
                    return [null, Status.UNAUTHORIZED];
                case 500:
                    return [null, Status.INTERNAL_SERVER_ERROR];
                case 404:
                    return [null, Status.NOT_FOUND];
            }

        }
    }
    return [null, Status.UNKNOWN];
}

export const getCSVRegistro = async(from_date: string, to_date: string): Promise<[string | null, Status]> => {
    try {
        const res = await axios.get(ServiceTypes.API_URL + `/registros/reporte?from=${from_date}&to=${to_date}`,);
        switch (res.status) {
            case 200:
                return [res.data, Status.OK];
            case 404:
                return [null, Status.NOT_FOUND];
            case 401:
                return [null, Status.UNAUTHORIZED];
            case 500:
                return [null, Status.INTERNAL_SERVER_ERROR];
        }
    } catch (error) {
        if (isAxiosError(error)) {
            switch (error.response?.status) {
                case 401:
                    return [null, Status.UNAUTHORIZED];
                case 500:
                    return [null, Status.INTERNAL_SERVER_ERROR];
                case 404:
                    return [null, Status.NOT_FOUND];
            }
        }
    }
    return [null, Status.UNKNOWN];
}

const GetService = {
    getUsuarios,
    getUsuarioByRut,
    verifyUsuario,
    getRegistros,
    getMotivos,
    getAdmins,
    getRoles
};
export default GetService;
