<script lang="ts">
import { defineComponent } from 'vue';
import ChileanRutify from 'chilean-rutify';
import { useRoute } from 'vue-router';
import { GetService, ServiceTypes, Status } from 'lab-control';
import { ref } from 'vue';

export default defineComponent({
    data() {
        return {
            usuarios: ref(new Array<ServiceTypes.Usuario>()),
            done: false,
            formatear_rut: ChileanRutify.formatRut,
        };
    },
    watch: {
        '$route.query': {
            immediate: true, // to call on first load
            async handler() {
                await this.search();
            },
        },
    },
    methods: {
        async search() {
            const route = useRoute().query.query;
            const [searchResult, status] = await GetService.Search(route);
            if (status == Status.OK) {
                this.usuarios.value = searchResult;
                return;
            }
            this.usuarios.value = new Array<ServiceTypes.Usuario>();
        },
    },
    async beforeMount() {},
    mounted() {
        this.usuarios;
    },
});
</script>
<template>
    <div class="container" v-if="usuarios.value && usuarios.value.length !== 0">
        <div class="card-body text-center" style="margin-top: 50px">
            <h4 class="card-title">Usuarios Lab</h4>
            <p class="card-text">Listado de usuarios autorizados para entrar al lab</p>
        </div>
        <div class="container-fluid table-responsive card mt-4">
            <table class="table table-bordered-outline text-center">
                <thead>
                    <tr>
                        <th scope="col">ID</th>
                        <th scope="col">RUT</th>
                        <th scope="col">Nombre</th>
                        <th scope="col">Correo</th>
                        <th scope="col">Rol</th>
                        <th scope="col">...</th>
                    </tr>
                </thead>
                <tbody v-for="usuario in usuarios.value">
                    <tr id="{{ usuario.rut }}">
                        <td>{{ usuario.rut }}</td>
                        <td>{{ formatear_rut(usuario.rut) }}</td>
                        <td>
                            {{ usuario.nombre }} {{ usuario.apellido_1 }} {{ usuario.apellido_2 }}
                        </td>
                        <td>{{ usuario.correo_uai }}</td>
                        <td class="text-center">
                            {{ usuario.rol.charAt(0).toUpperCase() + usuario.rol.slice(1) }}
                        </td>
                        <td>
                            <router-link
                                class="btn btn-primary btn-space"
                                :to="'usuarios?rut=' + usuario.rut"
                                ><font-awesome-icon :icon="['fa', 'search']" />
                                Detalles...</router-link
                            >
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
    </div>
</template>

<style></style>
