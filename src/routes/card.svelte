<script>
    export let data;
</script>

<div class="bg-white p-5 m-2 rounded-lg border-6 shadow text-black">
    <div class="flex flex-row gap-5">
        <div class="flex flex-col">
            <h3>{data.name}</h3>
            <p>{data.age}, {data.sex}</p>
        </div>
        <p class="font-bold">{data.time} pm</p>
        <button class="bg-blue-400 text-white rounded-xl font-bold p-2"><i>Edit</i></button>
        <button class="bg-green-500 text-white rounded-xl font-bold p-2">Start</button>
    </div>
</div>