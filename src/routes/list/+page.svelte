<script context="module" lang="ts">
    export const prerender = true;
    import TabList from '../../lib/components/TabList.svelte';
    let comp;
	import { invoke } from '@tauri-apps/api/tauri';
	import TabStore from '../../stores/TabStore.js';

    let data = [
    {
        "number": "001",
        "pic": "",
        "first_name": "Logan",
        "last_name": "Nguyen",
        "DOB": "9/15/98",
        "Age": "24",
        "Sex": "M",
        "Gender": "M",
        "Address": "222 Main St. Detroit",
        "Phone Number": "555-555-5555",
        "Email": "lg@asd.com",
        "Last Seen": "5/5/23",
        "PCP": "Dr. Geedee"
    },{
        "number": "001",
        "pic": "",
        "first_name": "Sean",
        "last_name": "Nguyen",
        "DOB": "9/15/98",
        "Age": "24",
        "Sex": "M",
        "Gender": "M",
        "Address": "222 Main St. Detroit",
        "Phone Number": "555-555-5555",
        "Email": "lg@asd.com",
        "Last Seen": "5/5/23",
        "PCP": "Dr. Geedee"
    },{
        "number": "001",
        "pic": "",
        "first_name": "Evan",
        "last_name": "Nguyen",
        "DOB": "9/15/98",
        "Age": "24",
        "Sex": "M",
        "Gender": "M",
        "Address": "222 Main St. Detroit",
        "Phone Number": "555-555-5555",
        "Email": "lg@asd.com",
        "Last Seen": "5/5/23",
        "PCP": "Dr. Geedee"
    },
    {
        "number": "001",
        "pic": "",
        "first_name": "Don",
        "last_name": "Nguyen",
        "DOB": "9/15/98",
        "Age": "24",
        "Sex": "M",
        "Gender": "M",
        "Address": "222 Main St. Detroit",
        "Phone Number": "555-555-5555",
        "Email": "lg@asd.com",
        "Last Seen": "5/5/23",
        "PCP": "Dr. Geedee"
    },
    {
        "number": "001",
        "pic": "",
        "first_name": "Mark",
        "last_name": "Nguyen",
        "DOB": "9/15/98",
        "Age": "24",
        "Sex": "M",
        "Gender": "M",
        "Address": "222 Main St. Detroit",
        "Phone Number": "555-555-5555",
        "Email": "lg@asd.com",
        "Last Seen": "5/5/23",
        "PCP": "Dr. Geedee",
        "HealthScore": "2"
    },
    {
        "number": "001",
        "pic": "",
        "first_name": "Maxine",
        "last_name": "Nguyen",
        "DOB": "9/15/98",
        "Age": "24",
        "Sex": "M",
        "Gender": "M",
        "Address": "222 Main St. Detroit",
        "Phone Number": "555-555-5555",
        "Email": "lg@asd.com",
        "Last Seen": "5/5/23",
        "PCP": "Dr. Geedee",
        "HealthScore": "2"
    }
    ];
</script>
<TabList bind:this={comp}></TabList>
<div class="">
    <table class="overflow-hidden absolute left-20 top-20 mx-5 my-8 rounded-lg shadow w-[92%]">
        <thead class="bg-gray-50 border-b-2 border-gray-200">
            <tr>			

                <th class="p-3 text-sm font-semibold tracking-wide text-left">#</th>
                <th class="p-3 text-sm font-semibold tracking-wide text-left">pic</th>
                <th class="p-3 text-sm font-semibold tracking-wide text-left">First Name</th>
                <th class="p-3 text-sm font-semibold tracking-wide text-left">Last Name</th>
                <th class="p-3 text-sm font-semibold tracking-wide text-left">DOB</th>
                <th class="p-3 text-sm font-semibold tracking-wide text-left">Age</th>
                <th class="p-3 text-sm font-semibold tracking-wide text-left">Sex</th>
                <th class="p-3 text-sm font-semibold tracking-wide text-left">Gender</th>
                <th class="p-3 text-sm font-semibold tracking-wide text-left">Address</th>
                <th class="p-3 text-sm font-semibold tracking-wide text-left">Phone Number</th>
                <th class="p-3 text-sm font-semibold tracking-wide text-left">Email</th>
                <th class="p-3 text-sm font-semibold tracking-wide text-left">Last Seen</th>
                <th class="p-3 text-sm font-semibold tracking-wide text-left">Physician</th>
                <th class="p-3 text-sm font-semibold tracking-wide text-left">Health Score</th>
            </tr>
        </thead>
        <tbody class="divide-y divide-gray-100">
            {#each Object.values(data) as row}
                <tr on:click="{() => {comp.addTab()}}" class="bg-white">
                    {#each Object.values(row) as cell}
                        <td class="px-3 py-5 text-sm text-left text-gray-700 whitespace-nowrap">{cell}</td>
                    {/each}
                </tr>
            {/each}                
        </tbody>
    </table>
</div>