import ServiceTypes, { Status } from './types';
import axios from 'axios';

axios.defaults.withCredentials = true;

export const deleteUsuario = async (rut: string): Promise<Status> => {
    try {
        const res = await axios.delete(ServiceTypes.API_URL + `/usuarios/${rut}`);
        switch (res.status) {
            case 200:
                return Status.OK;
            case 401:
                return Status.UNAUTHORIZED;
            case 404:
                return Status.NOT_FOUND;
            case 500:
                return Status.INTERNAL_SERVER_ERROR;
        }
    } catch (error) {
        if (axios.isAxiosError(error)) {
            switch (error.response?.status) {
                case 401:
                    return Status.UNAUTHORIZED;
                case 404:
                    return Status.NOT_FOUND;
                case 500:
                    return Status.INTERNAL_SERVER_ERROR;
            }
        }
    }
    return Status.UNKNOWN;
};

export const deleteMotivo = async (id: number): Promise<Status> => {
    try {
        const res = await axios.delete(ServiceTypes.API_URL + `/metadata/motivos/${id}`);
        switch (res.status) {
            case 200:
                return Status.OK;
            case 401:
                return Status.UNAUTHORIZED;
            case 404:
                return Status.NOT_FOUND;
            case 500:
                return Status.INTERNAL_SERVER_ERROR;
        }
    } catch (error) {
        if (axios.isAxiosError(error)) {
            switch (error.response?.status) {
                case 401:
                    return Status.UNAUTHORIZED;
                case 404:
                    return Status.NOT_FOUND;
                case 500:
                    return Status.INTERNAL_SERVER_ERROR;
            }
        }
    }
    return Status.UNKNOWN;
}

export const deleteRol = async (id: number): Promise<Status> => {
    try {
        const res = await axios.delete(ServiceTypes.API_URL + `/metadata/roles/${id}`);
        switch (res.status) {
            case 200:
                return Status.OK;
            case 401:
                return Status.UNAUTHORIZED;
            case 404:
                return Status.NOT_FOUND;
            case 500:
                return Status.INTERNAL_SERVER_ERROR;
        }
    } catch (error) {
        if (axios.isAxiosError(error)) {
            switch (error.response?.status) {
                case 401:
                    return Status.UNAUTHORIZED;
                case 404:
                    return Status.NOT_FOUND;
                case 500:
                    return Status.INTERNAL_SERVER_ERROR;
            }
        }
    }
    return Status.UNKNOWN;
}

export const deleteAdmin = async(email: string): Promise<Status> => {
    try {
        const res = await axios.post(ServiceTypes.API_URL + `/admin/delete`, {
            email: email
        });
        switch (res.status) {
            case 200:
                return Status.OK;
            case 401:
                return Status.UNAUTHORIZED;
            case 404:
                return Status.NOT_FOUND;
            case 500:
                return Status.INTERNAL_SERVER_ERROR;
        }
    } catch (error) {
        if (axios.isAxiosError(error)) {
            switch (error.response?.status) {
                case 401:
                    return Status.UNAUTHORIZED;
                case 404:
                    return Status.NOT_FOUND;
                case 500:
                    return Status.INTERNAL_SERVER_ERROR;
            }
        }
    }
    return Status.UNKNOWN;
}

const DeleteService = {
    deleteUsuario,
    deleteMotivo,
    deleteAdmin
};
export default DeleteService;
