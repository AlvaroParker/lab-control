<script lang="ts">
// import ServiceTypes from '../services/types.js';
import { defineComponent } from 'vue';
import GetService from '../services/get.service';
import ServiceTypes from '../services/types';
import ChileanRutify from 'chilean-rutify';
import { useRouter } from 'vue-router';
import DeleteService from '../services/delete.service';
import { AxiosError } from 'axios';

export default defineComponent({
    data() {
        return {
            usuario: {} as ServiceTypes.Usuario, // Here the usuario is stored
            display_not_found: false, // If not found component should be displayed or not
            format_rut: ChileanRutify.formatRut, // Format rut function
            delete_error: false, // If there was a delete error
            showModal: false, // If we should show modal or not
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
        // Delete usuario handling
        async deleteUsuario() {
            // Delete usuario DELETE request
            try {
                const res = await DeleteService.deleteUsuario(this.usuario.rut);
                if (res.status === 200) {
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
    },
    // Get the usuario before mount
    async beforeCreate() {
        const rut = this.$route.query.rut;
        if (!rut || typeof rut !== 'string') {
            this.display_not_found = true;
        } else {
            GetService.getUsuarioByRut(rut)
                .then((usuario) => {
                    if (usuario) {
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
                            </ul>
                        </div>
                    </div>
                    <div class="d-flex justify-content-center mt-3">
                        <button
                            class="ml-4 btn btn-danger btn-space me-4"
                            data-toggle="modal"
                            @click="showModal = true"
                        >
                            Eliminar usuario
                        </button>
                        <button class="btn btn-primary btn-space" @click="editUsuario">
                            Editar usuario
                        </button>
                    </div>

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
