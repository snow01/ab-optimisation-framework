<script>
  export let animationDuration = 250; //'Collapse animation duration'
  export let multipleActive = true; //'Whether you can have multiple collapse items opened at the same time'
  export let activeIndex = -1; //'Active collapse item index'
  import { onMount } from "svelte";

  function provide() {
    return {
      animationDuration: animationDuration,
      multipleActive: multipleActive,
      addItem: addItem,
      removeItem: removeItem,
      deactivateAll: deactivateAll
    };
  }

  function data() {
    return {
      items: []
    };
  }

  function addItem(item) {
    const index = $slots.default.indexOf(item.$vnode);
    if (index !== -1) {
      items.splice(index, 0, item);
    }
  }
  function removeItem(item) {
    const items = items;
    const index = items.indexOf(item);
    if (index > -1) {
      items.splice(index, 1);
    }
  }

  function deactivateAll() {
    items.forEach(item => {
      item.active = false;
    });
  }

  function activateItem() {
    if (activeIndex !== -1) {
      items[activeIndex].active = true;
    }
  }

  onMount(() => {
    activateItem();
  });
</script>


<div id="accordion" role="tablist" aria-multiselectable="true" class="accordion">
       <slot></slot>
</div>