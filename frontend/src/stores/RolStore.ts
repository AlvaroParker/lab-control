import { defineStore } from 'pinia';
import ServiceTypes from '../services/types';
import GetService from '../services/get.service';

export const useRolStore= defineStore('RolStore', {
    state: () => {
        return {
            rols: new Array<ServiceTypes.Rol>(),
            request_made: false,
        };
    },
    getters: {
        getRols: (state): Array<ServiceTypes.Rol> => {
            if (!state.request_made) {
                state.request_made = true;
                GetService.getRoles().then(([rols, _]) => {
                    if (rols) {
                        state.rols = rols;
                    }
                });
            }
            return state.rols;
        },
    },
    actions: {
        update() {
            GetService.getRoles()
                .then(([rols, _]) => {
                    if (rols) {
                        this.rols = rols;
                    }
                })
                .catch((_) => {});
        },
        clear() {
            this.rols= new Array<ServiceTypes.Rol>();
        },
    },
    // actions

    // getters
});
