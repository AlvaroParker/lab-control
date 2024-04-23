import ServiceTypes, { Status } from './types';
import axios, { isAxiosError } from 'axios';
import WebSocket from 'isomorphic-ws';

axios.defaults.withCredentials = true;


export const enrollAdmin = async (admin: ServiceTypes.AdminRegistro): Promise<Status> => {
    try {
        const res = await axios.post(ServiceTypes.API_URL + `/admin/signin`, {
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

export const enrollNewUsuario = async (usuario: ServiceTypes.Usuario): Promise<WebSocket> => {
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

    const url = `${ServiceTypes.WS_URI}/usuarios/enroll`;
    const ws = new WebSocket(url);

    ws.onopen = () => {
        ws.send(JSON.stringify(nuevo_usuario));
    };
    return ws;
};

export const rerollUsuario = async (rutUsuario: string): Promise<WebSocket> => {
    const url = `${ServiceTypes.WS_URI}/usuarios/reroll`;
    const ws = new WebSocket(url);

    // Send the user's rut to the server as socket message
    ws.onopen = () => {
        ws.send(JSON.stringify({ rut: rutUsuario }));
    };

    return ws;
}

export const nuevoMotivo = async (motivo: string): Promise<Status> => {
    try {
        const res = await axios.post(ServiceTypes.API_URL + `/metadata/motivos`, { motivo: motivo });
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

export const newRol = async (rol: string): Promise<Status> => {
    try {
        const res = await axios.post(ServiceTypes.API_URL + `/metadata/roles`, { rol: rol});
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

export const editUsuario = async (edit_usuario: ServiceTypes.Usuario, rut_viejo: string): Promise<Status> => {
    const nombre = cleanVal(edit_usuario.nombre);
    const apellido_1 = cleanVal(edit_usuario.apellido_1);
    const apellido_2 = cleanVal(edit_usuario.apellido_2);
    const correo_uai = cleanVal(edit_usuario.correo_uai);
    const rut = cleanVal(edit_usuario.rut);
    const rol = cleanVal(edit_usuario.rol ? edit_usuario.rol.toLowerCase() : undefined);
    try {
        const res = await axios.put(ServiceTypes.API_URL + `/usuarios/${rut_viejo}`, {
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

export const nuevoRegistro = async (rut: string, salida: boolean, motivo: string): Promise<Status> => {
    try {
        const res = await axios.post(ServiceTypes.API_URL + `/registros`, { rut, salida, motivo });
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

export const changePassword = async (email: string, pswd: string): Promise<Status> => {
    try {
        const res = await axios.post(ServiceTypes.API_URL + `/admin/change`, {
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
    editUsuario,
    nuevoRegistro,
    enrollNewUsuario,
    enrollAdmin,
    rerollUsuario,
    nuevoMotivo,
    changePassword
};
export default PostService;
