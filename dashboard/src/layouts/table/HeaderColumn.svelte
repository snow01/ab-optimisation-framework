<script>
	export let sortable = false;
	export let name = 'TODO';
	export let width = 1;
</script>

<style>
    th {
        font-weight: 600;
        padding: 0px 1.5rem !important;
        width: auto;
        min-width: auto;
        white-space: nowrap;
    }

    .caret-wrapper {
        display: -webkit-inline-box;
        display: -ms-inline-flexbox;
        display: inline-flex;
        -webkit-box-orient: vertical;
        -webkit-box-direction: normal;
        -ms-flex-direction: column;
        flex-direction: column;
        -webkit-box-align: center;
        -ms-flex-align: center;
        align-items: center;
        height: 34px;
        width: 24px;
        vertical-align: middle;
        cursor: pointer;
        overflow: initial;
        position: relative
    }

    .sort-caret {
        width: 0;
        height: 0;
        border: 5px solid transparent;
        position: absolute;
        left: 7px
    }

    .sort-caret.ascending {
        border-bottom-color: #c0c4cc;
        top: 5px
    }

    .sort-caret.descending {
        border-top-color: #c0c4cc;
        bottom: 7px
    }

    .sorting_asc .sort-caret.ascending {
        border-bottom-color: #172b4d
    }

    .sorting_desc .sort-caret.descending {
        border-top-color: #172b4d
    }
</style>

<th
	class='border-bottom-0 col-{width}' rowspan='1' colspan='1'>
	{name}
	{#if sortable}
	<span class='caret-wrapper'>
		<i class='sort-caret ascending' />
		<i class='sort-caret descending' />
	</span>
	{/if}
</th>

