<script lang="ts">
import { defineComponent } from 'vue';
import ChileanRutify from 'chilean-rutify';
import { useRouter } from 'vue-router';
import { AxiosError } from 'axios';

import { ServiceTypes, GetService, DeleteService, PostService, Status } from 'lab-control';

export default defineComponent({
    data() {
        return {
            usuario: {} as ServiceTypes.Usuario, // Here the usuario is stored
            display_not_found: false, // If not found component should be displayed or not
            format_rut: ChileanRutify.formatRut, // Format rut function
            delete_error: false, // If there was a delete error
            showModal: false, // If we should show modal or not
            showModalEnroll: false,
            error_detected: false,
            message: '',
            registrando_huella: false,
            current: 0,
            total: 1,
        };
    },
    setup() {
        // Router to go home
        const router = useRouter();
        const go_home = () => {
            router.push({
                name: 'Home',
            });
        };
        // Go to EDIT route
        const go_edit = (rut: string) => {
            router.push({
                name: 'EditUsuario',
                query: {
                    rut,
                },
            });
        };
        return {
            go_home,
            go_edit,
        };
    },
    methods: {
        async handleSubmit() {
            this.showModalEnroll = false;
            this.registrando_huella = true;
            const ws = await PostService.RerollUsuario(this.usuario.rut);
            ws.onerror = (error) => {
                console.log(error.message);
            };
            ws.onmessage = (data) => {
                let status = JSON.parse(data.data as string);
                this.total = status.total;
                this.current = status.current;
                let stage = `Registrando huella... ${status.current} de ${status.total}`;
                console.log(stage);
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
        },
        // Delete usuario handling
        async deleteUsuario() {
            // Delete usuario DELETE request
            try {
                const res = await DeleteService.DeleteUsuario(this.usuario.rut);
                if (res === Status.OK) {
                    this.showModal = false;
                    this.go_home();
                } else {
                    this.delete_error = true;
                }
            } catch (e) {
                const err = e as AxiosError;
                console.log(err);
            }
        },
        // Edit usuario handling
        editUsuario() {
            this.go_edit(this.usuario.rut);
        },
        async enrollHuella() {
            await this.handleSubmit();
            // Todo
        },
    },
    // Get the usuario before mount
    async beforeCreate() {
        const rut = this.$route.query.rut;
        if (!rut || typeof rut !== 'string') {
            this.display_not_found = true;
        } else {
            GetService.GetUsuarioByRut(rut)
                .then(([usuario, status]) => {
                    if (status === Status.OK && usuario) {
                        this.usuario = usuario;
                    } else {
                        this.display_not_found = true;
                    }
                })
                .catch((_err) => {
                    this.display_not_found = true;
                });
        }
    },
    mounted() {
        this.usuario;
    },
});
</script>
<template>
    <section className="container-fluid h-100">
        <div className="container py-5 h-100">
            <div className="row d-flex justify-content-center align-items-center h-100">
                <div className="col-12 col-md-8 col-lg-6 col-xl-5" v-if="usuario.nombre">
                    <div className="card shadow-2-card">
                        <div className="card-body">
                            <h5 className="card-title text-center">Usuario</h5>
                            <ul className="list-group list-group-flush">
                                <li className="list-group-item">
                                    Nombre: {{ usuario.nombre }} {{ usuario.apellido_1 }}
                                    {{ usuario.apellido_2 }}
                                </li>
                                <li className="list-group-item">
                                    Rut: {{ format_rut(usuario.rut) }}
                                </li>
                                <li className="list-group-item">
                                    Correo: {{ usuario.correo_uai }}
                                </li>
                                <li className="list-group-item" v-if="usuario.last_registro">
                                    Ultimo Registro:
                                    {{
                                        usuario.last_registro.salida ? 'Salida el ' : 'Entrada el '
                                    }}

                                    {{
                                        new Date(usuario.last_registro.fecha).toLocaleString(
                                            'es-CL',
                                            {
                                                timeZone: 'America/Santiago',
                                            }
                                        )
                                    }}
                                </li>

                                <li className="list-group-item" v-if="!usuario.last_registro">
                                    Ultimo registro: Nunca
                                </li>
                            </ul>
                        </div>
                    </div>
                    <div class="d-flex justify-content-center mt-3">
                        <button
                            class="mx-2 btn btn-danger btn-space me-4"
                            data-toggle="modal"
                            @click="showModal = true"
                        >
                            Eliminar usuario
                        </button>
                        <button class="mx-2 btn btn-primary btn-space" @click="editUsuario">
                            Editar usuario
                        </button>
                        <button
                            class="mx-2 btn btn-warning btn-space"
                            @click="showModalEnroll = true"
                        >
                            Cambiar huella
                        </button>
                    </div>
                    <Teleport to="body">
                        <Transition name="modal">
                            <div v-if="showModalEnroll" class="modal-mask">
                                <div class="modal-container border rounded-3">
                                    <div class="modal-header justify-content-center mb-3">
                                        Registrar huella nuevamente?
                                    </div>

                                    <div class="modal-footer justify-content-center">
                                        <button
                                            class="btn btn-danger modal-default-button me-5"
                                            @click="enrollHuella"
                                        >
                                            Registrar
                                        </button>
                                        <button
                                            class="btn btn-primary modal-default-button"
                                            @click="showModalEnroll = false"
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
                                        Eliminar usuario?
                                    </div>

                                    <div class="modal-footer justify-content-center">
                                        <button
                                            class="btn btn-danger modal-default-button me-5"
                                            @click="deleteUsuario"
                                        >
                                            Eliminar
                                        </button>
                                        <button
                                            class="btn btn-primary modal-default-button"
                                            @click="showModal = false"
                                        >
                                            Cancelar
                                        </button>
                                    </div>
                                </div>
                            </div>
                        </Transition>
                    </Teleport>
                </div>

                <Transition name="fade">
                    <div className="col-12 col-md-8 col-lg-6 col-xl-5" v-if="display_not_found">
                        <div className="card shadow-2-card">
                            <div className="card-body">
                                <h5 className="card-title text-center">Usuario no encontrado</h5>
                            </div>
                        </div>
                    </div>
                </Transition>
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
            </div>
        </div>
    </section>
</template>

<style>
tr:last-child td {
    border-bottom: 0px;
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
