<script lang="ts">
// import ServiceTypes from '../services/types.js';
import { defineComponent } from 'vue';
import ChileanRutify from 'chilean-rutify';
import { useRegistrosStore } from '../stores/RegistrosStore';

export default defineComponent({
    data() {
        return {
            registros: useRegistrosStore(),
            format_rut: ChileanRutify.formatRut,
        };
    },
    methods: {
        verbose_motivo(motivo: string) {
            if (motivo === 'ramo') {
                return 'Asistencia a ramo';
            } else if (motivo === 'ventana') {
                return 'Uso de ventana';
            }
            return motivo.charAt(0).toUpperCase() + motivo.slice(1);
        },
    },
    async beforeMount() {
        this.registros.update();
    },
    mounted() {},
});
</script>
<template>
    <div class="container mt-5">
        <div class="card-body text-center mt-5 title-container">
            <h4 class="card-title">Registro accesos</h4>
            <p class="card-text">Listado de ultimos accesos al lab</p>
        </div>

        <div class="d-flex justify-content-between">
            <button
                class="btn btn-primary me-2"
                @click.prevent="registros.prev"
                :disabled="registros.offset === 0"
            >
                Prev
            </button>
            <button
                class="btn btn-primary ms-2"
                @click.prevent="registros.next"
                :disabled="registros.getRegistros.length < 10"
            >
                Next
            </button>
        </div>

        <div class="table-responsive card mt-3" v-if="registros.getRegistros.length !== 0">
            <v-table density="compact" id="custom-table" v-if="registros.getRegistros.length !== 0">
                <thead>
                    <tr>
                        <th class="text-center">ID</th>
                        <th class="text-center">Rut</th>
                        <th class="text-center">Nombre</th>
                        <th class="text-center">Correo</th>
                        <th class="text-center">Rol</th>
                        <th class="text-center">Motivo</th>
                        <th class="text-center">...</th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="registro in registros.registros" :key="registro.id">
                        <td class="text-center">{{ registro.id }}</td>
                        <td class="text-center">{{ format_rut(registro.rut) }}</td>
                        <td class="text-center">
                            {{ registro.nombre }} {{ registro.apellido_1 }}
                            {{ registro.apellido_2 }}
                        </td>
                        <td class="text-center">{{ registro.correo_uai }}</td>
                        <td class="text-center">
                            {{ registro.rol.charAt(0).toUpperCase() + registro.rol.slice(1) }}
                        </td>
                        <td class="text-center">{{ verbose_motivo(registro.motivo) }}</td>
                        <td class="text-center">
                            {{ registro.salida ? 'Salida' : 'Entrada' }} el
                            {{
                                new Date(registro.fecha).toLocaleString('es-CL', {
                                    timeZone: 'America/Santiago',
                                })
                            }}
                        </td>
                    </tr>
                </tbody>
            </v-table>
        </div>
    </div>
</template>

<style>
#custom-table tbody tr {
    border-bottom: none;
}
.title-container h4 {
    margin-top: 50px;
}
</style>
