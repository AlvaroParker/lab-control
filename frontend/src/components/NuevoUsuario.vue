<script lang="ts">
import { defineComponent } from 'vue';
// import AuthService from '../services/auth.service';
import { useRouter } from 'vue-router';
import ChileanRutify, { normalizeRut } from 'chilean-rutify';
import ServiceTypes from '../services/types.js';
import PostService from '../services/post.service.js';

export default defineComponent({
    data() {
        return {
            // We will store the user here
            usuario: {} as ServiceTypes.Usuario,
            valid_rut: true,
            registrando_huella: false,
            error_detected: false,
            message: '',
            stage: '',
            value: 10,
            total: 1,
            current: 0,
            rutRules: [
                (value: string) => {
                    let normalized = ChileanRutify.normalizeRut(value);
                    if (
                        typeof value === 'string' &&
                        ChileanRutify.validRut(value) &&
                        normalized &&
                        normalized.length > 7 &&
                        normalized.length < 10
                    )
                        return true;
                    return 'RUT no valido';
                },
            ],
        };
    },
    setup() {
        const router = useRouter();
        const go_home = () => {
            router.push({
                name: 'Home',
            });
        };
        return {
            go_home,
        };
    },
    methods: {
        async handleSubmit() {
            this.registrando_huella = true;
            const rut = normalizeRut(this.usuario.rut);
            if (ServiceTypes.isUsuario(this.usuario) && rut && this.valid_rut) {
                this.usuario.rut = rut;
                this.registrando_huella = true;

                const ws = await PostService.enrollNewUsuario(this.usuario);
                ws.onerror = (error) => {
                    this.error_detected = true;
                    console.log(error.message);
                    this.message = 'Error al registra huella.';
                };
                ws.onmessage = (data) => {
                    let status = JSON.parse(data.data as string);
                    this.total = status.total;
                    this.current = status.current;
                    this.stage = `Registrando huella... ${status.current} de ${status.total}`;
                };
                ws.onclose = (event) => {
                    switch (event.code) {
                        case 1000:
                            this.go_home();
                            break;
                        case 4000:
                            this.error_detected = true;
                            this.message = 'Email o RUT ya registrados';
                            break;
                        case 4001:
                            this.error_detected = true;
                            this.message = 'RUT invalido';
                            break;
                        case 4002:
                            this.error_detected = true;
                            this.message = 'Faltan campos por completar';
                            break;
                        case 4500:
                            this.error_detected = true;
                            this.message = 'Error agregando usuario. Intentelo de nuevo mas tarde.';
                            break;
                        default:
                            break;
                    }
                    this.registrando_huella = false;
                };
            } else {
                this.message = 'Faltan campos por llenar';
                this.registrando_huella = false;
            }
        },
    },
    watch: {
        rut() {
            const valid = ChileanRutify.validRut(this.usuario.rut);
            const normalized = ChileanRutify.normalizeRut(this.usuario.rut);

            if (valid && normalized && normalized.length > 7 && normalized.length < 10) {
                this.valid_rut = true;
            } else {
                this.valid_rut = false;
            }
        },
    },
});
</script>

<template>
    <section class="gradient-custom">
        <div class="container py-5 h-100">
            <div class="row d-flex justify-content-center align-items-center h-100">
                <div class="col-12 col-md-8 col-lg-6 col-xl-5">
                    <div class="card" style="border-radius: 1rem">
                        <div class="card-body p-5 text-center">
                            <form
                                class="mb-md-0 mt-md-0 pb-1 form-container"
                                @submit.prevent="handleSubmit"
                            >
                                <h2 class="fw-bold mb-1 text-uppercase">Nuevo usuario</h2>
                                <p class="text-50 mb-2">
                                    Ingresa los datos para registrar un nuevo usuarios
                                </p>

                                <div class="form-outline form-white mb-2">
                                    <v-text-field
                                        v-model="usuario.nombre"
                                        label="Nombre"
                                        required
                                        :disabled="registrando_huella"
                                    ></v-text-field>
                                </div>

                                <div class="form-outline form-white mb-2">
                                    <v-text-field
                                        v-model="usuario.apellido_1"
                                        label="Primer apellido"
                                        :disabled="registrando_huella"
                                        required
                                    ></v-text-field>
                                </div>

                                <div class="form-outline form-white mb-2">
                                    <v-text-field
                                        v-model="usuario.apellido_2"
                                        label="Segundo apellido"
                                        :disabled="registrando_huella"
                                        required
                                    ></v-text-field>
                                </div>

                                <div class="form-outline form-white mb-2">
                                    <v-text-field
                                        v-model="usuario.rut"
                                        label="RUT"
                                        :rules="rutRules"
                                        :disabled="registrando_huella"
                                        required
                                    ></v-text-field>
                                </div>
                                <div>
                                    <v-select
                                        :items="['Alumno', 'Ayudante', 'Docente']"
                                        density="comfortable"
                                        label="Rol"
                                        :disabled="registrando_huella"
                                        v-model="usuario.rol"
                                    ></v-select>
                                </div>

                                <div class="form-outline form-white mb-4">
                                    <v-text-field
                                        v-model="usuario.correo_uai"
                                        label="Correo"
                                        :disabled="registrando_huella"
                                        required
                                    ></v-text-field>
                                </div>

                                <p style="color: red" v-if="error_detected">
                                    {{ message }}
                                </p>
                                <button
                                    v-show="!registrando_huella"
                                    class="btn btn-primary btn-lg px-5 mt-2 mb"
                                    :disabled="registrando_huella"
                                >
                                    Registrar huella
                                </button>

                                <Teleport to="body">
                                    <Transition name="modal">
                                        <div v-if="registrando_huella" class="modal-mask">
                                            <div
                                                class="modal-container d-flex flex-column align-items-center justify-content-center rounded-5"
                                            >
                                                <h6 class="mb-4 justify-content-center text-center">
                                                    Registrando, ponga su huella sobre el lector
                                                </h6>
                                                <v-progress-circular
                                                    v-if="registrando_huella"
                                                    :rotate="-90"
                                                    :size="100"
                                                    :width="15"
                                                    :model-value="(current * 100) / total"
                                                    color="primary"
                                                >
                                                    {{ current }}
                                                </v-progress-circular>
                                            </div>
                                        </div>
                                    </Transition>
                                </Teleport>
                            </form>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </section>
</template>

<style>
.card {
    margin-top: 50px;
}

.modal-mask {
    position: fixed;
    z-index: 9998;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(0, 0, 0, 0.5);
    display: flex;
    transition: opacity 0.3s ease;
}

.modal-container {
    width: 300px;
    margin: auto;
    padding: 20px 30px;
    background-color: #fff;
    border-radius: 2px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.33);
    transition: all 0.3s ease;
}

.modal-header h3 {
    margin-top: 0;
    color: #42b983;
}

.modal-body {
    margin: 20px 0;
}

.modal-default-button {
    float: right;
}

/*
 * The following styles are auto-applied to elements with
 * transition="modal" when their visibility is toggled
 * by Vue.js.
 *
 * You can easily play with the modal transition by editing
 * these styles.
 */

.modal-enter-from {
    opacity: 0;
}

.modal-leave-to {
    opacity: 0;
}

.modal-enter-from .modal-container,
.modal-leave-to .modal-container {
    -webkit-transform: scale(1.1);
    transform: scale(1.1);
}
</style>
