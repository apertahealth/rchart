<script lang="ts">
	import '../app.css';
</script>

<!-- top bar -->
<section class="flex flex-row absolute top-0 left-20 h-20 w-screen py-5 px-5 bg-gray-50">

	<!-- search box -->
	<form class="w-full max-w-md">
		<div class="relative flex items-center text-gray-400 focus-within:text-gray-600">
			<i class="fa-solid fa-magnifying-glass w-5 h-5 absolute ml-3 pointer-events-none" />
			<input
				type="text"
				name="search"
				placeholder="Search"
				autocomplete="off"
				class="w-full pr-3 pl-10 py-2 font-semibold placeholder-gray-500 text-bloack
						rounded-2xl border-none ring-2 ring-gray-300 focus:ring-gray-500
						focus:ring-2"/>
		</div>
	</form>

	<!-- three top buttons -->
	<div class="fixed right-0 mr-10 pt-2">
		<button><i class="fa-solid fa-bell h-6 w-6 text-gray-600" /></button>
		<button><i class="fa-solid fa-chart-simple h-6 w-6 text-gray-600" /></button>
		<button><i class="fa-solid fa-gear h-6 w-6 text-gray-600" /></button>
	</div>

</section>

<!-- side bar -->
<section>
	<div
		class="fixed top-0 left-0 h-screen w-20 m-0 flex flex-col bg-gray-50
				text-white shadow-lg"
	>
		<!-- the sidebar content-->
		<div class="mt-3">
			<div class="group">
				<div class="relative flex items-center justify-center">
					<img
						class="w-16 h-16 rounded-full border-2 border-blue-400"
						src="./src/lib/img/doctor_headshot.jpg"
						alt="profile-pic"
					/>
					<div>
						<img
							class="w-8 h-8 rounded-full justify-center absolute bottom-0 right-0"
							src="./src/lib/img/wsu.png"
							alt="institution"
						/>
						<span class="w-4 h-4 rounded-full bg-green-500 absolute bottom-0 left-2" />
					</div>
				</div>
				<span
					class="absolute w-auto p-2 m-2 min-2-max left-20 rounded-md
						shadow-md text-white bg-gray-600 text-xs font-bold
						transition-all duration-100 scale-0 origin-left group-hover:scale-100"
				>
					profile
				</span>
			</div>

			<div class="group">
				<i
					class="relative flex items-center justify-center h-16 w-16 mt-2 mb-2
					mx-auto shadow-lg bg-gray-200 text-gray-600 hover:bg-gray-700
					hover:text-white rounded-3xl hover:rounded-xl transition-all
					duration-300 ease-liner"
				>
					<i class="fa-solid fa-calendar-days h-8 w-8" />
				</i>
				<span
					class="absolute w-auto p-2 m-2 min-2-max left-20 rounded-md
							shadow-md text-white bg-gray-900 text-xs font-bold
							transition-all duration-100 scale-0 origin-left group-hover:scale-100"
				>
					dashboard
				</span>
			</div>

			<div class="group">
				<i
					class="relative flex items-center justify-center h-16 w-16 mt-2 mb-2
					mx-auto shadow-lg bg-gray-200 text-gray-600 hover:bg-gray-700
					hover:text-white rounded-3xl hover:rounded-xl transition-all
					duration-300 ease-liner"
				>
					<i class="fa-solid fa-list h-8 w-8" />
				</i>
				<span
					class="absolute w-auto p-2 m-2 min-2-max left-20 rounded-md
							shadow-md text-white bg-gray-900 text-xs font-bold
							transition-all duration-100 scale-0 origin-left group-hover:scale-100"
				>
					Patient List
				</span>
			</div>

			<div class="group">
				<i
					class="relative flex items-center justify-center h-16 w-16 mt-2 mb-2
					mx-auto shadow-lg bg-gray-200 text-gray-600 hover:bg-gray-700
					hover:text-white rounded-3xl hover:rounded-xl transition-all
					duration-300 ease-liner"
				>
					<i class="fa-solid fa-people-group h-8 w-8" />
				</i>
				<span
					class="absolute w-auto p-2 m-2 min-2-max left-20 rounded-md
							shadow-md text-white bg-gray-900 text-xs font-bold
							transition-all duration-100 scale-0 origin-left group-hover:scale-100"
				>
					Team
				</span>
			</div>

			<div class="group">
				<i
					class="relative flex items-center justify-center h-16 w-16 mt-2 mb-2
					mx-auto shadow-lg bg-gray-200 text-gray-600 hover:bg-gray-700
					hover:text-white rounded-3xl hover:rounded-xl transition-all
					duration-300 ease-liner"
				>
					<i class="fa-solid fa-book-medical h-8 w-8" />
				</i>
				<span
					class="absolute w-auto p-2 m-2 min-2-max left-20 rounded-md
							shadow-md text-white bg-gray-900 text-xs font-bold
							transition-all duration-100 scale-0 origin-left group-hover:scale-100"
				>
					Resources
				</span>
			</div>

			<div class="group">
				<i
					class="relative flex items-center justify-center h-16 w-16 mt-2 mb-2
					mx-auto shadow-lg bg-gray-200 text-gray-600 hover:bg-gray-700
					hover:text-white rounded-3xl hover:rounded-xl transition-all
					duration-300 ease-liner"
				>
					<i class="fa-solid fa-puzzle-piece h-8 w-8 pl-1 pb-1" />
				</i>
				<span
					class="absolute w-auto p-2 m-2 min-2-max left-20 rounded-md
							shadow-md text-white bg-gray-900 text-xs font-bold
							transition-all duration-100 scale-0 origin-left group-hover:scale-100"
				>
					Extensions
				</span>
			</div>
		</div>
	</div>
</section>

<!-- page contents -->
<main>
	<slot />
</main>

<!-- status bar -->
<footer>
	<section>
		<div class="fixed bottom-0 left-0 bg-blue-600" />
	</section>
</footer>

<style>
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

	footer {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		padding: 40px;
	}

	footer a {
		font-weight: bold;
	}

	@media (min-width: 480px) {
		footer {
			padding: 40px 0;
		}
	}
</style>
