import { createApp } from 'vue';
import './style.css';
import { createPinia } from 'pinia';

// Font awesome
import { IconDefinition, library } from '@fortawesome/fontawesome-svg-core';
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome';
import {
    faUserSecret,
    faUserLock,
    faBook,
    faUser,
    faPlus,
    faCheck,
    faSearch,
    faUserCircle,
    faSignOutAlt,
    faQrcode,
} from '@fortawesome/free-solid-svg-icons';

import App from './App.vue';
import Navbar from './components/Navbar.vue';
import { router } from './Router';

library.add(faUserSecret as IconDefinition);
library.add(faBook as IconDefinition);
library.add(faUser as IconDefinition);
library.add(faCheck as IconDefinition);
library.add(faPlus as IconDefinition);
library.add(faSearch as IconDefinition);
library.add(faUserLock as IconDefinition);
library.add(faUserCircle as IconDefinition);
library.add(faSignOutAlt as IconDefinition);
library.add(faQrcode as IconDefinition);

// Vuetify
import 'vuetify/styles';
import { createVuetify } from 'vuetify';
import * as components from 'vuetify/components';
import * as directives from 'vuetify/directives';

const vuetify = createVuetify({
    components,
    directives,
});
const pinia = createPinia();
const app = createApp(App)
    .component('font-awesome-icon', FontAwesomeIcon)
    .component('Navbar', Navbar);

app.use(router);
app.use(pinia);
app.use(vuetify);

app.mount('#app');
