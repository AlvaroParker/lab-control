import AuthService from './auth.service';
import ServiceTypes from './types';
import axios from 'axios';

axios.defaults.withCredentials = true;

export const deleteUsuario = async (rut: string) => {
    const res = await axios.delete(ServiceTypes.API_URL + `/usuarios/${rut}`, {
        headers: AuthService.getToken(),
    });
    if (res.status == 401) {
        // redirect
    }
    return res;
};

const DeleteService = {
    deleteUsuario,
};
export default DeleteService;
