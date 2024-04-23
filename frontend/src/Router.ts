import 'vue-router';
import { createRouter, createWebHistory } from 'vue-router';
import { is_authenticated } from './services/auth.service';
import Usuarios from './components/Usuarios.vue';
import Login from './components/Login.vue';
import Info from './components/Info.vue';
import NuevoUsuario from './components/NuevoUsuario.vue';
import Verificar from './components/Verificar.vue';
import Registro from './components/Registros.vue';
import EditUsuario from './components/EditUsuario.vue';
import QRCode from './components/QRReader/QRCode.vue';
import NuevoAdmin from './components/NuevoAdmin.vue';
import Motivos from './components/Motivos.vue';
import Admins from './components/Admins.vue';
import Roles from './components/Roles.vue';

const routes = [
    { path: '/', component: Usuarios, name: 'Home' },
    { path: '/login', component: Login, name: 'Login' },
    { path: '/usuarios', component: Info, name: 'Info' },
    { path: '/usuarios/reader', component: QRCode, name: 'QRReader' },
    { path: '/usuarios/edit', component: EditUsuario, name: 'EditUsuario' },
    { path: '/usuarios/nuevo', component: NuevoUsuario, name: 'NuevoUsuario' },
    { path: '/usuarios/verificar', component: Verificar, name: 'VerificarUsuario' },
    { path: '/registros', component: Registro, name: 'Registro' },
    { path: '/admin/new', component: NuevoAdmin, name: 'NuevoAdmin'},
    { path: '/motivos', component: Motivos, name: 'Motivos'},
    { path: '/roles', component: Roles, name: 'Roles'},
    { path: '/admin', component:Admins, name: 'Admin'}
];

export const router = createRouter({
    history: createWebHistory(),
    routes,
});

router.beforeEach(async (to, _from) => {
    const is_auth = await is_authenticated();
    if (
        !is_auth &&
        to.name !== 'Login' &&
        to.name !== 'VerificarUsuario' &&
        to.name !== 'QRReader'
    ) {
        return { name: 'Login', forceReload: true };
    } else if (is_auth && to.name === 'Login') {
        return { name: 'Home' };
    }
});

router.beforeResolve((to, _from, next) => {
    if (to.meta.forceReload) {
        window.location.reload();
    } else {
        next();
    }
});
