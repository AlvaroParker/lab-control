import ServiceTypes from './types';
import axios from 'axios';
import WebSocket from 'isomorphic-ws';

axios.defaults.withCredentials = true;

export const enrollUsuario = async (usuario: ServiceTypes.Usuario) => {
    const res = await axios.post(ServiceTypes.API_URL + `/usuarios`, {
        nombre: usuario.nombre,
        apellido_1: usuario.apellido_1,
        apellido_2: usuario.apellido_2,
        rut: usuario.rut,
        ultima_interaccion: new Date().toISOString(),
        entrada: true,
        correo_uai: usuario.correo_uai,
        is_disabled: false,
        rol: usuario.rol.toLowerCase(),
    });
    return res;
};

export const enrollAdmin = async (admin: ServiceTypes.AdminRegistro) => {
    const res = await axios.post(ServiceTypes.API_URL + `/admin/signin`, {
        nombre: admin.nombre,
        apellido_1: admin.apellido_1,
        apellido_2: admin.apellido_2,
        email: admin.email,
        pswd: admin.pswd
    });
    return res
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

export const nuevoMotivo = async (motivo: string) => {
    const res = await axios.post(ServiceTypes.API_URL + `/motivos`, { motivo: motivo });
    return res;
}

export const editUsuario = async (edit_usuario: ServiceTypes.Usuario, rut_viejo: string) => {
    const nombre = cleanVal(edit_usuario.nombre);
    const apellido_1 = cleanVal(edit_usuario.apellido_1);
    const apellido_2 = cleanVal(edit_usuario.apellido_2);
    const correo_uai = cleanVal(edit_usuario.correo_uai);
    const rut = cleanVal(edit_usuario.rut);
    const rol = cleanVal(edit_usuario.rol ? edit_usuario.rol.toLowerCase() : undefined);
    const res = await axios.put(ServiceTypes.API_URL + `/usuarios/${rut_viejo}`, {
        nombre,
        apellido_1,
        apellido_2,
        correo_uai,
        rut,
        rol,
    });
    return res;
};

export const nuevoRegistro = async (rut: string, salida: boolean, motivo: string) => {
    const res = await axios.post(ServiceTypes.API_URL + `/registros`, { rut, salida, motivo });
    return res;
};

const cleanVal = (s: string | undefined) => {
    if (!s || s.trim().length === 0) {
        return undefined;
    } else {
        return s;
    }
};

const PostService = {
    enrollUsuario,
    editUsuario,
    nuevoRegistro,
    enrollNewUsuario,
    enrollAdmin,
    rerollUsuario,
    nuevoMotivo
};
export default PostService;
