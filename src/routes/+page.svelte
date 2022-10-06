<script context="module" lang="ts">
	export const prerender = true;
	//import SideBar from "./SideBar.svelte";
</script>

<script lang="ts">
	// document.body.insertAdjacentHTML(
	// 	'afterbegin',
	// 	'<img class="background" src="/DALL·E generated image.png" alt="" />'
	// );

	import { text } from 'svelte/internal';
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="an ehr for the world" />
</svelte:head>

<!-- page content -->
<section>
	<h1 class="text-green-600">page content goes here</h1>
</section>

<style>
	section {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		flex: 1;
	}

	h1 {
		width: 100%;
	}

	.welcome {
		position: relative;
		width: 100%;
		height: 0;
		padding: 0 0 calc(100% * 495 / 2048) 0;
	}

	.welcome img {
		position: absolute;
		width: 100%;
		height: 100%;
		top: 0;
		display: block;
	}
</style>
