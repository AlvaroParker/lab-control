import { defineStore } from 'pinia';
import ServiceTypes, { Status } from '../services/types';
import GetService from '../services/get.service';

export const useAdminStore = defineStore('AdminStore', {
    state: () => {
        return {
            admins: new Array<ServiceTypes.AdminGeneric>(),
            request_made: false,
        };
    },
    getters: {
        getAdmins: (state): Array<ServiceTypes.AdminGeneric> => {
            if (!state.request_made) {
                state.request_made = true;
                GetService.getAdmins().then(([admins, status]) => {
                    if (status === Status.OK && admins) {
                        state.admins = admins;
                    }
                });
            }
            return state.admins;
        },
    },
    actions: {
        update() {
            GetService.getAdmins()
                .then(([admins, status]) => {
                    if (status === Status.OK && admins){
                        this.admins = admins;
                    }
                })
                .catch((_) => {});
        },
        clear() {
            this.admins = new Array<ServiceTypes.AdminGeneric>();
        },
    },
    // actions

    // getters
});
