import ServiceTypes from './types';
import axios, { AxiosResponse } from 'axios';

axios.defaults.withCredentials = true;

export const deleteUsuario = async (rut: string) => {
    const res = await axios.delete(ServiceTypes.API_URL + `/usuarios/${rut}`);
    if (res.status == 401) {
        // redirect
    }
    return res;
};

export const deleteMotivo = async (id: number) => {
    const res = await axios.delete(ServiceTypes.API_URL + `/motivos/${id}`);
    if (res.status == 401) {
        // redirect
    }
    return res;
}

export const deleteAdmin = async(email: string): Promise<AxiosResponse<any, any> | undefined> => {
    const res = await axios.post(ServiceTypes.API_URL + `/admin/delete`, {
        email: email
    });
    return res;
}

const DeleteService = {
    deleteUsuario,
    deleteMotivo,
    deleteAdmin
};
export default DeleteService;
