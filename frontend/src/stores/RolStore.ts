import { defineStore } from 'pinia';
import { GetService, ServiceTypes } from 'lab-control';

export const useRolStore = defineStore('RolStore', {
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
                GetService.GetRoles().then(([rols]) => {
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
            GetService.GetRoles()
                .then(([rols]) => {
                    if (rols) {
                        this.rols = rols;
                    }
                })
                .catch(() => {});
        },
        clear() {
            this.rols = new Array<ServiceTypes.Rol>();
        },
    },
    // actions

    // getters
});
