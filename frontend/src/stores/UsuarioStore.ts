import { defineStore } from 'pinia';
import ServiceTypes, { Status } from '../services/types';
import {GetService} from 'lab-control';

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
                GetService.GetUsuarios().then(([usuarios, status]) => {
                    if (status == Status.OK) {
                        state.usuarios = usuarios;
                    } else {
                        state.usuarios = new Array<ServiceTypes.Usuario>();
                    }
                });
            }
            return state.usuarios;
        },
    },
    actions: {
        update() {
            GetService.GetUsuarios()
                .then(([usuarios, status]) => {
                    if (status == Status.OK) {
                        this.usuarios = usuarios;
                    } else {
                        this.usuarios = new Array<ServiceTypes.Usuario>();
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
