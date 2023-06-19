import ServiceTypes from './types';
import axios, { AxiosResponse } from 'axios';
import Auth from './auth.service';
export const getUsuarios = async (): Promise<Array<ServiceTypes.Usuario> | undefined> => {
    const res = await axios.get(ServiceTypes.API_URL + '/usuarios', { headers: Auth.getToken() });
    return res.data;
};

export const getUsuarioByRut = async (rut: string): Promise<ServiceTypes.Usuario | undefined> => {
    const res = await axios.get(ServiceTypes.API_URL + `/usuarios/${rut}`, {
        headers: Auth.getToken(),
    });
    return res.data;
};

export const verifyUsuario = async (
    salida: boolean,
    motivo: string
): Promise<AxiosResponse<any, any> | undefined> => {
    const res = await axios.post(ServiceTypes.API_URL + '/usuarios/verify', { salida, motivo });
    return res;
};

export const getRegistros = async (
    offset: number,
    limit: number
): Promise<Array<ServiceTypes.Registro> | undefined> => {
    const res = await axios.get(
        ServiceTypes.API_URL + `/registros?limit=${limit}&offset=${offset}`,
        { headers: Auth.getToken() }
    );
    return res.data;
};

const GetService = {
    getUsuarios,
    getUsuarioByRut,
    verifyUsuario,
    getRegistros,
};
export default GetService;
