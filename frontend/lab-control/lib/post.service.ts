import ServiceTypes, { Status } from './types';
import axios, { isAxiosError } from 'axios';
import WebSocket from 'isomorphic-ws';
import { getURL, getWSURL } from '.';

axios.defaults.withCredentials = true;


export const EnrollAdmin = async (admin: ServiceTypes.AdminRegistro): Promise<Status> => {
    const api_url = getURL();
    try {
        const res = await axios.post(api_url + `/admin/signin`, {
            nombre: admin.nombre,
            apellido_1: admin.apellido_1,
            apellido_2: admin.apellido_2,
            email: admin.email,
            pswd: admin.pswd
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
        if (isAxiosError(error)) {
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

export const EnrollNewUsuario = async (usuario: ServiceTypes.Usuario): Promise<WebSocket> => {
    const nuevo_usuario = {
        nombre: usuario.nombre,
        apellido_1: usuario.apellido_1,
        apellido_2: usuario.apellido_2,
        rut: usuario.rut,
        ultima_interaccion: new Date().toISOString(),
        entrada: true,
        correo_uai: usuario.correo_uai,
        is_disabled: false,
        rol: usuario.rol.toLowerCase(),
    };
    const WS_URI = getWSURL();

    const url = `${WS_URI}/usuarios/enroll`;
    const ws = new WebSocket(url);

    ws.onopen = () => {
        ws.send(JSON.stringify(nuevo_usuario));
    };
    return ws;
};

export const RerollUsuario = async (rutUsuario: string): Promise<WebSocket> => {
    const WS_URI = getWSURL();
    const url = `${WS_URI}/usuarios/reroll`;
    const ws = new WebSocket(url);

    // Send the user's rut to the server as socket message
    ws.onopen = () => {
        ws.send(JSON.stringify({ rut: rutUsuario }));
    };

    return ws;
}

export const NewMotivo = async (motivo: string): Promise<Status> => {
    const api_url = getURL();
    try {
        const res = await axios.post(api_url + `/metadata/motivos`, { motivo: motivo });
        switch (res.status) {
            case 200:
                return Status.OK;
            case 400:
                return Status.BAD_REQUEST;
            case 401:
                return Status.UNAUTHORIZED;
            case 404:
                return Status.NOT_FOUND;
            case 500:
                return Status.INTERNAL_SERVER_ERROR;
        }
    } catch (error) {
        if (isAxiosError(error)) {
            switch (error.response?.status) {
                case 400:
                    return Status.BAD_REQUEST;
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

export const NewRol = async (rol: string): Promise<Status> => {
    const api_url = getURL();
    try {
        const res = await axios.post(api_url + `/metadata/roles`, { rol: rol });
        switch (res.status) {
            case 200:
                return Status.OK;
            case 400:
                return Status.BAD_REQUEST;
            case 401:
                return Status.UNAUTHORIZED;
            case 404:
                return Status.NOT_FOUND;
            case 500:
                return Status.INTERNAL_SERVER_ERROR;
        }
    } catch (error) {
        if (isAxiosError(error)) {
            switch (error.response?.status) {
                case 400:
                    return Status.BAD_REQUEST;
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

export const EditUsuario = async (edit_usuario: ServiceTypes.Usuario, rut_viejo: string): Promise<Status> => {
    const api_url = getURL();
    const nombre = cleanVal(edit_usuario.nombre);
    const apellido_1 = cleanVal(edit_usuario.apellido_1);
    const apellido_2 = cleanVal(edit_usuario.apellido_2);
    const correo_uai = cleanVal(edit_usuario.correo_uai);
    const rut = cleanVal(edit_usuario.rut);
    const rol = cleanVal(edit_usuario.rol ? edit_usuario.rol.toLowerCase() : undefined);
    try {
        const res = await axios.put(api_url + `/usuarios/${rut_viejo}`, {
            nombre,
            apellido_1,
            apellido_2,
            correo_uai,
            rut,
            rol,
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
        if (isAxiosError(error)) {
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

export const NewRegistro = async (rut: string, salida: boolean, motivo: string): Promise<Status> => {
    const api_url = getURL();
    try {
        const res = await axios.post(api_url + `/registros`, { rut, salida, motivo });
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
        if (isAxiosError(error)) {
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

export const ChangePassword = async (email: string, pswd: string): Promise<Status> => {
    const api_url = getURL();
    try {
        const res = await axios.post(api_url + `/admin/change`, {
            email: email,
            pswd: pswd
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
        if (isAxiosError(error)) {
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

const cleanVal = (s: string | undefined) => {
    if (!s || s.trim().length === 0) {
        return undefined;
    } else {
        return s;
    }
};

const PostService = {
    EditUsuario,
    NewRegistro,
    EnrollNewUsuario,
    EnrollAdmin,
    RerollUsuario,
    NewMotivo,
    ChangePassword,
    NewRol
};
export default PostService;
