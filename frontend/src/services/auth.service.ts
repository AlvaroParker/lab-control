import ServiceTypes from './types';
import axios from 'axios';
import Cookies from 'js-cookie';

axios.defaults.withCredentials = true;

export const login = async (
    email: string,
    pswd: string
): Promise<ServiceTypes.Admin | undefined> => {
    console.log(ServiceTypes.API_URL);
    return axios
        .post(ServiceTypes.API_URL + '/admin/login', {
            email,
            pswd,
        })
        .then((response) => {
            if (response.data.cookie) {
                const cookie = response.data.cookie;
                Cookies.set('auth-cookie', cookie, {
                    expires: 1,
                    SameSite: 'Lax',
                });
                delete response.data.cookie;
                localStorage.setItem('user', JSON.stringify(response.data));
            }

            return response.data;
        });
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

const getUser = async () => {
    if (await is_authenticated()) {
        const user = localStorage.getItem('user');
        if (user) {
            const user_js = JSON.parse(user);
            return user_js as ServiceTypes.Admin;
        } else {
            return undefined;
        }
    } else {
        return undefined;
    }
};

const logout = async () => {
    await axios.post(ServiceTypes.API_URL + '/admin/logout');
    Cookies.remove('auth-cookie');
    localStorage.removeItem('user');
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
