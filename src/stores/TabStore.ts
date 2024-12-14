// src/stores/TabStore.ts
import { writable } from 'svelte/store';

// Define the type for a tab
export type Tab = {
	id: string;
	title: string;
};

// Store to manage tabs
const TabStore = writable<Tab[]>([]);

// Active tab store to track the currently active tab
const ActiveTabStore = writable<string | null>(null);

// Function to add a tab
function addTab(tab: Tab): void {
	TabStore.update((tabs) => {
		const existingTab = tabs.find((t) => t.id === tab.id);
		if (!existingTab) {
			return [...tabs, tab];
		}
		return tabs;
	});

	// Set this tab as active
	ActiveTabStore.set(tab.id);
}

// Function to set a tab as active
function setActiveTab(tabId: string): void {
	ActiveTabStore.set(tabId);
}

// Function to remove a tab
function removeTab(tabId: string): void {
	TabStore.update((tabs) => tabs.filter((tab) => tab.id !== tabId));
	TabStore.update((tabs) => {
		if (tabs.length > 0) {
			ActiveTabStore.set(tabs[0].id); // Set the first tab as active if there are any remaining
		} else {
			ActiveTabStore.set(null);
		}
		return tabs;
	});
}

export { TabStore, ActiveTabStore, addTab, setActiveTab, removeTab };

