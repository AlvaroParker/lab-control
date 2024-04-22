import ServiceTypes, { Status } from './types';
import axios from 'axios';
import Cookies from 'js-cookie';
import { getURL } from '.';

axios.defaults.withCredentials = true;

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

        switch (res.status) {
            case 200:
                return [res.data, Status.OK];
            case 400:
                return [null, Status.BAD_REQUEST];
            case 401:
                return [null, Status.UNAUTHORIZED];
            case 404:
                return [null, Status.NOT_FOUND];
            case 500:
                return [null, Status.INTERNAL_SERVER_ERROR];
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

const GetUser = async (): Promise<ServiceTypes.Admin | null> => {
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

const Logout = async (): Promise<Status> => {
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

const AuthService = {
    Login,
    GetToken,
    GetUser,
    Logout
};
export default AuthService;
