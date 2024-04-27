import ServiceTypes, { Status } from './types';
import axios from 'axios';
import Cookies from 'js-cookie';
import { getURL } from '.';

axios.defaults.withCredentials = true;

/**
 * This namespace contains all the functions that handle the authentication of the user.
 * Possible responses (`Status`) are:
 * - `Status.OK` (200)
 * - `Status.BAD_REQUEST` (400)
 * - `Status.UNAUTHORIZED` (401)
 * - `Status.NOT_FOUND` (404)
 * - `Status.INTERNAL_SERVER_ERROR` (500)
 * 
 * You should try to handle the `Status` of the request to give feedback to the user.
 */
namespace AuthService {

    /**
     * This function logs in a user given its email and password. If the user is successfully logged in,
     * a cookie will be stored in the browser for future requests and the user data will be stored in the local storage under the key `user`.
     * 
     * @param email - Email of the user
     * @param pswd - Password of the user
     * @returns `Status` - Status of the request
     * 
     * @example
     * Simple example:
     * ```
     * import { AuthService, Status } from 'lab-control';
     * 
     * const [user, status] = await AuthService.Login("some@email.com", "somePassword123");
     * if (status === Status.OK) {
     *   console.log("User logged in successfully");
     *   localStorage.getItem('user'); // This will return the user data
     *   console.log(user);
     * }
     * ```
     */
    export const Login = async (
        email: string,
        pswd: string
    ): Promise<[ServiceTypes.Admin | null, Status]> => {
        const api_url = getURL();
        try {
            const res = await axios.post(api_url + '/admin/login', {
                email,
                pswd,
            });
            if (res.status === 200) {
                delete res.data.cookie;
                localStorage.setItem('user', JSON.stringify(res.data));
                return [res.data, Status.OK];
            }
        } catch (error) {
            if (axios.isAxiosError(error)) {
                switch (error.response?.status) {
                    case 400:
                        return [null, Status.BAD_REQUEST];
                    case 401:
                        return [null, Status.UNAUTHORIZED];
                    case 404:
                        return [null, Status.NOT_FOUND];
                    case 500:
                        return [null, Status.INTERNAL_SERVER_ERROR];
                }
            }
        }
        return [null, Status.UNKNOWN];
    };

    /**
     * This function checks if the user is authenticated. It does this by checking if the user has a cookie that will be used to authenticate, 
     * it doesn't actually check if the auth cookie is valid.
     * 
     * @returns `boolean` - True if the user is authenticated, false otherwise
     */
    export const IsAuthenticated = async (): Promise<boolean> => {
        // This only check if it has a cookie that will be used to authenticated. If the cookie is no longer valid
        // Then the server will response with Unauthorized and this should be handled with a redirect
        const token = GetToken();
        if (token) {
            return true;
        } else {
            return false;
        }
    };

    /**
     * This function retrieves the user data from local storage if the user is authenticated. 
     * If the user is not authenticated, it will return null.
     * 
     * @returns `ServiceTypes.Admin | null` - The user data if the user is authenticated, null otherwise
     * 
     * @example
     * Simple example:
     * ```
     * import { AuthService } from 'lab-control';
     * 
     * const user = await AuthService.GetUser();
     * if (user) {
     *  console.log(user);
     * }
     * ```
     */
    export const GetUser = async (): Promise<ServiceTypes.Admin | null> => {
        if (await IsAuthenticated()) {
            const user = localStorage.getItem('user');
            if (user) {
                const user_js = JSON.parse(user);
                return user_js as ServiceTypes.Admin;
            } else {
                return null;
            }
        } else {
            return null;
        }
    };

    /**
     * This function logs out the user by deleting the auth cookie, the user data from local storage 
     * and sending a request to the server to logout.
     * 
     * @returns `Status` - Status of the request
     * 
     * @example
     * Simple example:
     * ```
     * import { AuthService, Status } from 'lab-control';
     * 
     * const status = await AuthService.Logout();
     * if (status === Status.OK) {
     *  console.log("User logged out successfully");
     * }
     * ```
     */
    export const Logout = async (): Promise<Status> => {
        const api_url = getURL();
        try {
            await axios.post(api_url + '/admin/logout');
            Cookies.remove('auth-cookie');
            localStorage.removeItem('user');

            return Status.OK;
        } catch (error) {
            return Status.UNKNOWN;
        }
    };

    const GetToken = () => {
        // Check if cookie has expired
        let cookie = Cookies.get('auth-cookie'); // => 'value'
        if (cookie) {
            return { 'cookie-auth': cookie };
        }
        return undefined;
    };
}

export default AuthService;
