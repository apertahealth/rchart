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
	<div class="fixed top-0 left-0 h-screen w-20 m-0 flex flex-col bg-gray-50
				text-white shadow-lg">

		<!-- the sidebar content-->
		<div class="mt-3">
			<div class="group">
				<div class="relative flex items-center justify-center">
					<img class="w-16 h-16 rounded-full border-2 border-blue-400" src="./src/lib/img/doctor_headshot.jpg" alt="profile-pic">
					<div>
						<img class="w-8 h-8 rounded-full justify-center absolute bottom-0 right-0" src="./src/lib/img/wsu.png" alt="institution">
						<span class="w-4 h-4 rounded-full bg-green-500 absolute bottom-0 left-2"></span>
					</div>
				</div>
				<span class="absolute w-auto p-2 m-2 min-2-max left-14 rounded-md
						shadow-md text-white bg-gray-900 text-xs font-bold
						transition-all duration-100 scale-0 origin-left group-hover:scale-100">
				go to profile
				</span>
			</div>
		
			<div class="group">
				<i class="relative flex items-center justify-center h-16 w-16 mt-2 mb-2
					mx-auto shadow-lg bg-gray-800 text-green-500 hover:bg-green-600
					hover:text-white rounded-3xl hover:rounded-xl transition-all
					duration-300 ease-liner">
					<i class="fa-solid fa-calendar-days h-8 w-8"></i>
				</i>
				<span class="absolute w-auto p-2 m-2 min-2-max left-14 rounded-md
							shadow-md text-white bg-gray-900 text-xs font-bold
							transition-all duration-100 scale-0 origin-left group-hover:scale-100">
					dashboard
				</span>
			</div>

			<div class="group">
				<i class="relative flex items-center justify-center h-16 w-16 mt-2 mb-2
					mx-auto shadow-lg bg-gray-800 text-green-500 hover:bg-green-600
					hover:text-white rounded-3xl hover:rounded-xl transition-all
					duration-300 ease-liner">
					<i class="fa-solid fa-list h-8 w-8"></i>
				</i>
				<span class="absolute w-auto p-2 m-2 min-2-max left-14 rounded-md
							shadow-md text-white bg-gray-900 text-xs font-bold
							transition-all duration-100 scale-0 origin-left group-hover:scale-100">
					Patient List
				</span>
			</div>
			
			<div class="group">
				<i class="relative flex items-center justify-center h-16 w-16 mt-2 mb-2
					mx-auto shadow-lg bg-gray-800 text-green-500 hover:bg-green-600
					hover:text-white rounded-3xl hover:rounded-xl transition-all
					duration-300 ease-liner">
					<i class="fa-solid fa-people-group h-8 w-8"></i>
				</i>
				<span class="absolute w-auto p-2 m-2 min-2-max left-14 rounded-md
							shadow-md text-white bg-gray-900 text-xs font-bold
							transition-all duration-100 scale-0 origin-left group-hover:scale-100">
					Team
				</span>
			</div>
			
			<div class="group">
				<i class="relative flex items-center justify-center h-16 w-16 mt-2 mb-2
					mx-auto shadow-lg bg-gray-800 text-green-500 hover:bg-green-600
					hover:text-white rounded-3xl hover:rounded-xl transition-all
					duration-300 ease-liner">
					<i class="fa-solid fa-book-medical h-8 w-8"></i>
				</i>
				<span class="absolute w-auto p-2 m-2 min-2-max left-14 rounded-md
							shadow-md text-white bg-gray-900 text-xs font-bold
							transition-all duration-100 scale-0 origin-left group-hover:scale-100">
					Resources
				</span>
			</div>
			
			<div class="group">
				<i class="relative flex items-center justify-center h-16 w-16 mt-2 mb-2
					mx-auto shadow-lg bg-gray-800 text-green-500 hover:bg-green-600
					hover:text-white rounded-3xl hover:rounded-xl transition-all
					duration-300 ease-liner">
					<i class="fa-solid fa-puzzle-piece h-8 w-8 pl-1 pb-1"></i>
				</i>
				<span class="absolute w-auto p-2 m-2 min-2-max left-14 rounded-md
							shadow-md text-white bg-gray-900 text-xs font-bold
							transition-all duration-100 scale-0 origin-left group-hover:scale-100">
					Extensions
				</span>
			</div>
		</div>

	</div>
</section>

<!-- search bar -->
<section>
	<div class="absolute top-0 left-20 h-20 w-screen flex justify-between py-3 px-6 bg-gray-50 border-b">
		<form class="w-full max-w-md">
			<div class="relative flex items-center text-gray-400 focus-within:text-gray-600">
				<i class="fa-solid fa-magnifying-glass w-5 h-5 absolute ml-3 pointer-events-none"></i>
				<input type="text" name="search" placeholder="Search" autocomplete="off"
				class="w-full pr-3 pl-10 py-2 font-semibold placeholder-gray-500 text-bloack
				rounded-2xl border-none ring-2 ring-gray-300 focus:ring-gray-500
				focus:ring-2">
			</div>
		</form>
	</div>
</section>
<section>
	<button><i class="fa-solid fa-bell"></i></button>
	<button><i class="fa-solid fa-chart-simple"></i></button>
	<button><i class="fa-solid fa-gear"></i></button>
</section>

<!-- page content -->
<section>
	<h1 class="text-3xl, text-green-600">hello peeps! This is the dev branch!</h1>
</section>

<!-- status bar -->
<section>

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
