<script lang="ts">
import Navbar from './components/Navbar.vue';
import { is_authenticated } from './services/auth.service';

export default {
    data() {
        return {
            route: this.$route.name,
            is_auth: false,
        };
    },
    async beforeMount() {
        await this.is_user_auth();
    },
    methods: {
        async is_user_auth() {
            this.is_auth = await is_authenticated();
        },
        getName() {
            return this.$route.name;
        },
    },
    components: {
        Navbar,
    },
};
</script>

<template>
    <Navbar v-if="is_auth"></Navbar>
    <main v-if="is_auth">
        <RouterView></RouterView>
    </main>
    <RouterView v-else></RouterView>
</template>

<style scoped></style>
