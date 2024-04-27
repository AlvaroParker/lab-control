<script lang="ts">
import { defineComponent } from 'vue';
import { useRouter } from 'vue-router';

import { PostService, ServiceTypes, Status } from 'lab-control';

export default defineComponent({
    data() {
        return {
            nombre: '',
            apellido_1: '',
            apellido_2: '',
            password: '',
            email: '',
            newAdminErr: null,
        };
    },
    setup() {
        const router = useRouter();

        const go_home = () => {
            router.push({
                name: 'Admin',
            });
        };
        return {
            go_home,
        };
    },
    methods: {
        handleSubmit() {
            const admin: ServiceTypes.AdminRegistro = {
                nombre: this.nombre,
                apellido_1: this.apellido_1,
                apellido_2: this.apellido_2,
                pswd: this.password,
                email: this.email,
            };
            PostService.EnrollAdmin(admin).then((status) => {
                if (status === Status.OK) {
                    this.go_home();
                } else {
                    // Error happenned, todo
                    this.go_home();
                }
            });
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
                                <h2 class="fw-bold mb-1 text-uppercase">Nuevo Admin</h2>
                                <p class="text-50 mb-2">
                                    Ingresa los datos para registrar un nuevo administrador
                                </p>

                                <div class="form-outline form-white mb-2">
                                    <v-text-field
                                        v-model="nombre"
                                        label="Nombre"
                                        required
                                    ></v-text-field>
                                </div>

                                <div class="form-outline form-white mb-2">
                                    <v-text-field
                                        v-model="apellido_1"
                                        label="Primer apellido"
                                        required
                                    ></v-text-field>
                                </div>

                                <div class="form-outline form-white mb-2">
                                    <v-text-field
                                        v-model="apellido_2"
                                        label="Segundo apellido"
                                        required
                                    ></v-text-field>
                                </div>

                                <div class="form-outline form-white mb-2">
                                    <v-text-field
                                        v-model="email"
                                        label="Email"
                                        required
                                    ></v-text-field>
                                </div>

                                <div class="form-outline form-white mb-4">
                                    <v-text-field
                                        v-model="password"
                                        type="password"
                                        label="Password"
                                        required
                                    ></v-text-field>
                                </div>

                                <button class="btn btn-primary btn-lg px-5 mt-2 mb">
                                    Registrar admin
                                </button>
                            </form>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </section>
</template>
