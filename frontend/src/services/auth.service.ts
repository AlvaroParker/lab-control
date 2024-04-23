import ServiceTypes, { Status } from './types';
import axios from 'axios';
import Cookies from 'js-cookie';

axios.defaults.withCredentials = true;

export const login = async (
    email: string,
    pswd: string
): Promise<[ServiceTypes.Admin | null, Status]> => {
    try {
        const res = await axios.post(ServiceTypes.API_URL + '/admin/login', {
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

export const is_authenticated = async (): Promise<boolean> => {
    // This only check if it has a cookie that will be used to authenticated. If the cookie is no longer valid
    // Then the server will response with Unauthorized and this should be handled with a redirect
    const token = getToken();
    if (token) {
        return true;
    } else {
        return false;
    }
};

const getUser = async (): Promise<ServiceTypes.Admin | null> => {
    if (await is_authenticated()) {
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

const logout = async (): Promise<Status> => {
    try {
        await axios.post(ServiceTypes.API_URL + '/admin/logout');
        Cookies.remove('auth-cookie');
        localStorage.removeItem('user');

        return Status.OK;
    } catch (error) {
        return Status.UNKNOWN;
    }
};

const getToken = () => {
    // Check if cookie has expired
    let cookie = Cookies.get('auth-cookie'); // => 'value'
    if (cookie) {
        return { 'cookie-auth': cookie };
    }
    return undefined;
};

const AuthService = {
    login,
    getToken,
    getUser,
    logout,
};
export default AuthService;
