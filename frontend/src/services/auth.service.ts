import ServiceTypes from './types';
import axios from 'axios';
import Cookies from 'js-cookie';

export const login = async (
    email: string,
    pswd: string
): Promise<ServiceTypes.Admin | undefined> => {
    return axios
        .post(ServiceTypes.API_URL + '/admin/login', {
            email,
            pswd,
        })
        .then((response) => {
            if (response.data.cookie) {
                const cookie = response.data.cookie;
                Cookies.set('auth-cookie', cookie, {expires: 1, sameSite: "none"});
                delete response.data.cookie;
                localStorage.setItem('user', JSON.stringify(response.data));
            }

            return response.data;
        });
};

export const is_authenticated = async (): Promise<boolean> => {
    const token = getToken();
    if (token) {
        try {
            let res = await axios.get(ServiceTypes.API_URL + '/admin/auth', { headers: token });
            return res.status === 200;
        } catch {
            return false;
        }
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
    await axios.post(ServiceTypes.API_URL + '/admin/logout', {}, {headers: getToken()});
    Cookies.remove('auth-cookie');
    localStorage.removeItem('user');
};

const getToken = () => {
    let cookie = Cookies.get('auth-cookie') // => 'value'
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
