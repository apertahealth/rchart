<script context="module" lang="ts">
	export const prerender = false;
</script>

<!-- <script lang="ts"> -->
<!-- </script> -->

<svelte:head>
	<title>Home</title>
	<meta name="description" content="an ehr for the world" />
</svelte:head>

<!-- page content -->
<div class="absolute top-20 left-20 right-10 bottom-5 m-5 flex flex-row justify-between gap-5">
	
	<!-- appointments -->
	<div class="bg-gray-50 rounded-lg shadow-xl grow">
		<div class="ml-5 mt-3">
			<h1 class="text-left">Today's Appointments</h1>
			<p>Lorem ipsum dolor sit, amet consectetur adipisicing elit. Vero suscipit quae magni, nisi autem cupiditate necessitatibus molestias, delectus iste, mollitia debitis repellendus! Modi, eius sit. Rerum praesentium error vero natus!</p>
		</div>
	</div>
	
	<div class="flex flex-col gap-5 w-2/3">

		<!-- calendar-->
		<div class="bg-gray-50 rounded-lg shadow-xl h-2/4">
			<div class="ml-5 mt-3">
				<h1 class="text-left">Calendar</h1>
				<p>Lorem ipsum dolor sit, amet consectetur adipisicing elit. Vero suscipit quae magni, nisi autem cupiditate necessitatibus molestias, delectus iste, mollitia debitis repellendus! Modi, eius sit. Rerum praesentium error vero natus!</p>
			</div>
		</div>

		<!-- graph -->
		<div class="bg-gray-50 rounded-lg shadow-xl grow">
			<div class="ml-5 mt-3">
				<h1 class="text-left">Some Graph</h1>
				<p>Lorem ipsum dolor sit, amet consectetur adipisicing elit. Vero suscipit quae magni, nisi autem cupiditate necessitatibus molestias, delectus iste, mollitia debitis repellendus! Modi, eius sit. Rerum praesentium error vero natus!</p>
			</div>
		</div>
	</div>

	<!-- tasks -->
	<div class="bg-gray-50 rounded-lg shadow-xl w-2/3">
		<div class="ml-5 mt-3">
			<h1 class="text-left">My Tasks</h1>
			<p>Lorem ipsum dolor sit, amet consectetur adipisicing elit. Vero suscipit quae magni, nisi autem cupiditate necessitatibus molestias, delectus iste, mollitia debitis repellendus! Modi, eius sit. Rerum praesentium error vero natus!</p>
		</div>
	</div>
</div>

<style>
</style>
