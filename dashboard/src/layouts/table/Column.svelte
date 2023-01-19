<script>
	export let width = 1;
</script>

<style>
    td {
        padding: 1rem 1.5rem !important;
        border-top: 1px solid #e9ecef;

        width: auto;
        min-width: auto;
        white-space: nowrap;
        box-sizing: border-box;
        word-break: break-all;
        line-height: 23px;
        text-align: left;
        color: #525f7f;
        font-size: 0.8125rem;
    }
</style>

<td
	class='col-{width}'
	rowspan='1'
	colspan='1'>
	<slot></slot>
</td>