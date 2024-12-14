<script lang="ts">
    //import Tab from '/Tab.svelte';
	import {TabStore, ActiveTabStore, addTab, setActiveTab, removeTab } from '../../stores/TabStore';

	// a store of the number of active tabs
	// $storeTabs = [
	// 	{ id:0, name: 'tab 1'},
	// 	{ id:1, name: "tab 2"},
	// 	{ id:2, name: "tab 3"}
	// ];

	let tabs: any[] = [];
    
	TabStore.subscribe(data => {
		tabs = data;
	});

	const deleteTab = (id: number) => {
		// tabs = tabs.filter((tab) => tab.id != id);
		// console.log("tab deleted");
	}

	// const addTab = () => {
	// 	// let idd = newtabs++;
	// 	// let next = {name: "newtab!", id: idd};
	// 	// TabStore.update(currentTabs => {
	// 	// 	return [next, ...currentTabs];
	// 	// });
		
	// 	// console.log(tabs.length + 1);
	// 	// let next = {name: 'newtab', id: tabs.length + 1};
	// 	// tabs.push(next);
	// 	// console.log(tabs);
	// 	// tabs = tabs;
	// }
</script>

<!-- the list of tabs -->
<div class="flex flex-row gap-0 bg-white mt-3">
    {#each tabs as tab (tab.id)}
    <div>
        <!-- a single tab -->
        <div class="bg-white px-2 pt-1 inline-flex border-r border-black active:rounded-t-lg active:bg-gray-300 active:border-none">
            <h4>{tab.name}</h4>
            <h4>{tab.id}</h4>

            <!-- delete tab button -->
            <button on:click={() => {deleteTab(tab.id)}}>x</button>
        </div>
    </div>
    {:else}
    <p>no tabs left!</p>
    {/each}
</div>

<style>
</style>