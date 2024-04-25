import { defineStore } from 'pinia';
import {GetService, Status, ServiceTypes} from 'lab-control'

export const useRegistrosStore = defineStore('RegistrosStore', {
    state: () => {
        return {
            registros: new Array<ServiceTypes.Registro>(),
            request_made: false,
            offset: 0,
            limit: 10,
        };
    },
    getters: {
        getRegistros: (state): Array<ServiceTypes.Registro> => {
            return state.registros;
        },
    },
    actions: {
        async update() {
            const [registros, status] = await GetService.GetRegistros(this.offset, this.limit);
            if (status === Status.OK) {
                this.registros = registros!;
            }
            return this.registros;
        },
        async next() {
            this.offset += 10;
            const [registro, status] = await GetService.GetRegistros(this.offset, this.limit);
            // Ignore this error
            if (status === Status.OK) {
                if (registro?.length !== 0) {
                    this.registros = registro as Array<ServiceTypes.Registro>;
                } else {
                    this.offset -= 10;
                }
            }
        },
        async prev() {
            if (this.offset >= 10) {
                this.offset -= 10;
            }
            const [registros, status] = await GetService.GetRegistros(this.offset, this.limit);
            if (status === Status.OK) {
                this.registros = registros ?? this.registros;
            }
        },
        clear() {
            this.registros = [];
        },
    },
    // actions

    // getters
});
