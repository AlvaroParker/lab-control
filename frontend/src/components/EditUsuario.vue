<script lang="ts">
import { defineComponent } from 'vue';
// import AuthService from '../services/auth.service';
import { useRouter } from 'vue-router';
import ChileanRutify, { formatRut } from 'chilean-rutify';
import { AxiosError } from 'axios';

import { GetService, ServiceTypes, PostService, Status } from 'lab-control';

export default defineComponent({
    data() {
        return {
            usuario: {} as ServiceTypes.Usuario, // The original usuario
            edit_usuario: {} as ServiceTypes.Usuario, // Object where we will store the edited fields
            valid_rut: true, // Check if rut is valid, we won't be able to submit the form if this is set to false
            formatRut, // Format rut function
            error_message: '',
            rutRules: [
                // Rut rules to display a warning when is not valid
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
        // Router to go home after edit
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
    async beforeMount() {
        // Get the user before mounting the component
        const rut = this.$route.query.rut;
        if (rut && typeof rut === 'string') {
            const [usuario, status] = await GetService.GetUsuarioByRut(rut);
            if (status === Status.OK && usuario) {
                this.usuario = usuario;
            }
        }
    },
    methods: {
        async handleSubmit() {
            // If this is only white space, then pass an empty string, if not, try to normalize rut and assign this.nuevo_rut in case of failure
            const rut =
                !this.edit_usuario.rut || this.edit_usuario.rut.trim() === ''
                    ? ''
                    : ChileanRutify.normalizeRut(this.edit_usuario.rut) ?? '';
            // Set the rut to edit_usuario object
            this.edit_usuario.rut = rut;
            // Make the PUT request
            try {
                const res = await PostService.EditUsuario(this.edit_usuario, this.usuario.rut);
                if (res === Status.OK) {
                    this.go_home();
                } else {
                    this.error_message = 'Error al publicar las ediciones.';
                }
            } catch (e) {
                console.log(e);
                const axios_err = e as AxiosError;
                if (axios_err.response?.status === 500) {
                    this.error_message = 'Error al publicar las ediciones.';
                } else {
                    this.error_message = 'Error desconocido. Intentelo de nuevo mas tarde.';
                }
            }
        },
        handleRut() {
            const valid = ChileanRutify.validRut(this.edit_usuario.rut);
            const normalized = ChileanRutify.normalizeRut(this.edit_usuario.rut);

            if (valid && normalized && normalized.length > 7 && normalized.length < 10) {
                this.valid_rut = true;
            } else {
                if (this.edit_usuario.rut.length === 0) {
                    this.valid_rut = true;
                } else {
                    this.valid_rut = false;
                }
            }
        },
    },
    watch: {
        nuevo_rut() {
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
                                <h2 class="fw-bold mb-1 text-uppercase">Editar usuario</h2>
                                <p class="text-50 mb-2">
                                    Ingresa los datos para editar usuario. Deja en blanco si no
                                    deseas modificar.
                                </p>

                                <div class="form-outline form-white mb-2">
                                    <v-text-field
                                        v-model="edit_usuario.nombre"
                                        :label="usuario.nombre"
                                    ></v-text-field>
                                </div>

                                <div class="form-outline form-white mb-2">
                                    <v-text-field
                                        v-model="edit_usuario.apellido_1"
                                        :label="usuario.apellido_1"
                                    ></v-text-field>
                                </div>

                                <div class="form-outline form-white mb-2">
                                    <v-text-field
                                        v-model="edit_usuario.apellido_2"
                                        :label="usuario.apellido_2"
                                    ></v-text-field>
                                </div>

                                <div class="form-outline form-white mb-2">
                                    <v-text-field
                                        v-model="edit_usuario.rut"
                                        :label="formatRut(usuario.rut)"
                                    ></v-text-field>
                                </div>
                                <div>
                                    <v-select
                                        :items="['Alumno', 'Ayudante', 'Docente']"
                                        density="comfortable"
                                        v-model="edit_usuario.rol"
                                        :label="usuario.rol"
                                    ></v-select>
                                </div>

                                <div class="form-outline form-white mb-4">
                                    <v-text-field
                                        v-model="edit_usuario.correo_uai"
                                        :label="usuario.correo_uai"
                                    ></v-text-field>
                                </div>

                                <button
                                    class="btn btn-primary btn-lg px-5 mt-2 mb"
                                    @click.prevent="handleSubmit"
                                >
                                    Subir cambios
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
