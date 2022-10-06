<script context="module" lang="ts">
	export const prerender = true;
	//import SideBar from "./SideBar.svelte";
</script>

<script lang="ts">
	// document.body.insertAdjacentHTML(
	// 	'afterbegin',
	// 	'<img class="background" src="/DALL·E generated image.png" alt="" />'
	// );

	import { text } from "svelte/internal";

</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="an ehr for the world" />
</svelte:head>
<section>
	<div class="fixed top-0 left-0 h-screen w-16 m-0 flex flex-col bg-gray-900
				text-white shadow-lg">
		<div class="group">
			<i class="relative flex items-center justify-center h-12 w-12 mt-2 mb-2
				mx-auto shadow-lg bg-gray-800 text-green-500 hover:bg-green-600
				hover:text-white rounded-3xl hover:rounded-xl transition-all
				duration-300 ease-liner">
				ME
			</i>
			<span class="absolute w-auto p-2 m-2 min-2-max left-14 rounded-md
						shadow-md text-white bg-gray-900 text-xs font-bold
						transition-all duration-100 scale-0 origin-left group-hover:scale-100">
				go to profile
			</span>
		</div>
		
		<div class="group">
			<i class="relative flex items-center justify-center h-12 w-12 mt-2 mb-2
				mx-auto shadow-lg bg-gray-800 text-green-500 hover:bg-green-600
				hover:text-white rounded-3xl hover:rounded-xl transition-all
				duration-300 ease-liner">
				Dash
			</i>
			<span class="absolute w-auto p-2 m-2 min-2-max left-14 rounded-md
						shadow-md text-white bg-gray-900 text-xs font-bold
						transition-all duration-100 scale-0 origin-left group-hover:scale-100">
				dashboard
			</span>
		</div>

		<div class="group">
			<i class="relative flex items-center justify-center h-12 w-12 mt-2 mb-2
				mx-auto shadow-lg bg-gray-800 text-green-500 hover:bg-green-600
				hover:text-white rounded-3xl hover:rounded-xl transition-all
				duration-300 ease-liner">
				List
			</i>
			<span class="absolute w-auto p-2 m-2 min-2-max left-14 rounded-md
						shadow-md text-white bg-gray-900 text-xs font-bold
						transition-all duration-100 scale-0 origin-left group-hover:scale-100">
				Patient List
			</span>
		</div>
		
		<div class="group">
			<i class="relative flex items-center justify-center h-12 w-12 mt-2 mb-2
				mx-auto shadow-lg bg-gray-800 text-green-500 hover:bg-green-600
				hover:text-white rounded-3xl hover:rounded-xl transition-all
				duration-300 ease-liner">
				Team
			</i>
			<span class="absolute w-auto p-2 m-2 min-2-max left-14 rounded-md
						shadow-md text-white bg-gray-900 text-xs font-bold
						transition-all duration-100 scale-0 origin-left group-hover:scale-100">
				Team
			</span>
		</div>
		
		<div class="group">
			<i class="relative flex items-center justify-center h-12 w-12 mt-2 mb-2
				mx-auto shadow-lg bg-gray-800 text-green-500 hover:bg-green-600
				hover:text-white rounded-3xl hover:rounded-xl transition-all
				duration-300 ease-liner">
				Resou
			</i>
			<span class="absolute w-auto p-2 m-2 min-2-max left-14 rounded-md
						shadow-md text-white bg-gray-900 text-xs font-bold
						transition-all duration-100 scale-0 origin-left group-hover:scale-100">
				Resources
			</span>
		</div>
	</div>
	<!-- <SideBar /> -->
</section>
<section>
	<h1 class="text-3xl, text-green-600">hello peeps! This is the dev branch!</h1>
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
