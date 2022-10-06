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
</section>
<section>
	<div class="absolute top-0 left-16 flex justify-between py-3 px-6 bg-gray-50 border-b">
		<form>
			<input type="text" name="search" placeholder="Search" autocomplete="off"
			class="px-3 py-2 font-semibold placeholder-gray-500 text-bloack
			rounded-2xl border-none ring-2 ring-gray-300 focus:ring-gray-500
			focus:ring-2">
		</form>
	</div>
</section>
<section>
	<button>bell</button><button>graph</button><button>gear</button>
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
