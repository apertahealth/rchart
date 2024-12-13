<script lang="ts">
	let isOpen = false;

	const toggleDrawer = () => {
		isOpen = !isOpen;
	};
</script>

<!-- Background -->
{#if isOpen}
	<div class="fixed inset-0 bg-opacity-60 z-40" on:click={toggleDrawer} />
{/if}

<!-- Drawer -->
<div
	class="fixed right-0 top-20 h-full my-5 pb-5 bg-gray-100 shadow-sm rounded-lg transition-transform duration-300 flex flex-row"
	class:translate-x-[80%]={!isOpen}
>
	<!-- Pull Tab -->
	<div class="flex flex-col justify-center items-center p-0 m-0 w-0">
		<div
			class="relative -rotate-90 px-4 py-2 mr-8 bg-gray-100 rounded-lg cursor-pointer whitespace-nowrap"
			on:click={toggleDrawer}
		>
			<span class="text-m font-bold ">Message Center</span>
		</div>
	</div>

	<!-- Drawer Contents -->
	<div class="flex flex-row py-4 px-4 mx-2 h-full ">

		<!-- Messages -->
		<div class="flex flex-col">
			<h2 class="text-lg font-semibold">Message Center</h2>

			<p class="text-sm text-gray-600">New Message</p>
			<p class="text-sm text-gray-600">New Message</p>
			<p class="text-sm text-gray-600">New Message</p>
			<p class="text-sm text-gray-600">New Message</p>
			<p class="text-sm text-gray-600">New Message</p>
			<p class="text-sm text-gray-600">New Message</p>
			<p class="text-sm text-gray-600">New Message</p>
			<p class="text-sm text-gray-600">New Message</p>
			<p class="text-sm text-gray-600">New Message</p>
			<p class="text-sm text-gray-600">New Message</p>
		</div>
	</div>
</div>
