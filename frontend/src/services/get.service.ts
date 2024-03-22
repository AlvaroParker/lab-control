import ServiceTypes from './types';
import axios, { AxiosResponse } from 'axios';

axios.defaults.withCredentials = true;

export const getUsuarios = async (): Promise<Array<ServiceTypes.Usuario> | undefined> => {
    const res = await axios.get(ServiceTypes.API_URL + '/usuarios');
    return res.data;
};

export const getUsuarioByRut = async (rut: string): Promise<ServiceTypes.Usuario | undefined> => {
    const res = await axios.get(ServiceTypes.API_URL + `/usuarios/${rut}`);
    return res.data;
};

export const verifyUsuario = async (
    salida: boolean,
    motivo: string
): Promise<AxiosResponse<any, any> | undefined> => {
    const res = await axios.post(ServiceTypes.API_URL + '/usuarios/verify', { salida, motivo });
    return res;
};

export const getMotivos = async (): Promise<AxiosResponse<any, any>> => {
    const res = await axios.get(ServiceTypes.API_URL + '/motivos');
    return res
}

export const getRegistros = async (
    offset: number,
    limit: number
): Promise<Array<ServiceTypes.Registro> | undefined> => {
    const res = await axios.get(
        ServiceTypes.API_URL + `/registros?limit=${limit}&offset=${offset}`
    );
    return res.data;
};

const GetService = {
    getUsuarios,
    getUsuarioByRut,
    verifyUsuario,
    getRegistros,
    getMotivos
};
export default GetService;
