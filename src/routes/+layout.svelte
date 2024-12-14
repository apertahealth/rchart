<script lang="ts">
	import '../app.css';

	// components
	import TabList from '../lib/components/TabList.svelte';
	import MessageCenter from '../lib/components/MessageCenter.svelte';
	import SidebarTab from '../lib/components/SidebarTab.svelte';
	import ProfileButton from '$lib/components/ProfileButton.svelte';
	import StatusBar from '$lib/components/StatusBar.svelte';

	// stores
	import { SideBarStore, setTab } from '../stores/SideBarStore';
	$: $SideBarStore; // keeps track of the active side bar for styling

	import { TabStore, ActiveTabStore, addTab, setActiveTab, removeTab } from '../stores/TabStore';
	$: $ActiveTabStore;
	$: $TabStore;

	let newTabTitle: string = 'New Tab';
	// Add a new tab when called
	function handleAddTab() {
		const newTab = {
			id: `tab-${Date.now()}`, // Generate a unique ID using timestamp
			title: newTabTitle
		};
		addTab(newTab); // Call addTab to add the tab
	}

	const sideBarTabInfo = [
		{ label: 'Dashboard', icon: 'fa-calendar-days', path: '/' },
		{ label: 'Patient List', icon: 'fa-list', path: '/list' },
		{ label: 'Team', icon: 'fa-people-group', path: '/team' },
		{ label: 'Resources', icon: 'fa-book-medical', path: '/resources' },
		{ label: 'Extensions', icon: 'fa-puzzle-piece', path: '/extensions' }
	];
</script>

<body class="overscroll-none bg-gray-300">
	<!-- top bar -->
	<section class="flex flex-col absolute top-0 left-20 h-30 w-full pt-3 px-5 bg-white">
		<!-- the top part of the bar -->
		<div class="flex flex-row">
			<!-- search box -->
			<form class="w-full max-w-md">
				<div class="relative flex items-center text-gray-400 focus-within:text-gray-500">
					<i
						class="fa-solid fa-magnifying-glass w-5 h-5 absolute ml-3 pointer-events-none"
					/>
					<input
						type="text"
						name="search"
						placeholder="Search"
						autocomplete="off"
						class="w-full pr-3 pl-10 py-2 font-semibold bg-gray-100 placeholder-gray-500 text-block
								rounded-2xl border-none ring-2 ring-gray-300 focus:ring-gray-500
								focus:ring-2"
					/>
				</div>
			</form>

			<!-- three top buttons -->
			<div
				class="fixed right-0 mr-10 bg-gray-300 px-3 pt-3 pb-2 rounded-xl flex flex-row gap-3"
			>
				<!-- notifications -->
				<button
					on:click={() => {
						handleAddTab();
						setTab(-1); // show all inactive sidebar tabs
					}}><i class="fa-solid fa-bell h-6 w-6 text-gray-500" /></button
				>

				<!-- statistics -->
				<button
					on:click={() => {
						setTab(-1); // show all inactive sidebar tabs
					}}
					><a href="/stats"
						><i class="fa-solid fa-chart-simple h-6 w-6 text-gray-500" /></a
					></button
				>

				<!-- settings -->
				<button
					on:click={() => {
						setTab(-1); // show all inactive sidebar tabs
					}}
					><a href="/options"><i class="fa-solid fa-gear h-6 w-6 text-gray-500" /></a
					></button
				>
			</div>
		</div>

		<!-- the bottom part of the bar is just tabs -->
		<TabList />
	</section>

	<!-- side bar -->
	<section class="fixed top-0 left-0 h-screen w-20 m-0 p-0 flex flex-col gap-0  bg-white">
		<!-- profile button-->
		<ProfileButton />

		<!-- sidebar tabs -->
		{#each sideBarTabInfo as { label, icon, path }, index}
			<button
				on:click={() => {
					setTab(index);
				}}
			>
				<SidebarTab {label} {icon} {path} isActive={index == $SideBarStore} />
			</button>
		{/each}
	</section>

	<!-- page contents -->
	<main>
		<slot />
	</main>

	<!-- message center -->
	<MessageCenter />

	<!-- status bar -->
	<StatusBar />
</body>

<style>
	body {
		margin: 0;
		height: 100%;
		width: 100%;
		overflow: hidden;
	}

	main {
		flex: 1;
		display: flex;
		flex-direction: column;
		padding: 1rem;
		width: 100%;
		max-width: 1024px;
		margin: 0 auto;
		box-sizing: border-box;
	}
</style>
