import ServiceTypes from './types';
import axios from 'axios';

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

const DeleteService = {
    deleteUsuario,
    deleteMotivo
};
export default DeleteService;
