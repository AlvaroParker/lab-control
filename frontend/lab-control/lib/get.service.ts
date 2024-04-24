import { getURL } from '.';
import ServiceTypes, { Status } from './types';
import axios, { isAxiosError } from 'axios';

axios.defaults.withCredentials = true;

export const GetUsuarios = async (): Promise<[Array<ServiceTypes.Usuario>, Status]> => {
    const api_url = getURL();
    try {
        const res = await axios.get(api_url + '/usuarios');
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

export const GetUsuarioByRut = async (rut: string): Promise<[ServiceTypes.Usuario | null, Status]> => {
    const api_url = getURL();
    try {
        const res = await axios.get(api_url + `/usuarios/${rut}`);
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

export const VerifyUsuario = async (
    salida: boolean,
    motivo: string
): Promise<[ServiceTypes.Usuario | null, Status]> => {
    const api_url = getURL();
    try {
        const res = await axios.post(api_url + '/usuarios/verify', { salida, motivo });
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

export const GetMotivos = async (): Promise<[Array<ServiceTypes.Motivo> | null, Status]> => {
    const api_url = getURL();
    try {
        const res = await axios.get(api_url + '/metadata/motivos');
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

export const GetRoles = async (): Promise<[Array<ServiceTypes.Rol> | null, Status]> => {
    const api_url = getURL();
    try {
        const res = await axios.get(api_url + '/metadata/roles');
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

export const GetRegistros = async (
    offset: number,
    limit: number
): Promise<[Array<ServiceTypes.Registro> | null, Status]> => {
    const api_url = getURL();
    try {
        const res = await axios.get(
            api_url + `/registros?limit=${limit}&offset=${offset}`
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

export const GetAdmins = async (): Promise<[Array<ServiceTypes.AdminGeneric> | null, Status]> => {
    const api_url = getURL();
    try {
        const res = await axios.get(api_url + '/admin/all');
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

export const GetCSVRegistro = async (from_date: string, to_date: string): Promise<[string | null, Status]> => {
    const api_url = getURL();
    try {
        const res = await axios.get(api_url + `/registros/reporte?from=${from_date}&to=${to_date}`,);
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
    GetUsuarios,
    GetUsuarioByRut,
    VerifyUsuario,
    GetRegistros,
    GetMotivos,
    GetAdmins,
    GetRoles,
    GetCSVRegistro
};
export default GetService;
