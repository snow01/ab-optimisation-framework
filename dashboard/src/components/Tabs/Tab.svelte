<script>
	import { getContext } from 'svelte';
	import { TABS } from './Tabs.svelte';

	const tab = {};
	const { registerTab, selectTab, selectedTab } = getContext(TABS);

	registerTab(tab);
</script>

<div class:selected="{$selectedTab === tab}" class="tab-pane" on:click="{() => selectTab(tab)}">
	<slot></slot>
</div>
