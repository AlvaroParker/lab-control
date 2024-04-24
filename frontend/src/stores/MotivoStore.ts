import { defineStore } from 'pinia';
import ServiceTypes from '../services/types';
import {GetService } from 'lab-control'

export const useMotivoStore= defineStore('MotivoStore', {
    state: () => {
        return {
            motivos: new Array<ServiceTypes.Motivo>(),
            request_made: false,
        };
    },
    getters: {
        getMotivos: (state): Array<ServiceTypes.Motivo> => {
            if (!state.request_made) {
                state.request_made = true;
                GetService.GetMotivos().then(([motivos, status]) => {
                    console.log(status);
                    console.log(motivos);
                    if (motivos) {
                        state.motivos= motivos;
                    }
                });
            }
            return state.motivos;
        },
    },
    actions: {
        update() {
            GetService.GetMotivos()
                .then(([motivos, _]) => {
                    if (motivos) {
                        this.motivos = motivos;
                    }
                })
                .catch((_) => {});
        },
        clear() {
            this.motivos = new Array<ServiceTypes.Motivo>();
        },
    },
    // actions

    // getters
});
