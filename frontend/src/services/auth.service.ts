import ServiceTypes from './types';
import axios from 'axios';

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
            if (response.data.token) {
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
    localStorage.removeItem('user');
};

const getToken = () => {
    const user_local = localStorage.getItem('user');
    if (user_local) {
        const user = JSON.parse(user_local);
        if (user.token) {
            return { Authorization: 'Bearer ' + user.token };
        }
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
