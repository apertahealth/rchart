<script>
    import Tab from './tab.svelte';
	import TabStore from '../../stores/TabStore.js';

	// a store of the number of active tabs
	// $storeTabs = [
	// 	{ id:0, name: 'tab 1'},
	// 	{ id:1, name: "tab 2"},
	// 	{ id:2, name: "tab 3"}
	// ];

	export let tabs = [];
    
	TabStore.subscribe(data => {
		tabs = data;
	});

	const deleteTab = (id) => {
		tabs = tabs.filter((tab) => tab.id != id);
		console.log("tab deleted");
	}

	const addTab = () => {
		// let idd = newtabs++;
		// let next = {name: "newtab!", id: idd};
		// TabStore.update(currentTabs => {
		// 	return [next, ...currentTabs];
		// });
		
		console.log(tabs.length + 1);
		let next = {name: 'newtab', id: tabs.length + 1};
		tabs.push(next);
		console.log(tabs);
		tabs = tabs;
	}
</script>

<!-- the list of tabs -->
<div class="flex flex-row gap-5 bg-red-500 mx-5">
    {#each tabs as tab (tab.id)}
    <div>
        <!-- a single tab -->
        <div class="bg-green-300 px-2 inline-flex">
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