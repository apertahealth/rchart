import { error } from '@sveltejs/kit';
import { enhance } from '$lib/form';
import type { PageLoad } from '@sveltejs/kit';

// see https://kit.svelte.dev/docs#loading
export const load: PageLoad = async ({ fetch }) => {
	const res = await fetch('/todos.json');

	if (res.ok) {
		const todos = await res.json();

		return { todos };
	}

	const { message } = await res.json();

	throw error(500, message);
};
