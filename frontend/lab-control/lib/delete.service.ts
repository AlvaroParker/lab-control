import { getURL } from '.';
import ServiceTypes, { Status } from './types';
import axios from 'axios';

axios.defaults.withCredentials = true;

export const DeleteUsuario = async (rut: string): Promise<Status> => {
    const api_url = getURL();
    try {
        const res = await axios.delete(api_url + `/usuarios/${rut}`);
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

export const DeleteMotivo = async (id: number): Promise<Status> => {
    const api_url = getURL();
    try {
        const res = await axios.delete(api_url + `/metadata/motivos/${id}`);
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

export const DeleteRol = async (id: number): Promise<Status> => {
    const api_url = getURL();
    try {
        const res = await axios.delete(api_url + `/metadata/roles/${id}`);
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

export const DeleteAdmin = async (email: string): Promise<Status> => {
    const api_url = getURL();
    try {
        const res = await axios.post(api_url + `/admin/delete`, {
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
    DeleteUsuario,
    DeleteMotivo,
    DeleteAdmin,
    DeleteRol
};
export default DeleteService;
