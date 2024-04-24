<script lang="ts">
// import ServiceTypes from '../services/types.js';
import { defineComponent } from 'vue';
import {useMotivoStore} from '../stores/MotivoStore'
import ChileanRutify from 'chilean-rutify';

import { DeleteService, PostService, Status } from 'lab-control';

export default defineComponent({
    data() {
        return {
            motivos: useMotivoStore(),
            formatear_rut: ChileanRutify.formatRut,
            showModal: false,
            selected: -1,
            showModalNuevo: false,
            inputMotivo: "",
            errorBadRequest: false,
        };
    },
    methods: {
        async agregarNuevoMotivo() {
            const res = await PostService.NewMotivo(this.inputMotivo);
            if (res === Status.BAD_REQUEST) {
                this.errorBadRequest = true;
            } else {
                this.motivos.update()
                this.showModalNuevo = false;
                this.inputMotivo = ""
            }
        },
        async eliminar(id: number) {
            const status = await DeleteService.DeleteMotivo(id);
            if (status !== Status.OK) {
                // TODO: Handle error
            }
            this.motivos.update()
            this.showModal = false;
            this.selected = -1;
        },
        select_motivo(id: number) {
            this.selected = id;
            this.showModal = true;
        }
    },
    async beforeMount() {
        this.motivos.update();
    },
    mounted() {
        this.motivos;
    },
});
</script>
<template>
    <div class="container">
        <div class="card-body text-center" style="margin-top: 50px">
            <h4 class="card-title">Motivos de entrada Lab</h4>
            <p class="card-text">Estos son los motivos de entrada al lab, estos pueden ser genericos como "Uso de ventana", ramos en especifico como "TICS200", nombres de profesores, etc.</p>
        </div>

        <div class="d-flex justify-content-between">
            <button
                class="btn btn-primary me-2"
            @click="showModalNuevo = true"
            ><font-awesome-icon :icon="['fa', 'plus']" />
                Nuevo motivo
            </button>
        </div>

        <div class="container-fluid table-responsive card mt-4">
            <table class="table table-bordered-outline text-center">
                <thead>
                    <tr>
                        <th scope="col">ID</th>
                        <th scope="col">Motivo</th>
                        <th scope="col">Eliminar</th>
                    </tr>
                </thead>
                <tbody v-if="motivos.getMotivos.length != 0" v-for="motivo in motivos.getMotivos">
                    <tr id="{{motivo.id}}">
                        <td>{{motivo.id}}</td>
                        <td>{{ motivo.motivo }}</td>
                        <td>
                            <a
                                class="btn btn-danger btn-space"
                                @click="() => select_motivo(motivo.id)"
                                ><font-awesome-icon :icon="['fa', 'trash']" />
                                Eliminar</a
                            >
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
                            Agregar motivo nuevo?
                        </div>
                        <p class="text-center text-danger" v-if="errorBadRequest">El motivo proporcionado ya existe o esta en blanco</p>
                        <div class="modal-footer justify-content-center mb-3">
                            <input v-model="inputMotivo" placeholder="Ingresa un nuevo motivo">
                        </div>

                        <div class="modal-footer justify-content-center">
                            <button
                                class="btn btn-primary modal-default-button me-5"
                                @click="agregarNuevoMotivo"
                            >
                                Agregar
                            </button>
                            <button
                                class="btn btn-danger modal-default-button"
                                @click="showModalNuevo = false; errorBadRequest = false; inputMotivo = ''"
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
                <div v-if="showModal" class="modal-mask">
                    <div class="modal-container border rounded-3">
                        <div class="modal-header justify-content-center mb-3">
                            Eliminar motivo?
                        </div>

                        <div class="modal-footer justify-content-center">
                            <button
                                class="btn btn-danger modal-default-button me-5"
                                @click="() => eliminar(selected)"
                            >
                                Eliminar
                            </button>
                            <button
                                class="btn btn-primary modal-default-button"
                                @click="showModal = false; selected = -1"
                            >
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
