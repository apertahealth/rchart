import { writable } from 'svelte/store';

const TabStore = writable([
    {
        name: 'tab1',
        id: 1
    },
    {
        name: 'tab2',
        id: 2
    }
]);

export default TabStore;