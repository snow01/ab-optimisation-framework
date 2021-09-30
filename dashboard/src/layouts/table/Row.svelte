<script>
	export let id = 'TODO';
</script>

<style>
    tr {
        border-top: 1px solid #ebeef5;
        transition: all 0.3s ease-in-out;
    }

    tr:hover {
        background-color: transparent !important;
        transition: all 0.3s ease-in-out;
    }
</style>

<tr {id}>
	<slot></slot>
</tr>