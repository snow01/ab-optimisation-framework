<script>
  import { onMount } from "svelte";
  import { fly } from "svelte/transition";
  export let id = "";
  export let value = [];
  export let readonly = false;
  export let placeholder = "";

  let input,
    inputValue,
    options = [],
    activeOption,
    showOptions = false,
    selected = {},
    first = true,
    slot;
  const iconClearPath =
    "M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z";

  onMount(() => {
    slot.querySelectorAll("option").forEach(o => {
      o.selected && !value.includes(o.value) && (value = [...value, o.value]);
      options = [...options, { value: o.value, name: o.textContent }];
    });
    value &&
      (selected = options.reduce(
        (obj, op) =>
          value.includes(op.value) ? { ...obj, [op.value]: op } : obj,
        {}
      ));
    first = false;
  });

  $: if (!first) value = Object.values(selected).map(o => o.value);
  $: filtered = options.filter(o =>
    inputValue ? o.name.toLowerCase().includes(inputValue.toLowerCase()) : o
  );
  $: if (
    (activeOption && !filtered.includes(activeOption)) ||
    (!activeOption && inputValue)
  )
    activeOption = filtered[0];

  function add(token) {
    if (!readonly) selected[token.value] = token;
  }

  function remove(value) {
    if (!readonly) {
      const { [value]: val, ...rest } = selected;
      selected = rest;
    }
  }

  function optionsVisibility(show) {
    if (readonly) return;
    if (typeof show === "boolean") {
      showOptions = show;
      show && input.focus();
    } else {
      showOptions = !showOptions;
    }
    if (!showOptions) {
      activeOption = undefined;
    }
  }

  function handleKeyup(e) {
    if (e.keyCode === 13) {
      Object.keys(selected).includes(activeOption.value)
        ? remove(activeOption.value)
        : add(activeOption);
      inputValue = "";
    }
    if ([38, 40].includes(e.keyCode)) {
      // up and down arrows
      const increment = e.keyCode === 38 ? -1 : 1;
      const calcIndex = filtered.indexOf(activeOption) + increment;
      activeOption =
        calcIndex < 0
          ? filtered[filtered.length - 1]
          : calcIndex === filtered.length
          ? filtered[0]
          : filtered[calcIndex];
    }
  }

  function handleBlur(e) {
    optionsVisibility(false);
    document.getElementById("inputArrow").classList.remove("el-icon-arrow-up");
    document.getElementById("inputArrow").classList.add("el-icon-arrow-down");
    document.getElementById("tokens").classList.remove("border-primary");
  }

  function handleTokenClick(e) {
    if (e.target.closest(".token-remove")) {
      e.stopPropagation();
      remove(e.target.closest(".token").dataset.id);
    } else if (e.target.closest(".remove-all")) {
      selected = [];
      inputValue = "";
    } else {
      optionsVisibility(true);
      document.getElementById("inputArrow").classList.add("el-icon-arrow-up");
      document.getElementById("inputArrow").classList.remove("el-icon-arrow-down");
      document.getElementById("tokens").classList.add("border-primary");
    }
  }

  function handleOptionMousedown(e) {
    const value = e.target.dataset.value;
    if (selected[value]) {
      remove(value);
    } else {
      add(options.filter(o => o.value === value)[0]);
      input.focus();
    }
  }
</script>

<style>
  .multiselect {
    background-color: white;
    position: relative;
    min-height: 43px;
  }

  .tokens {
    padding-top: 6px;
    min-height: 43px;
    padding-bottom: 6px;
    padding-left: 10px;
    align-items: center;
    display: flex;
    flex-wrap: wrap;
    position: relative;
  }
  .tokens::after {
    background: none repeat scroll 0 0 transparent;
    bottom: -1px;
    content: "";
    display: block;
    height: 2px;
    left: 50%;
    position: absolute;
    transition: width 0.3s ease 0s, left 0.3s ease 0s;
    width: 0;
  }
  .tokens.showOptions::after {
    width: 100%;
    left: 0;
  }
  .token {
    align-items: center;
    background-color: #172b4d;
    color: white;
    border-radius: 1.25rem;
    display: flex;
    height: 25px;
    margin: 0.125rem;
    padding: 0.625rem 0.625rem 0.5rem;
    transition: background-color 0.3s;
    white-space: nowrap;
  }

  .token i.el-tag__close {
    background: transparent;
  }

  .readonly .token {
    color: hsl(0, 0%, 40%);
  }
  .token-remove,
  .remove-all {
    align-items: center;
    background-color: hsl(214, 15%, 55%);
    border-radius: 50%;
    color: hsl(214, 17%, 92%);
    display: flex;
    justify-content: center;
    height: 1.25rem;
    margin-left: 0.25rem;
    min-width: 1.25rem;
  }
  .token-remove:hover,
  .remove-all:hover {
    background-color: hsl(215, 21%, 43%);
    cursor: pointer;
  }

  .actions {
    align-items: center;
    display: flex;
    flex: 1;
    min-width: 15rem;
  }

  input {
    border: none;
    font-size: 1.5rem;
    line-height: 1.5rem;
    margin: 0;
    outline: none;
    padding: 0;
    width: 100%;
    min-height: 43px;
  }

  .dropdown-arrow path {
    fill: hsl(0, 0%, 70%);
  }
  .multiselect:hover .dropdown-arrow path {
    fill: hsl(0, 0%, 50%);
  }

  .icon-clear path {
    fill: white;
  }

  .options {
    box-shadow: 0px 2px 4px rgba(0, 0, 0, 0.1), 0px -2px 4px rgba(0, 0, 0, 0.1);
    left: 0;
    list-style: none;
    margin-block-end: 0;
    margin-block-start: 0;
    max-height: 70vh;
    overflow: auto;
    padding-inline-start: 0;
    position: absolute;
    top: calc(100% + 1px);
    width: 100%;
  }
  li {
    font-size: 14px;
    padding: 0 20px;
    position: relative;
    overflow: hidden;
    text-overflow: ellipsis;
    color: #606266;
    height: 34px;
    line-height: 34px;
    -webkit-box-sizing: border-box;
    box-sizing: border-box;
    cursor: pointer;
    background-color: white;
  }
  li:first-child {
    margin-top: 10px;
  }
  li:hover {
    background-color: #f7fafc;
  }
  li:last-child {
    border-bottom-left-radius: 0.2rem;
    border-bottom-right-radius: 0.2rem;
  }
  li.selected {
    color: #6175e4;
    font-weight: 700;
  }
  li.selected:after {
    position: absolute;
    right: 20px;
    font-family: element-icons;
    content: "\E611";
    font-size: 12px;
    font-weight: 700;
    -webkit-font-smoothing: antialiased;
  }
  .hidden {
    display: none;
  }
</style>

<div class="multiselect border rounded" class:readonly id="tokens">
  <div
    class="tokens el-select rounded"
    class:showOptions
    on:click={handleTokenClick}>
    {#each Object.values(selected) as s}
      <span
        class="token el-tag el-tag--info el-tag--small rounded"
        data-id={s.value}>
        <span>{s.name}</span>
        {#if !readonly}
          <i
            class="el-tag__close el-icon-close text-white token-remove"
            title="Remove {s.name}">
            <path d={iconClearPath} />
          </i>
        {/if}
      </span>
    {/each}
    <div class="actions">
      {#if !readonly}
        <input
          {id}
          autocomplete="off"
          bind:value={inputValue}
          bind:this={input}
          on:keyup={handleKeyup}
          on:blur={handleBlur}
          {placeholder}
          style="position: absolute; z-index: -1;" />
        <span class="el-input__suffix">
          <span class="el-input__suffix-inner">
            <i
              class="el-select__caret el-input__icon el-icon-arrow-down"
              id="inputArrow" />
          </span>
        </span>
      {/if}
    </div>
  </div>

  <select bind:this={slot} type="multiple" class="hidden">
    <slot />
  </select>

  {#if showOptions}
    <ul
      class="options"
      style="z-index: 100000;"
      transition:fly={{ duration: 200, y: 5 }}
      on:mousedown|preventDefault={handleOptionMousedown}>
      {#each filtered as option}
        <li
          class:selected={selected[option.value]}
          class:active={activeOption === option}
          data-value={option.value}>
          {option.name}
        </li>
      {/each}
    </ul>
  {/if}
</div>
