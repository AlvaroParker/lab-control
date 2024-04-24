<script lang="ts">
// import ServiceTypes from '../services/types.js';
import { defineComponent } from 'vue';
import { useRolStore } from '../stores/RolStore'
import ChileanRutify from 'chilean-rutify';

import { DeleteService, PostService, Status } from 'lab-control';

export default defineComponent({
    data() {
        return {
            roles: useRolStore(),
            formatear_rut: ChileanRutify.formatRut,
            showModal: false,
            selected: -1,
            showModalNuevo: false,
            inputRol: "",
            errorBadRequest: false,
        };
    },
    methods: {
        async agregarNuevoRol() {
            const res = await PostService.NewRol(this.inputRol);
            if (res === Status.BAD_REQUEST) {
                this.errorBadRequest = true;
            } else {
                this.roles.update()
                this.showModalNuevo = false;
                this.inputRol = ""
            }
        },
        async eliminar(id: number) {
            const status = await  DeleteService.DeleteRol(id);
            if (status !== Status.OK) {
                // TODO: Handle error
            }
            this.roles.update()
            this.showModal = false;
            this.selected = -1;
        },
        select_rol(id: number) {
            this.selected = id;
            this.showModal = true;
        }
    },
    async beforeMount() {
        this.roles.update();
    },
    mounted() {
        this.roles;
    },
});
</script>
<template>
    <div class="container">
        <div class="card-body text-center" style="margin-top: 50px">
            <h4 class="card-title">Roles de usuarios del Lab</h4>
            <p class="card-text">Estos son los roles que puede tener un usuario del laboratorio. Posibles roles pueden
                ser alumno, docente, ayudante, practicante, etc.</p>
        </div>

        <div class="d-flex justify-content-between">
            <button class="btn btn-primary me-2" @click="showModalNuevo = true"><font-awesome-icon
                    :icon="['fa', 'plus']" />
                Nuevo rol
            </button>
        </div>

        <div class="container-fluid table-responsive card mt-4">
            <table class="table table-bordered-outline text-center">
                <thead>
                    <tr>
                        <th scope="col">ID</th>
                        <th scope="col">Rol</th>
                        <th scope="col">Eliminar</th>
                    </tr>
                </thead>
                <tbody v-if="roles.getRols.length != 0" v-for="rol in roles.getRols">
                    <tr id="{{rol.id}}">
                        <td>{{ rol.id }}</td>
                        <td>{{ rol.rol.charAt(0).toUpperCase() + rol.rol.slice(1) }}</td>
                        <td>
                            <a class="btn btn-danger btn-space" @click="() => select_rol(rol.id)"><font-awesome-icon
                                    :icon="['fa', 'trash']" />
                                Eliminar</a>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>

        <Teleport to="body">
            <Transition name="modal">
                <div v-if="showModalNuevo" class="modal-mask">
                    <div class="modal-container border rounded-3">
                        <div class="modal-header justify-content-center mb-3">
                            Agregar rol nuevo?
                        </div>
                        <p class="text-center text-danger" v-if="errorBadRequest">El rol proporcionado ya existe o esta
                            en blanco</p>
                        <div class="modal-footer justify-content-center mb-3">
                            <input v-model="inputRol" placeholder="Ingresa un nuevo rol">
                        </div>

                        <div class="modal-footer justify-content-center">
                            <button class="btn btn-primary modal-default-button me-5" @click="agregarNuevoRol">
                                Agregar
                            </button>
                            <button class="btn btn-danger modal-default-button"
                                @click="showModalNuevo = false; errorBadRequest = false; inputRol = ''">
                                Cancelar
                            </button>
                        </div>
                    </div>
                </div>
            </Transition>
        </Teleport>

        <Teleport to="body">
            <Transition name="modal">
                <div v-if="showModal" class="modal-mask">
                    <div class="modal-container border rounded-3">
                        <div class="modal-header justify-content-center mb-3">
                            Eliminar rol?
                        </div>

                        <div class="modal-footer justify-content-center">
                            <button class="btn btn-danger modal-default-button me-5" @click="() => eliminar(selected)">
                                Eliminar
                            </button>
                            <button class="btn btn-primary modal-default-button"
                                @click="showModal = false; selected = -1">
                                Cancelar
                            </button>
                        </div>
                    </div>
                </div>
            </Transition>
        </Teleport>
    </div>
</template>

<style></style>
