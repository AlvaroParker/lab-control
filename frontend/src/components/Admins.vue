<script lang="ts">
// import ServiceTypes from '../services/types.js';
import { defineComponent } from 'vue';
import { useAdminStore } from '../stores/AdminStore';
import ChileanRutify from 'chilean-rutify';
import { deleteAdmin } from '../services/delete.service';
import AuthService from '../services/auth.service';
import ServiceTypes, { Status } from '../services/types';
import { changePassword } from '../services/post.service';

export default defineComponent({
    data() {
        return {
            admins: useAdminStore(),
            formatear_rut: ChileanRutify.formatRut,
            showModal: false,
            showModalError: false,
            showModalPswd: false,
            selected: {} as ServiceTypes.AdminGeneric,
            user: {} as ServiceTypes.Admin,
            inputPswd1: '',
            inputPswd2: '',
            missingPswd: false
        };
    },
    methods: {
        handleCambiar() {
            if (this.inputPswd1 === this.inputPswd2 && this.inputPswd1 !== '') 
                this.cambiarPswd()
            else 
                this.missingPswd = true
        },
        async eliminarAdmin(email: string) {
            if (this.user.email !== email) {
                const status = await deleteAdmin(email);
                if (status !== Status.OK) {
                    // TODO: Handle error
                }
                this.admins.update();
                this.selected = {} as ServiceTypes.AdminGeneric;
                this.showModal = false;
            }
        },
        async getUser() {
            const user = await AuthService.getUser();
            if (user) {
                this.user = user;
            }
        },
        async cambiarPswd() {
            if (this.inputPswd1 === this.inputPswd2)
            {
                const status = await changePassword(this.selected.email, this.inputPswd1);
                if (status !== Status.OK) {
                    // TODO: Handle error on password change
                }
            }
            this.showModalPswd = false;
            this.selected = {} as ServiceTypes.AdminGeneric;
        }
    },
    async beforeMount() {
        this.admins.update();
        this.getUser();
    },
    mounted() {
        this.admins;
    },
});
</script>
<template>
    <div class="container" v-if="admins.admins.length !== 0">
        <div class="card-body text-center" style="margin-top: 50px">
            <h4 class="card-title">Admins del Lab</h4>
            <p class="card-text">Listado de adminstradores autorizados en el LAB</p>
        </div>
        <div class="container-fluid table-responsive card mt-4">
            <table class="table table-bordered-outline text-center">
                <thead>
                    <tr>
                        <th scope="col">Nombre</th>
                        <th scope="col">Correo</th>
                        <th scope="col">Eliminar</th>
                        <th scope="col">Cambiar contrasena</th>
                    </tr>
                </thead>
                <tbody v-for="admin in admins.getAdmins">
                    <tr id="{{ admin.email }}">
                        <td>{{ admin.nombre }} {{ admin.apellido_1 }} {{ admin.apellido_2 }}</td>
                        <td>
                            {{ admin.email }}
                        </td>
                        <td>
                            <button class="btn btn-danger btn-space" @click="if (admin.email == user.email) {showModalError = true} else {showModal = true; selected=admin}"
                                ><font-awesome-icon :icon="['fa', 'trash']" />
                                Eliminar </button>
                        </td>
                        <td>
                            <button class="btn btn-warning btn-space" @click="showModalPswd = true; selected = admin"
                                ><font-awesome-icon :icon="['fa', 'pencil-alt']" />
                                Cambiar </button>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
    </div>

    <Teleport to="body">
        <Transition name="modal">
            <div v-if="showModal" class="modal-mask">
                <div class="modal-container border rounded-3">
                    <div class="modal-header justify-content-center mb-3">
                        Eliminar admin?
                    </div>

                    <div class="modal-footer justify-content-center">
                        <button
                            class="btn btn-danger modal-default-button me-5"
                            @click="() => eliminarAdmin(selected.email)"
                        >
                            Eliminar
                        </button>
                        <button
                            class="btn btn-primary modal-default-button"
                            @click="showModal = false; selected = {} as ServiceTypes.AdminGeneric"
                        >
                            Cancelar
                        </button>
                    </div>
                </div>
            </div>
        </Transition>
    </Teleport>
    <Teleport to="body">
        <Transition name="modal">
            <div v-if="showModalError" class="modal-mask">
                <div class="modal-container border rounded-3">
                    <div class="modal-header justify-content-center mb-3 text-center text-red">
                        <p><font-awesome-icon :icon="['fa', 'exclamation-triangle']" />
                        No puedes eliminar tu propio usuario!</p>
                    </div>

                    <div class="modal-footer justify-content-center">
                        <button
                            class="btn btn-primary modal-default-button"
                            @click="showModalError = false; selected = {} as ServiceTypes.AdminGeneric"
                        >
                            Cancelar
                        </button>
                    </div>
                </div>
            </div>
        </Transition>
    </Teleport>
    <Teleport to="body">
        <Transition name="modal">
            <div v-if="showModalPswd" class="modal-mask">
                <div class="modal-container border rounded-3">
                    <div class="modal-header justify-content-center mb-3 text-center">
                        Ingrese la nueva contrasena para {{ selected.nombre }} {{ selected.apellido_1 }} {{ selected.apellido_2 }}
                        ({{ selected.email }})
                    </div>
                    <div class="modal-footer justify-content-center my-5" v-if="missingPswd">
                        <p class="text-red">
                    <font-awesome-icon :icon="['fa', 'exclamation-triangle']" />
                            Contrasenas no coinciden</p>
                    </div>
                    <div class="modal-footer justify-content-center mb-5">
                        <input v-model="inputPswd1" placeholder="Ingrese contrasena" type="password"  required>
                    </div>

                    <div class="modal-footer justify-content-center my-5">
                        <input v-model="inputPswd2" placeholder="Ingrese contrasena" type="password" required>
                    </div>
                    <div class="modal-footer justify-content-center">
                        <button
                            class="btn btn-danger modal-default-button me-5"
                            @click="handleCambiar"
                        >
                            Cambiar
                        </button>
                        <button
                            class="btn btn-primary modal-default-button"
                            @click="showModalPswd = false; selected = {} as ServiceTypes.AdminGeneric; missingPswd = false"
                        >
                            Cancelar
                        </button>
                    </div>
                </div>
            </div>
        </Transition>
    </Teleport>

</template>

<style></style>
