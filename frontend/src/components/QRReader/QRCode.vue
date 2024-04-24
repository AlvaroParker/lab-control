<script lang="ts">
import { QrcodeStream } from 'vue3-qrcode-reader';
import QRTypes from './QRTypes';
import { defineComponent } from 'vue';
import ChileanRutify from 'chilean-rutify';
import { useMotivoStore } from '../../stores/MotivoStore';

import {PostService} from 'lab-control'
import { Status } from 'lab-control';

export default defineComponent({
    data() {
        return {
            camera: 'auto', // Camera status, either 'auto' or 'off'
            rut: '', // Get the rut from the qr code
            motivo: '', // Motive of access/exit
            motivos: useMotivoStore(),
            showModal: false, // Either to show modal with motive choice or not
            formatRut: ChileanRutify.formatRut, // Function to format rut
        };
    },
    methods: {
        async onInit(promise: Promise<any>) {
            // show loading indicator
            try {
                // Await promise
                await promise;
            } catch (e) {
                // There was an error loading the camera
                console.log(e);
            }
        },
        // Submit the new registro, with salida and motivo
        async submitRegistro(salida: boolean, motivo: string) {
            // Send the POST request
            const status = await PostService.NewRegistro(this.rut, salida, motivo);
            if (status !== Status.OK) {
                // TODO: Handle errors
            }
            this.showModal = false;
            // Reset camera
            this.turnOffCamera();
            this.turnOnCamera();
        },
        // When QR code is detected
        async onDetect(promise: any) {
            try {
                // Await for the detected object
                const detect: QRTypes.Detect = await promise;
                // Create a URL object with the scanned text
                const url = new URL(detect.content);

                // Get the search params from the URL
                const queryParams = new URLSearchParams(url.search);
                // Extract the rut
                let rut = queryParams.get('RUN');
                // If rut is not undefined
                if (rut) {
                    const normalized_rut = ChileanRutify.normalizeRut(rut);
                    if (normalized_rut) {
                        this.rut = normalized_rut;
                        this.showModal = true;
                    }
                }
            } catch (error) {
                console.log(error);
            }
        },
        turnOffCamera() {
            this.camera = 'off';
        },
        turnOnCamera() {
            this.camera = 'on';
        },
    },
    // When element is unmounted, turn off camera
    unmounted() {
        this.camera = 'off';
    },
    async beforeMount() {
        this.motivos.update();
    },
    mounted() {
        this.motivos;
    },
    components: {
        QrcodeStream,
    },
});
</script>

<template>
    <div class="container d-flex flex-column align-items-center" style="margin-top: 100px">
        <h5>Leyendo QR Rut</h5>
        <qrcode-stream
            @init="onInit"
            @detect="onDetect"
            :camera="camera"
            style="width: 50%"
            class="img-fluid custom-image rounded"
        ></qrcode-stream>
        <p v-if="rut !== ''">Rut detectado: {{ formatRut(rut) }}</p>
    </div>

    <Teleport to="body">
        <Transition name="modal">
            <div v-if="showModal" class="modal-mask justify-content-center">
                <div class="modal-container border rounded-3">
                    <div class="modal-header justify-content-center mb-3">
                        Seleccione una opcion
                    </div>

                    <div class="container modal-body">
                        <div class="row justify-content-center align-items-center" v-for="motivo in motivos.getMotivos">
                            <div class="col-12">
                                <button class="btn btn-danger mb-5" @click="() => {submitRegistro(false, motivo.motivo);}">
                                    {{ motivo.motivo }}
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </Transition>
    </Teleport>
</template>
