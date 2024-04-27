import { defineStore } from 'pinia';
import { ServiceTypes, Status } from 'lab-control';
import { GetService } from 'lab-control';

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
                GetService.GetAdmins().then(([admins, status]) => {
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
            GetService.GetAdmins()
                .then(([admins, status]) => {
                    if (status === Status.OK && admins) {
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
