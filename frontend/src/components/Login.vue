<script lang="ts">
import { defineComponent } from 'vue';
import AuthService from '../services/auth.service';
import { useRouter } from 'vue-router';

export default defineComponent({
    data() {
        return {
            username: '',
            password: '',
            auth_err: false,
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
        handleSubmit() {
            AuthService.login(this.username, this.password)
                .then((res) => {
                    if (res) {
                        this.$router.push('/').then(() => {
                            location.reload();
                        });
                    } else {
                        this.auth_err = true;
                    }
                })
                .catch((_err) => {
                    this.auth_err = true;
                });
        },
    },
});
</script>

<template>
    <section class="vh-100">
        <div class="container py-5 h-100">
            <div class="row d-flex justify-content-center align-items-center h-100">
                <div class="col-12 col-md-8 col-lg-6 col-xl-5">
                    <div class="card bg-dark text-white" style="border-radius: 1rem">
                        <div class="card-body p-5 text-center">
                            <form class="mb-md-5 mt-md-4 pb-5" @submit.prevent="handleSubmit">
                                <h2 class="fw-bold mb-2 text-uppercase">Iniciar sesion</h2>
                                <p class="text-50 mb-5">
                                    Porfavor inicia sesion para ingresar a la plataforma
                                </p>

                                <div class="form-outline form-white mb-4">
                                    <input
                                        required
                                        type="email"
                                        id="typeEmailX"
                                        class="form-control form-control-lg"
                                        v-model="username"
                                    />
                                    <label class="form-label" for="typeEmailX">Email</label>
                                </div>

                                <div class="form-outline form-white mb-4">
                                    <input
                                        required
                                        type="password"
                                        id="typePasswordX"
                                        class="form-control form-control-lg"
                                        v-model="password"
                                    />
                                    <label class="form-label" for="typePasswordX">Contraseña</label>
                                </div>
                                <p style="color: red" v-if="auth_err">
                                    Usuario o contraseña incorrectos
                                </p>

                                <button class="btn btn-primary btn-lg px-5 mt-5" type="submit">
                                    Login
                                </button>
                            </form>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </section>
</template>

<style></style>
