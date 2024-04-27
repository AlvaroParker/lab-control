import * as ServiceTypes from './types';
import { Status } from './types';
import axios, { isAxiosError } from 'axios';
import WebSocket from 'isomorphic-ws';
import { getURL, getWSURL } from '.';

axios.defaults.withCredentials = true;

/**
 * This function enrolls an admin to the system. The admin data must be provided in the `admin` object.
 * 
 * @param admin - Admin data to enroll
 * @returns `Status` - Status of the request
 * 
 * @example
 * Simple example:
 * ```
 * import { PostService, Status } from 'lab-control';
 * 
 * const admin = {
 *  nombre: "John",
 *  apellido_1: "Doe",
 *  apellido_2: "Rambo",
 *  email: "john@doe.com",
 *  pswd: "someSuperSecretPassword"
 * }
 * const status = await PostService.EnrollAdmin(admin);
 * if (status === Status.OK) {
 *  console.log("Admin enrolled successfully");
 * }
 * ```
 */
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
        if (res.status === 200)
            return Status.OK;
    } catch (error) {
        if (isAxiosError(error)) {
            switch (error.response?.status) {
                case 400:
                    return Status.BAD_REQUEST;
                case 401:
                    return Status.UNAUTHORIZED;
                case 404:
                    return Status.NOT_FOUND;
                case 409:
                    return Status.CONFLICT;
                case 500:
                    return Status.INTERNAL_SERVER_ERROR;
            }
        }
    }
    return Status.UNKNOWN;
}

/**
 * This function starts the enrollment process of a new user. The user data must be provided in the `usuario` object.
 * Once the request is sent, if the fingerprint scanner is connected, the user will be prompted to scan their fingerprint. 
 * The websocket will send how many times the user has to scan their fingerprint (`total`) and how many scan it has already done (`current`).
 * 
 * Example of the message sent by the server:
 * ```json
 * {
 *  "total": 5,
 *  "current": 1
 * }
 * ```
 * This means that the user has to scan their fingerprint 5 times and they have already scanned it once.
 * To handle the websocket messages, you can use the `onmessage` event of the WebSocket object:
 * ```typescript
 * ws.onmessage = (event) => {
 *      const data = JSON.parse(event.data);
 *      console.log(data.total); // Prints the total number of scans
 *      console.log(data.current); // Prints the current scan
 * }
 * ```
 * The websocket connection also has custom close codes that can be handled using the `onclose` event of the `WebSocket` object:
 * 
 * - `1000`: The user was successfully enrolled
 * - `4000`: Email o RUT already registered
 * - `4001`: Invalid RUT
 * - `4002`: Missing fields
 * - `4500`: Internal server error
 * 
 * @param usuario - User data to enroll
 * @returns `WebSocket` - WebSocket connection
 * 
 * @example
 * Example of how to use the function:
 * 
 * ```
 * import { PostService, ServiceTypes } from 'lab-control';
 * 
 * const usuario: ServiceTypes.Usuario = {
 *      nombre: "John",
 *      apellido_1: "Doe",
 *      apellido_2: "Rambo",
 *      rut: "12345678-9",
 *      print_path: "", // This will be filled by the server
 *      correo_uai: "some@email.com",
 *      rol: "alumno", // A list of valid roles can be obtained with `GetService.GetRoles()`
 *      last_registro: null
 * };
 * 
 * const ws = await PostService.EnrollNewUsuario(usuario);
 * ws.onmessage = (event) => {
 *     const data = JSON.parse(event.data);
 *     console.log(data.total); // Prints the total number of scans
 *     console.log(data.current); // Prints the current scan
 * };
 * ```
 */
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

/**
 * This function allows to update the fingerprint of a user given its rut.
 * Once the request is sent, if the fingerprint scanner is connected, the user will be prompted to scan their fingerprint. 
 * The websocket will send how many times the user has to scan their fingerprint (`total`) and how many scan it has already done (`current`).
 * 
 * Example of the message sent by the server:
 * ```json
 * {
 *  "total": 5,
 *  "current": 1
 * }
 * ```
 * This means that the user has to scan their fingerprint 5 times and they have already scanned it once.
 * To handle the websocket messages, you can use the `onmessage` event of the WebSocket object:
 * ```typescript
 * ws.onmessage = (event) => {
 *      const data = JSON.parse(event.data);
 *      console.log(data.total); // Prints the total number of scans
 *      console.log(data.current); // Prints the current scan
 * }
 * ```
 * The websocket connection also has custom close codes that can be handled using the `onclose` event of the `WebSocket` object:
 * 
 * - `1000`: The user was successfully enrolled
 * - `4000`: Email o RUT already registered
 * - `4001`: Invalid RUT
 * - `4002`: Missing fields
 * - `4500`: Internal server error
 * 
 * @param rutUsuario - The rut of the user to reroll
 * @returns `WebSocket` - WebSocket connection
 * 
 * @example
 * Example of how to use the function:
 * ```
 * import { PostService, Status } from 'lab-control';
 * 
 * const ws = await PostService.RerollUsuario("12345678-9");
 * ws.onmessage = (event) => {
 *      const data = JSON.parse(event.data);
 *      console.log(data.total); // Prints the total number of scans
 *      console.log(data.current); // Prints the current scan
 *  };
 * ```
 */
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

/**
 * This function adds a new motivo to the database.
 * 
 * @param motivo - The new motivo to add
 * @returns `Status` - Status of the request
 * 
 * @example
 * Simple example:
 * ```
 * import { PostService, Status } from 'lab-control';
 * 
 * const status = await PostService.NewMotivo("Ayudantia");
 * if (status === Status.OK) {
 *      console.log("Motivo added successfully");
 * }
 * ```
 */
export const NewMotivo = async (motivo: string): Promise<Status> => {
    const api_url = getURL();
    try {
        const res = await axios.post(api_url + `/metadata/motivos`, { motivo: motivo });
        if (res.status === 200)
            return Status.OK;
    } catch (error) {
        if (isAxiosError(error)) {
            switch (error.response?.status) {
                case 400:
                    return Status.BAD_REQUEST;
                case 401:
                    return Status.UNAUTHORIZED;
                case 404:
                    return Status.NOT_FOUND;
                case 409:
                    return Status.CONFLICT;
                case 500:
                    return Status.INTERNAL_SERVER_ERROR;
            }
        }
    }
    return Status.UNKNOWN;
}

/**
 * This function adds a new rol to the database.
 * 
 * @param rol - The new rol to add
 * @returns `Status` - Status of the request
 * 
 * @example
 * Simple example:
 * ```
 * import { PostService, Status } from 'lab-control';
 * 
 * const status = await PostService.NewRol("alumno");
 * if (status === Status.OK) {
 *    console.log("Rol added successfully");
 * };
 * ```
 */
export const NewRol = async (rol: string): Promise<Status> => {
    const api_url = getURL();
    try {
        const res = await axios.post(api_url + `/metadata/roles`, { rol: rol });
        if (res.status === 200)
            return Status.OK;
    } catch (error) {
        if (isAxiosError(error)) {
            switch (error.response?.status) {
                case 400:
                    return Status.BAD_REQUEST;
                case 401:
                    return Status.UNAUTHORIZED;
                case 404:
                    return Status.NOT_FOUND;
                case 409:
                    return Status.CONFLICT;
                case 500:
                    return Status.INTERNAL_SERVER_ERROR;
            }
        }
    }
    return Status.UNKNOWN;
}

/**
 * This function allows to edit the data of a user given its rut. The new data must be provided in the `edit_usuario` object.
 * If a field in the `edit_usuario` object is `undefined` or an empty string, it will not be updated.
 * 
 * @param edit_usuario - The new data for the user
 * @param rut_viejo - The rut of the user to edit
 * @returns `Status` - Status of the request
 * 
 * @example
 * A simple example:
 * ```
 * import { PostService, Status } from 'lab-control';
 * 
 * const edit_usuario = {
 *  nombre: "John",
 * };
 * 
 * const status = await PostService.EditUsuario(edit_usuario, "12345678-9");
 * 
 * if (status === Status.OK) {
 *      console.log("User edited successfully");
 * };
 * ``` 
 */
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
        if (res.status === 200)
            return Status.OK;
    } catch (error) {
        if (isAxiosError(error)) {
            switch (error.response?.status) {
                case 400:
                    return Status.BAD_REQUEST;
                case 401:
                    return Status.UNAUTHORIZED;
                case 404:
                    return Status.NOT_FOUND;
                case 409:
                    return Status.CONFLICT;
                case 500:
                    return Status.INTERNAL_SERVER_ERROR;
            }
        }
    }
    return Status.UNKNOWN;
};

/**
 * This function allows to create a new registro in the database. The user's rut, if they are entering or leaving and the reason must be provided.
 * Not to be confused with `GetService.VerifyUsuario` which requires the user to scan their fingerprint, here the user provides their rut. This function
 * can be used as a fallback if the fingerprint scanner is not available.
 * 
 * @param rut - The rut of the user that's entering or leaving
 * @param salida - If the user is entering or leaving
 * @param motivo - The reason of the user's action
 * 
 * @returns `Status` - Status of the request
 * 
 * @example
 * A simple example:
 * ```
 * import { PostService, Status } from 'lab-control';
 * 
 * const status = await PostService.NewRegistro("12345678-9", false, "Ayudantia");
 * if (status === Status.OK) {
 *     console.log("Registro added successfully");
 * }
 * ```
 */
export const NewRegistro = async (rut: string, salida: boolean, motivo: string): Promise<Status> => {
    const api_url = getURL();
    try {
        const res = await axios.post(api_url + `/registros`, { rut, salida, motivo });
        if (res.status === 200)
            return Status.OK;
    } catch (error) {
        if (isAxiosError(error)) {
            switch (error.response?.status) {
                case 400:
                    return Status.BAD_REQUEST;
                case 401:
                    return Status.UNAUTHORIZED;
                case 404:
                    return Status.NOT_FOUND;
                case 409:
                    return Status.CONFLICT;
                case 500:
                    return Status.INTERNAL_SERVER_ERROR;
            }
        }
    }
    return Status.UNKNOWN;
};

/**
 * This function allows to change the password of an admin given its email.
 * 
 * @param email - Email of the admin
 * @param pswd - New password of the admin
 * @returns `Status` - Status of the request
 * 
 * @example
 * A simple example:
 * ```
 * import { PostService, Status } from 'lab-control';
 * 
 * const status = await PostService.ChangePassword("some@email.com", "newPassword");
 * if (status === Status.OK) {
 *    console.log("Password changed successfully");
 * };
 * ```
 */
export const ChangePassword = async (email: string, pswd: string): Promise<Status> => {
    const api_url = getURL();
    try {
        const res = await axios.post(api_url + `/admin/change`, {
            email: email,
            pswd: pswd
        });
        if (res.status === 200)
            return Status.OK;
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

const cleanVal = (s: string | undefined) => {
    if (!s || s.trim().length === 0) {
        return undefined;
    } else {
        return s;
    }
};
