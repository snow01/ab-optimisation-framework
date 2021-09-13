<script>
  export let columns = []; //'Table columns'
  export let data = []; //'Table data'
  export let type = ""; //'Whether table is striped or hover type'
  export let theadClasses = ""; //'<thead> css classes'
  export let tbodyClasses = ""; //<tbody> css classes'
  export let tableClasses = "";

  function hasValue(item, column) {
    return item[column.toLowerCase()] !== "undefined";
  }

  function itemValue(item, column) {
    return item[column.toLowerCase()];
  }
</script>

<table class="tabe tablesorter {tableClasses}">
  <thead class={theadClasses}>
    <tr>
      <slot name="column" :column={columns}>
        {#each columns as column}
          <th key={column}>{column}</th>
        {/each}
      </slot>
    </tr>
  </thead>
  <tbody class={tbodyClasses}>
    {#each data as item, index}
      <tr key={index}>
        <slot :row={item} :index={index}>
          {#each columns as column, index}
            {#if hasValue(item, column)}
              <td key={index}>{itemValue(item, column)}</td>
            {/if}
          {/each}
        </slot>
      </tr>
    {/each}
  </tbody>
</table>
