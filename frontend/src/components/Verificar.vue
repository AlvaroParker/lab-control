<script lang="ts">
import { defineComponent } from 'vue';
import ServiceTypes from '../services/types';
import GetService from '../services/get.service';
import ChileanRutify from 'chilean-rutify';
import { AxiosError } from 'axios';

export default defineComponent({
    data() {
        return {
            usuario: {} as ServiceTypes.Usuario,
            ultimo_registro: {} as ServiceTypes.Registro,
            format_rut: ChileanRutify.formatRut,
            display: false,
            sensor_error: false,
            not_found: false,
            disabled: false,
            showModal: false,
            esperando_huella: false,
        };
    },
    setup() {},
    methods: {
        getRegistro() {},
        handleSubmit(salida: boolean, motivo: string) {
            this.esperando_huella = true;
            this.showModal = false;
            const timeout = 2000;
            this.disabled = true;
            GetService.verifyUsuario(salida, motivo)
                .then((res) => {
                    if (res?.data) {
                        this.esperando_huella = false;
                        this.usuario = res.data;
                        this.display = true;
                        console.log(this.usuario);
                        setTimeout(() => {
                            this.display = false;
                            this.usuario = {} as ServiceTypes.Usuario;
                            this.disabled = false;
                        }, timeout);
                    } else {
                        this.esperando_huella = true;
                        this.usuario = {} as ServiceTypes.Usuario;
                        this.not_found = true;
                        this.not_found = true;
                        this.display = true;
                        setTimeout(() => {
                            this.display = false;
                            this.usuario = {} as ServiceTypes.Usuario;
                            this.not_found = false;
                            this.disabled = false;
                        }, timeout);
                    }
                })
                .catch((err) => {
                    this.esperando_huella = false;
                    this.usuario = {} as ServiceTypes.Usuario;
                    const axios_err = err as AxiosError;
                    if (axios_err.response?.status === 404) {
                        this.display = true;
                        this.disabled = true;
                        this.not_found = true;
                        setTimeout(() => {
                            this.display = false;
                            this.disabled = false;
                            this.not_found = false;
                        }, timeout);
                    } else {
                        this.display = false;
                        this.sensor_error = true;
                    }
                });
        },
    },
    beforeMount() {},
});
</script>
<template>
    <div class="container vh-100 d-flex align-items-center justify-content-center">
        <div class="card shadow-2-card w-100 mb-5" id="show-alumno">
            <div class="card-body" v-if="!usuario.nombre && display && not_found">
                <h5 class="card-title text-center">Usuario no encontrado</h5>
            </div>
            <div class="card-body" v-if="sensor_error">
                <h5 class="card-title text-center">Sensor de huella no disponible</h5>
            </div>
            <div class="d-flex justify-content-center mt-3 mb-3 btn-group blocks">
                <button
                    class="btn btn-warning col-2 custom-btn-height"
                    :disabled="disabled"
                    @click.prevent="() => (showModal = true)"
                >
                    Marcar entrada
                </button>
                <button
                    class="btn btn-success col-2 custom-btn-height"
                    :disabled="disabled"
                    @click.prevent="() => handleSubmit(true, 'salida')"
                >
                    Marcar salida
                </button>
            </div>
        </div>
    </div>

    <Teleport to="body">
        <Transition name="modal">
            <div v-if="esperando_huella" class="modal-mask">
                <div
                    class="modal-container d-flex flex-column align-items-center justify-content-center rounded-5"
                >
                    <h6 class="mb-4 justify-content-center text-center">
                        Esperando huella, ponga su huella en el lector.
                    </h6>
                    <v-progress-circular
                        :size="50"
                        color="primary"
                        indeterminate
                        v-if="esperando_huella"
                    ></v-progress-circular>
                </div>
            </div>
        </Transition>
    </Teleport>

    <Teleport to="body">
        <Transition name="fade">
            <div class="card shadow-2-card mt-3 modal-mask" v-if="usuario.nombre && display">
                <div class="card-body">
                    <h5 class="card-title text-center">Usuario</h5>
                    <ul class="list-group list-group-flush">
                        <li class="list-group-item">
                            Nombre: {{ usuario.nombre }} {{ usuario.apellido_1 }}
                            {{ usuario.apellido_2 }}
                        </li>
                        <li class="list-group-item">Rut: {{ format_rut(usuario.rut) }}</li>
                        <li class="list-group-item">Correo: {{ usuario.correo_uai }}</li>
                    </ul>
                </div>
            </div>
        </Transition>
    </Teleport>

    <Teleport to="body">
        <Transition name="modal">
            <div v-if="showModal" class="modal-mask">
                <div class="container d-flex justify-content-center">
                    <div class="d-flex row mx-5 justify-content-center align-items-center">
                        <div>
                            <button
                                class="btn btn-success btn-space btn-lg mb-5"
                                :disabled="disabled"
                                @click.prevent="() => handleSubmit(false, 'ventana')"
                                style="width: 30rem !important;height: 25rem !important;"
                            >
                                Ventana
                            </button>
                            <button
                                class="btn btn-success btn-space btn-lg mt-5"
                                :disabled="disabled"
                                @click.prevent="() => handleSubmit(false, 'uso libre')"
                                style="width: 30rem !important;height: 25rem !important;"
                            >
                                Uso Libre
                            </button>
                        </div>
                    </div>
                    <div class="d-flex row mx-5 justify-content-center align-items-center">
                        <div>
                            <button
                                class="btn btn-success btn-space btn-lg mb-5"
                                :disabled="disabled"
                                @click.prevent="() => handleSubmit(false, 'ramo')"
                                    style="width: 30rem !important;height: 25rem !important;"
                            >
                                Asistencia a ramo
                            </button>
                            <button
                                class="btn btn-success btn-space btn-lg mt-5"
                                :disabled="disabled"
                                @click.prevent="() => handleSubmit(false, 'investigacion')"
                                    style="width: 30rem !important;height: 25rem !important;"
                            >
                                Investigacion
                            </button>
                        </div>
                    </div>

                </div>
            </div>
        </Transition>
    </Teleport>
</template>
<style>
.blocks .btn {
    padding: 24px 12px;
    margin: 0 5px;
    border-radius: 20;
}

.custom-btn-height {
    height: 125px;
    /* Define your desired height here */
}
</style>
