import { defineStore } from 'pinia';
import ServiceTypes from '../services/types';
import GetService from '../services/get.service';

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
            const registros = await GetService.getRegistros(this.offset, this.limit);
            this.registros = registros ?? this.registros;
            return this.registros;
        },
        async next() {
            this.offset += 10;
            const registro = await GetService.getRegistros(this.offset, this.limit);
            // Ignore this error
            if (registro?.length !== 0) {
                this.registros = registro as Array<ServiceTypes.Registro>;
            } else {
                this.offset -= 10;
            }
        },
        async prev() {
            if (this.offset >= 10) {
                this.offset -= 10;
            }
            const registros = await GetService.getRegistros(this.offset, this.limit);
            this.registros = registros ?? this.registros;
        },
        clear() {
            this.registros = [];
        },
    },
    // actions

    // getters
});
