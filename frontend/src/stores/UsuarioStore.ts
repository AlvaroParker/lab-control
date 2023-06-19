import { defineStore } from 'pinia';
import ServiceTypes from '../services/types';
import GetService from '../services/get.service';

export const useUsuarioStore = defineStore('UsuarioStore', {
    state: () => {
        return {
            usuarios: new Array<ServiceTypes.Usuario>(),
            request_made: false,
        };
    },
    getters: {
        getUsuarios: (state): Array<ServiceTypes.Usuario> => {
            if (!state.request_made) {
                state.request_made = true;
                GetService.getUsuarios().then((usuarios) => {
                    if (usuarios) {
                        state.usuarios = usuarios;
                    }
                });
            }
            return state.usuarios;
        },
    },
    actions: {
        update() {
            GetService.getUsuarios()
                .then((usuarios) => {
                    if (usuarios) {
                        this.usuarios = usuarios;
                    }
                })
                .catch((_) => {});
        },
        clear() {
            this.usuarios = new Array<ServiceTypes.Usuario>();
        },
    },
    // actions

    // getters
});
