<script lang="ts">
import { defineComponent } from 'vue';
// import AuthService from '../services/auth.service';
import PostService from '../services/post.service.js';
import { useRouter } from 'vue-router';
import ChileanRutify, { normalizeRut } from 'chilean-rutify';
import { AxiosError } from 'axios';
import ServiceTypes from '../services/types.js';

export default defineComponent({
    data() {
        return {
            // We will store the user here
            usuario: {} as ServiceTypes.Usuario,
            valid_rut: true,
            registrando_huella: false,
            user_exists_err: false,
            message: '',
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
                try {
                    const res = await PostService.enrollUsuario(this.usuario);
                    switch (res.status) {
                        case 200:
                            this.go_home();
                            break;

                        default:
                            console.log(res);
                            console.log('Error while enrolling');
                            break;
                    }
                    this.registrando_huella = false;
                } catch (err: any) {
                    const axios_error = err as AxiosError;
                    switch (axios_error.response?.status) {
                        case 500:
                            this.message = 'Sensor de huella no disponible.';
                            break;
                        case 409:
                            this.message = 'RUT o Email ya se encuentra registrado.';
                            break;
                        default:
                            this.message =
                                'Error al intentar crear nuevo usuario. Intentelo de nuevo mas tarde.';
                            console.log(axios_error);
                            break;
                    }
                    this.registrando_huella = false;
                }
            } else {
                this.message = 'Faltan campos por llenar';
                this.registrando_huella = false;
            }
        },
        handleRut() {
            const valid = ChileanRutify.validRut(this.usuario.rut);
            const normalized = ChileanRutify.normalizeRut(this.usuario.rut);

            if (valid && normalized && normalized.length > 7 && normalized.length < 10) {
                this.valid_rut = true;
            } else {
                this.valid_rut = false;
            }
        },
    },
    watch: {
        rut() {
            this.handleRut();
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
                                    Ingresa los datos para registrar un nuevo usuario
                                </p>

                                <div class="form-outline form-white mb-2">
                                    <v-text-field
                                        v-model="usuario.nombre"
                                        label="Nombre"
                                        required
                                    ></v-text-field>
                                </div>

                                <div class="form-outline form-white mb-2">
                                    <v-text-field
                                        v-model="usuario.apellido_1"
                                        label="Primer apellido"
                                        required
                                    ></v-text-field>
                                </div>

                                <div class="form-outline form-white mb-2">
                                    <v-text-field
                                        v-model="usuario.apellido_2"
                                        label="Segundo apellido"
                                        required
                                    ></v-text-field>
                                </div>

                                <div class="form-outline form-white mb-2">
                                    <v-text-field
                                        v-model="usuario.rut"
                                        label="RUT"
                                        :rules="rutRules"
                                        required
                                    ></v-text-field>
                                </div>
                                <div>
                                    <v-select
                                        :items="['Alumno', 'Ayudante', 'Docente']"
                                        density="comfortable"
                                        label="Rol"
                                        v-model="usuario.rol"
                                    ></v-select>
                                </div>

                                <div class="form-outline form-white mb-4">
                                    <v-text-field
                                        v-model="usuario.correo_uai"
                                        label="Correo"
                                        required
                                    ></v-text-field>
                                </div>

                                <p style="color: red" v-if="user_exists_err">
                                    {{ message }}
                                </p>
                                <button
                                    class="btn btn-primary btn-lg px-5 mt-2 mb"
                                    :disabled="registrando_huella"
                                >
                                    Registrar huella
                                </button>
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
</style>
