<script lang="ts">
import { defineComponent } from 'vue';
import { AuthService, ServiceTypes } from 'lab-control';
import { useRoute } from 'vue-router';

export default defineComponent({
    data() {
        return {
            user: {} as ServiceTypes.Admin,
            query: '',
        };
    },
    watch: {
        $route: {
            immediate: true, // to call on first load
            async handler(newRoute) {
                if (newRoute.name !== 'Search' && newRoute.name !== 'Info') {
                    this.query = '';
                }
            },
        },
    },
    methods: {
        getName() {
            return this.$route.name;
        },
        async getUser() {
            const user = await AuthService.GetUser();
            if (user) {
                this.user = user;
            }
        },
        async logout() {
            await AuthService.Logout();
            this.$router.push('/').then(() => {
                location.reload();
            });
        },
        async onSearch() {
            // Navigate to search page
            this.$router.push({ name: 'Search', query: { query: this.query } });
        },
    },
    beforeMount() {
        this.getUser();
    },
});
</script>
<template>
    <header class="container pb-5">
        <!-- Sidebar -->
        <nav id="sidebarMenu" class="collapse d-lg-block sidebar collapse bg-white">
            <div class="position-sticky">
                <div class="list-group list-group-flush mx-3 mt-4">
                    <router-link
                        to="/registros"
                        class="list-group-item list-group-item-action py-2 ripple"
                        :class="{ active: 'Registro' == getName() }"
                        aria-current="true"
                    >
                        <font-awesome-icon :icon="['fa', 'book']" /> Accesos
                    </router-link>

                    <router-link
                        to="/usuarios/nuevo"
                        class="list-group-item list-group-item-action py-2 ripple"
                        :class="{ active: 'NuevoUsuario' == getName() }"
                    >
                        <font-awesome-icon :icon="['fa', 'plus']" /> Nuevo Usuario
                    </router-link>

                    <router-link
                        to="/"
                        class="list-group-item list-group-item-action py-2 ripple"
                        :class="{
                            active:
                                'Home' == getName() ||
                                'Info' == getName() ||
                                'EditUsuario' == getName(),
                        }"
                    >
                        <font-awesome-icon :icon="['fa', 'book']" /> Usuarios
                    </router-link>

                    <router-link
                        to="/usuarios/verificar"
                        class="list-group-item list-group-item-action py-2 ripple"
                        :class="{ active: 'VerificarUsuario' == getName() }"
                    >
                        <font-awesome-icon :icon="['fa', 'check']" /> Verificar</router-link
                    >

                    <router-link
                        to="/usuarios/reader"
                        class="list-group-item list-group-item-action py-2 ripple"
                        :class="{ active: 'QRReader' == getName() }"
                    >
                        <font-awesome-icon :icon="['fa', 'qrcode']" /> QR Reader
                    </router-link>

                    <router-link
                        to="/admin/new"
                        class="list-group-item list-group-item-action py-2 ripple"
                        :class="{ active: 'NuevoAdmin' == getName() }"
                    >
                        <font-awesome-icon :icon="['fa', 'plus']" /> Nuevo Admin
                    </router-link>
                    <router-link
                        to="/motivos"
                        class="list-group-item list-group-item-action py-2 ripple"
                        :class="{ active: 'Motivos' == getName() }"
                    >
                        <font-awesome-icon :icon="['fa', 'book']" /> Administrar motivos
                    </router-link>

                    <router-link
                        to="/roles"
                        class="list-group-item list-group-item-action py-2 ripple"
                        :class="{ active: 'Roles' == getName() }"
                    >
                        <font-awesome-icon :icon="['fa', 'book']" /> Administrar roles
                    </router-link>
                    <router-link
                        to="/admin"
                        class="list-group-item list-group-item-action py-2 ripple"
                        :class="{ active: 'Admin' == getName() }"
                    >
                        <font-awesome-icon :icon="['fa', 'lock']" /> Administradores
                    </router-link>
                </div>
            </div>
        </nav>

        <!-- Sidebar -->
        <nav id="main-navbar" class="navbar navbar-expand-lg navbar-dark bg-dark fixed-top">
            <div class="container-fluid">
                <button
                    class="navbar-toggler"
                    type="button"
                    data-mdb-toggle="collapse"
                    data-mdb-target="#sidebarMenu"
                    aria-controls="sidebarMenu"
                    aria-expanded="false"
                    aria-label="Toggle navigation"
                ></button>

                <a class="navbar-brand" href="#">
                    <img src="/public/uai_logo.gif" height="35" alt="" />
                </a>
                <form
                    class="d-none d-md-flex input-group w-auto my-auto"
                    @submit.prevent="onSearch"
                >
                    <input
                        v-model="query"
                        autocomplete="off"
                        type="search"
                        class="form-control rounded bg-dark text-white"
                        placeholder="Buscar alumno"
                        style="min-width: 225px"
                    />
                </form>

                <ul class="navbar-nav ms-auto mb-2 mb-lg-0">
                    <li class="nav-item dropdown">
                        <a
                            :on-click="getName"
                            class="nav-link dropdown-toggle text-white"
                            id="navbarDropdown1"
                            role="button"
                            data-bs-toggle="dropdown"
                            aria-expanded="false"
                        >
                            <font-awesome-icon :icon="['fa', 'user-circle']" />
                        </a>
                        <ul
                            class="dropdown-menu dropdown-menu-end"
                            aria-labelledby="navbarDropdown1"
                            id="user-area"
                        >
                            <li class="dropdown-item">
                                {{ user.nombre }} {{ user.apellido_1 }} {{ user.apellido_2 }}
                                <p style="color: grey">{{ user.email }}</p>
                            </li>
                            <li>
                                <hr class="dropdown-divider" />
                            </li>
                            <li class="dropdown-item btn" @click="logout">
                                <font-awesome-icon :icon="['fa', 'sign-out-alt']" /> Cerrar sesion
                            </li>
                        </ul>
                    </li>
                </ul>
            </div>
        </nav>
    </header>
</template>

<style>
body {
    background-color: #fbfbfb;
}

@media (min-width: 991.98px) {
    main {
        padding-left: 240px;
    }
}

/* Sidebar */
.sidebar {
    position: fixed;
    top: 0;
    bottom: 0;
    left: 0;
    padding: 58px 0 0;
    /* Height of navbar */
    box-shadow:
        0 2px 5px 0 rgb(0 0 0 / 5%),
        0 2px 10px 0 rgb(0 0 0 / 5%);
    width: 240px;
    z-index: 600;
}

@media (max-width: 991.98px) {
    .sidebar {
        width: 100%;
    }
}

.sidebar .active {
    border-radius: 5px;
    box-shadow:
        0 2px 5px 0 rgb(0 0 0 / 16%),
        0 2px 10px 0 rgb(0 0 0 / 12%);
}

.sidebar-sticky {
    position: relative;
    top: 0;
    height: calc(100vh - 48px);
    padding-top: 0.5rem;
    overflow-x: hidden;
    overflow-y: auto;
    /* Scrollable contents if viewport is shorter than content. */
}

.form-control::placeholder {
    /* Chrome, Firefox, Opera, Safari 10.1+ */
    color: rgb(67, 67, 67);
    opacity: 1;
    /* Firefox */
}
</style>
