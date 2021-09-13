<script>
  export let pageCount = 0; //"Pagination page count. This should be specified in combination with perPage"
  export let perPage = 10; //"Pagination per page. Should be specified with total or pageCount"
  export let total = 0; //"Can be specified instead of pageCount. The page count in this case will be total/perPage"
  export let value = 1; //"Pagination value"
  // export let size; //"Pagination size"
  export let align; // "Pagination alignment (e.g center|start|end)"
  export let currentPage;
  export let pages;
  export let className;
  let next = false;
  let items = [];

  if(pageCount === 0){
    for (var i = 0; i < total / perPage; i++) {
      items.push(i + 1);
    }
    if (value === total / perPage) {
      next = true;
    }
  } else {
    for (var i = 0; i < pages; i++) {
      items.push(i + 1);
    }
    if (value === pages) {
      next = true;
    }
  }

  function nextPage() {
    if (currentPage !== total / perPage) {
      currentPage = currentPage + 1;
    } else {
      currentPage = currentPage;
    }
  }

  function prevPage() {
    if (currentPage !== 1) {
      currentPage = currentPage - 1;
    } else {
      currentPage = currentPage;
    }
  }
</script>

<ul
  role="menubar"
  aria-disabled="false"
  aria-label="Pagination"
  class="pagination b-pagination {align !== '' ? `align-items-${align}` : ''} {className}">
  <li
    role="presentation"
    class="page-item {currentPage === 1 ? 'disabled' : ''}"
    on:click={() => prevPage()}>
    <a href="#!" class="page-link" aria-label="Previous">
      <span aria-hidden="true">
        <i class="fa fa-angle-left" aria-hidden="true" />
      </span>
    </a>
  </li>
  {#each items as item}
    <li
      role="presentation"
      class="page-item {item === currentPage ? 'active' : ''}"
      on:click={() => ((value = item), (currentPage = item))}>
      <button
        role="menuitemradio"
        type="button"
        aria-label="Go to page {item}"
        aria-checked="true"
        aria-posinset="1"
        aria-setsize="5"
        tabindex="0"
        class="page-link">
        {item}
      </button>
    </li>
  {/each}
  <li
    role="presentation"
    class="page-item {currentPage === total / perPage ? 'disabled' : ''}"
    on:click={() => nextPage()}>
    <a href="#!" class="page-link" aria-label="Previous">
      <span aria-hidden="true">
        <i class="fa fa-angle-right" aria-hidden="true" />
      </span>
    </a>
  </li>
</ul>
