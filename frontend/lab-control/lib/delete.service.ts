import { getURL } from '.';
import { Status } from './types';
import axios from 'axios';

axios.defaults.withCredentials = true;

/**
 * This namespace contains all the functions that make DELETE requests to the server.
 * Possible responses (`Status`) are:
 * - `Status.OK` (200)
 * - `Status.BAD_REQUEST` (400)
 * - `Status.UNAUTHORIZED` (401)
 * - `Status.NOT_FOUND` (404)
 * - `Status.INTERNAL_SERVER_ERROR` (500)
 * 
 * You should try to handle the `Status` of the request to give feedback to the user.
 */
namespace DeleteService {
    /**
     * This function deletes a user from the database given its rut.
     * 
     * - If the user doesn't exists, it will return `Status.NOT_FOUND`
     * - If the user does exists, it will return `Status.OK`
     * 
     * @param rut - Rut of the user to delete
     * @returns `Status` - Status of the request
     * 
     * @example
     * Simple example:
     * ```
     * import { DeleteService, Status } from 'lab-control';
     * 
     * const status = await DeleteService.DeleteUsuario("12345678-9");
     * if (status === Status.OK) {
     *    console.log("User deleted successfully");
     * } else {
     *   console.log("Error deleting user"); 
     * }
     * ```
     */
    export const DeleteUsuario = async (rut: string): Promise<Status> => {
        const api_url = getURL();
        try {
            const res = await axios.delete(api_url + `/usuarios/${rut}`);
            if (res.status === 200)
                return Status.OK
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

    /**
     * This function deletes a `motivo` from the database given its id.
     * 
     * - If the `motivo` doesn't exists, it will return `Status.NOT_FOUND`
     * - If the `motivo` does exists, it will return `Status.OK`
     * 
     * @param id - Id of the motivo to delete
     * @returns `Status` - Status of the request
     * 
     * @example
     * Simple example:
     * ```
     * import { DeleteService, Status } from 'lab-control';
     * 
     * const status = await DeleteService.DeleteMotivo(1);
     * if (status === Status.OK) {
     *   console.log("Motivo deleted successfully");
     * } else {
     *  console.log("Error deleting motivo");
     * }
     * ```
     */
    export const DeleteMotivo = async (id: number): Promise<Status> => {
        const api_url = getURL();
        try {
            const res = await axios.delete(api_url + `/metadata/motivos/${id}`);
            if (res.status === 200)
                return Status.OK
        } catch (error) {
            if (axios.isAxiosError(error)) {
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

    /**
     * This function deletes a `rol` from the database given its id.
     * 
     * - If the rol doesn't exists, it will return `Status.NOT_FOUND`
     * - If the rol does exists, it will return `Status.OK`
     * 
     * @param id - Id of the rol to delete
     * @returns `Status` - Status of the request
     * 
     * @example
     * Simple example:
     * ```
     * import { DeleteService, Status } from 'lab-control';
     * 
     * const status = await DeleteService.DeleteRol(1);
     * if (status === Status.OK) {
     *  console.log("Rol deleted successfully");
     * } else {
     * console.log("Error deleting rol");
     * }
     * ```
     */
    export const DeleteRol = async (id: number): Promise<Status> => {
        const api_url = getURL();
        try {
            const res = await axios.delete(api_url + `/metadata/roles/${id}`);
            if (res.status === 200)
                return Status.OK
        } catch (error) {
            if (axios.isAxiosError(error)) {
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

    /**
     * This function deletes an admin from the database given its email.
     * 
     * - If the admin doesn't exists, it will return `Status.NOT_FOUND`
     * - If the admin does exists, it will return `Status.OK`
     * 
     * @param email - The email of the admin to delete
     * @returns `Status` - Status of the request
     * 
     * @example
     * Simple example:
     * ```
     * import { DeleteService, Status } from 'lab-control';
     * 
     * const status = await DeleteService.DeleteAdmin("some@email.com");
     * switch (status) {
     *  case Status.OK:
     *   console.log("Admin deleted successfully");
     *   break;
     *  case Status.NOT_FOUND:
     *   console.log("Admin not found");
     *   break;
     *  default:
     *   console.log("Error deleting admin");
     * }
     * ```
     */
    export const DeleteAdmin = async (email: string): Promise<Status> => {
        const api_url = getURL();
        try {
            const res = await axios.post(api_url + `/admin/delete`, {
                email: email
            });
            if (res.status === 200)
                return Status.OK
        } catch (error) {
            if (axios.isAxiosError(error)) {
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
}

export default DeleteService;
