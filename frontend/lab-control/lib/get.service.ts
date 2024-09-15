import * as ServiceTypes from './types'
import { Status } from './types'
import { isAxiosError } from 'axios'
import { axiosInstace as axios } from '.'

export const Search = async (
    query: string
): Promise<[Array<ServiceTypes.Usuario> | null, Status]> => {
    try {
        const res = await axios.get(`/usuarios/search?query=${query}`)
        if (res.status === 200) return [res.data, Status.OK]
    } catch (error) {
        if (isAxiosError(error)) {
            switch (error.response?.status) {
                case 400:
                    return [null, Status.BAD_REQUEST]
                case 401:
                    return [null, Status.UNAUTHORIZED]
                case 404:
                    return [null, Status.NOT_FOUND]
                case 409:
                    return [null, Status.CONFLICT]
                case 500:
                    return [null, Status.INTERNAL_SERVER_ERROR]
            }
        }
    }
    return [null, Status.UNKNOWN]
}

/**
 * Gets the list of users from the server
 * @remarks
 *
 * This function is used to get the list of users from the server. It returns a tuple with the array of users and the status code.
 * If the status code is `Status.OK` then the array of users is valid, otherwise it is empty.
 *
 * @returns [Array<ServiceTypes.Usuario>, Status]
 *
 * @example
 * Simple example:
 * ```
 * import { GetService, Status } from 'lab-control';
 *
 * const [usuarios, status] = await GetService.GetUsuarios();
 * if (status === Status.OK) {
 *   console.log(usuarios);
 * }
 * ````
 **/
export const GetUsuarios = async (): Promise<
    [Array<ServiceTypes.Usuario>, Status]
> => {
    try {
        const res = await axios.get('/usuarios')
        if (res.status === 200) {
            return [res.data, Status.OK]
        }
    } catch (err) {
        if (isAxiosError(err)) {
            switch (err.response?.status) {
                case 400:
                    return [[], Status.BAD_REQUEST]
                case 401:
                    return [[], Status.UNAUTHORIZED]
                case 404:
                    return [[], Status.NOT_FOUND]
                case 500:
                    return [[], Status.INTERNAL_SERVER_ERROR]
            }
        }
    }
    return [[], Status.UNKNOWN]
}

/**
 * Returns the user with the given run if it exists, otherwise it returns null. If Status is OK, the user exists and
 * it is safe to use the returned user.
 *
 * @remarks
 *
 * This function is used to get a user by its rut. It returns a tuple with the user and the status code.
 * If the status code is `Status.OK` then the user is valid, otherwise it is null.
 *
 * @param rut - The rut of the user to get
 * @returns [ServiceTypes.Usuario | null, Status]
 *
 * @example
 * Simple example:
 * ```
 * import { GetService, Status } from 'lab-control';
 *
 * const [usuario, status] = await GetService.GetUsuarioByRut('12345678-9');
 * if (status === Status.OK) {
 *  console.log(usuario);
 * }
 * ```
 */
export const GetUsuarioByRut = async (
    rut: string
): Promise<[ServiceTypes.Usuario | null, Status]> => {
    try {
        const res = await axios.get(`/usuarios/${rut}`)
        if (res.status) return [res.data, Status.OK]
    } catch (err) {
        if (isAxiosError(err)) {
            switch (err.response?.status) {
                case 400:
                    return [null, Status.BAD_REQUEST]
                case 401:
                    return [null, Status.UNAUTHORIZED]
                case 404:
                    return [null, Status.NOT_FOUND]
                case 500:
                    return [null, Status.INTERNAL_SERVER_ERROR]
            }
        }
    }
    return [null, Status.UNKNOWN]
}

/**
 * This function is used to mark the entrance or exit of users from the premises. It activates the fingerprint reader and
 * waits for the user to verify their identity. If the user is verified, it returns the user, otherwise it returns null.
 *
 * @remarks
 * This function returns a tuple with the user (`ServiceTypes.Usuario`) and the status code (`Status`).
 * If the status code is `Status.OK` then the user is valid, otherwise it is `null`.
 *
 * @param salida - If the user is leaving the lab/center
 * @param motivo - The reason why the user is entering/leaving the lab/center. You can provide your own `motivo` or one from the list of `motivos` provided by the {@link GetService.GetMotivos} function.
 * @returns `[ServiceTypes.Usuario | null, Status]` - The user that was verified and the status of the request.
 *
 * @example
 * Simple example:
 * ```
 * import { GetService, Status } from 'lab-control';
 *
 * const [usuario, status] = await GetService.VerifyUsuario(false, 'Asistencia Ramo');
 * if (status === Status.OK) {
 *  console.log(usuario);
 * }
 *
 * ```
 */
export const VerifyUsuario = async (
    salida: boolean,
    motivo: string
): Promise<[ServiceTypes.Usuario | null, Status]> => {
    try {
        const res = await axios.post('/usuarios/verify', {
            salida,
            motivo,
        })
        if (res.status) return [res.data, Status.OK]
    } catch (error) {
        if (isAxiosError(error)) {
            switch (error.response?.status) {
                case 400:
                    return [null, Status.BAD_REQUEST]
                case 401:
                    return [null, Status.UNAUTHORIZED]
                case 404:
                    return [null, Status.NOT_FOUND]
                case 500:
                    return [null, Status.INTERNAL_SERVER_ERROR]
            }
        }
    }
    return [null, Status.UNKNOWN]
}

/**
 * This function is used to get the list of `motivos` (reasons) to enter or leave the lab/center.
 *
 * @remarks
 * This function returns a tuple with the list of reasons (`Array<ServiceTypes.Motivo>`) and the status code (`Status`).
 * If the status code is `Status.OK` then the list of reasons is valid, otherwise it is `null`.
 *
 * @returns `[Array<ServiceTypes.Motivo> | null, Status]` - The list of reasons or `null` if there was an error.
 *
 * @example
 * Simple example:
 * ```
 * import { GetService, Status } from 'lab-control';
 *
 * const [motivos, status] = await GetService.GetMotivos();
 * if (status === Status.OK) {
 * console.log(motivos);
 * }
 * ```
 */
export const GetMotivos = async (): Promise<
    [Array<ServiceTypes.Motivo> | null, Status]
> => {
    try {
        const res = await axios.get('/metadata/motivos')
        if (res.status) return [res.data, Status.OK]
    } catch (error) {
        if (isAxiosError(error)) {
            switch (error.response?.status) {
                case 400:
                    return [null, Status.BAD_REQUEST]
                case 401:
                    return [null, Status.UNAUTHORIZED]
                case 404:
                    return [null, Status.NOT_FOUND]
                case 500:
                    return [null, Status.INTERNAL_SERVER_ERROR]
            }
        }
    }
    return [null, Status.UNKNOWN]
}

/**
 * This function is used to get the list of roles that exist for a user. This is used to register a new user or to update an existing user.
 *
 * See {@link PostService.EnrollNewUsuario} for more information on how to register a new user.
 *
 * @remarks
 * This function returns a tuple with the list of roles (`Array<ServiceTypes.Rol>`) and the status code (`Status`).
 * If the status code is `Status.OK` then the list of roles is valid, otherwise it is `null`.
 *
 * @returns `[Array<ServiceTypes.Rol> | null, Status]` - The list of roles or `null` if there was an error.
 *
 * @example
 * Simple example:
 * ```
 * import { GetService, Status } from 'lab-control';
 *
 * const [roles, status] = await GetService.GetRoles();
 * if (status === Status.OK) {
 * console.log(roles);
 * }
 * ```
 */
export const GetRoles = async (): Promise<
    [Array<ServiceTypes.Rol> | null, Status]
> => {
    try {
        const res = await axios.get('/metadata/roles')
        if (res.status === 200) return [res.data, Status.OK]
    } catch (error) {
        if (isAxiosError(error)) {
            switch (error.response?.status) {
                case 400:
                    return [null, Status.BAD_REQUEST]
                case 401:
                    return [null, Status.UNAUTHORIZED]
                case 404:
                    return [null, Status.NOT_FOUND]
                case 500:
                    return [null, Status.INTERNAL_SERVER_ERROR]
            }
        }
    }
    return [null, Status.UNKNOWN]
}

/**
 * This function is used to get the list of records of users entering or leaving the lab/center.
 *
 * @remarks
 * This function returns a tuple with the list of records (`Array<ServiceTypes.Registro>`) and the status code (`Status`).
 * If the status code is `Status.OK` then the list of records is valid, otherwise it is `null`.
 *
 * @param offset - The offset to start getting the records
 * @param limit - The limit of records to get
 * @returns `[Array<ServiceTypes.Registro> | null, Status]` - The list of records or `null` if there was an error.
 *
 * @example
 * Simple example:
 * ```
 * import { GetService, Status } from 'lab-control';
 *
 * const [registros, status] = await GetService.GetRegistros(0, 10); // Get the first 10 records
 * if (status === Status.OK) {
 * console.log(registros);
 * }
 * ```
 */
export const GetRegistros = async (
    offset: number,
    limit: number
): Promise<[Array<ServiceTypes.Registro> | null, Status]> => {
    if (offset < 0 || limit < 0) {
        return [null, Status.BAD_REQUEST]
    }
    try {
        const res = await axios.get(
            `/registros?limit=${limit}&offset=${offset}`
        )
        if (res.status === 200) return [res.data, Status.OK]
    } catch (error) {
        if (isAxiosError(error)) {
            switch (error.response?.status) {
                case 400:
                    return [null, Status.BAD_REQUEST]
                case 401:
                    return [null, Status.UNAUTHORIZED]
                case 404:
                    return [null, Status.NOT_FOUND]
                case 500:
                    return [null, Status.INTERNAL_SERVER_ERROR]
            }
        }
    }
    return [null, Status.UNKNOWN]
}

/**
 * This function is used to get the list of admins that exist in the system.
 *
 * @remarks
 * This function returns a tuple with the list of admins (`Array<ServiceTypes.AdminGeneric>`) and the status code (`Status`).
 * If the status code is `Status.OK` then the list of admins is valid, otherwise it is `null`.
 *
 * @returns `[Array<ServiceTypes.AdminGeneric> | null, Status]` - The list of admins or `null` if there was an error.
 *
 * @example
 * Simple example:
 * ```
 * import { GetService, Status } from 'lab-control';
 *
 * const [admins, status] = await GetService.GetAdmins();
 * if (status === Status.OK) {
 * console.log(admins);
 * }
 * ```
 */
export const GetAdmins = async (): Promise<
    [Array<ServiceTypes.AdminGeneric> | null, Status]
> => {
    try {
        const res = await axios.get('/admin/all')
        if (res.status === 200) return [res.data, Status.OK]
    } catch (error) {
        if (isAxiosError(error)) {
            switch (error.response?.status) {
                case 400:
                    return [null, Status.BAD_REQUEST]
                case 401:
                    return [null, Status.UNAUTHORIZED]
                case 404:
                    return [null, Status.NOT_FOUND]
                case 500:
                    return [null, Status.INTERNAL_SERVER_ERROR]
            }
        }
    }
    return [null, Status.UNKNOWN]
}

/**
 * This function is used to get the CSV content of the records of users entering or leaving the lab/center.
 *
 * @remarks
 * This function returns a tuple with the CSV content (`string`) and the status code (`Status`).
 * If the status code is `Status.OK` then the CSV content is valid, otherwise it is `null`.
 *
 * @param from_date - The date to start getting the records
 * @param to_date - The date to end getting the records
 * @returns `[string | null, Status]` - The CSV content or `null` if there was an error.
 *
 * @example
 * Simple example:
 * ```
 * import { GetService, Status } from 'lab-control';
 *
 * const [csv, status] = await GetService.GetCSVRegistro('2021-01-01', '2021-12-31'); // Get the records from 2021
 * if (status === Status.OK) {
 * console.log(csv);
 * }
 * ```
 */
export const GetCSVRegistro = async (
    from_date: string,
    to_date: string
): Promise<[string | null, Status]> => {
    try {
        const res = await axios.get(
            `/registros/reporte?from=${from_date}&to=${to_date}`
        )
        if (res.status === 200) return [res.data, Status.OK]
    } catch (error) {
        if (isAxiosError(error)) {
            switch (error.response?.status) {
                case 400:
                    return [null, Status.BAD_REQUEST]
                case 401:
                    return [null, Status.UNAUTHORIZED]
                case 404:
                    return [null, Status.NOT_FOUND]
                case 500:
                    return [null, Status.INTERNAL_SERVER_ERROR]
            }
        }
    }
    return [null, Status.UNKNOWN]
}
