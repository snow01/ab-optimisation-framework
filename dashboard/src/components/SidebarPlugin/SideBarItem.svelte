<script>
  export let menu = false; // "Whether the item is a menu. Most of the item it's not used and should be used only if you want to override the default behavior."
  export let link = {
    name: "",
    path: "",
    children: 0,
    isActive: false,
    single: false
  }; //'Sidebar link. Can contain name, path, icon and other attributes. See examples for more info'
  import { fly } from "svelte/transition";
  import {v1 as uuidv1} from "uuid";
  let parentId = uuidv1();
  let childrenId = uuidv1();
  let sublinkId = uuidv1();
  import { onMount } from "svelte";
  menu = isMenu();
  let addLink = false;
  let collapsed = false;
  if (link.isActive === true) {
    collapsed = true;
  }

  function isMenu() {
    if (link.children != 0) {
      menu = true;
      return true;
    } else {
      return false;
    }
  }
  onMount(async () => {
    if (collapsed === true) {
      const parent = document.getElementById(parentId);
      const children = document.getElementById(childrenId);
      parent.style.height = children.clientHeight + "px";
    }
  });

  function collapseSubMenu() {
    collapsed = !collapsed;
    const parent = document.getElementById(parentId);
    if (collapsed === true) {
      const children = document.getElementById(childrenId);
      parent.style.height = children.scrollHeight + "px";
      if (
        !parent.parentElement.parentElement.classList.contains("navbar-nav")
      ) {
        let list = document.querySelectorAll("li.sidebar-item");
        for (let i = 0; i < list.length; i++) {
          let childrens = list[i].getElementsByClassName("sidebarcollapse");
          let height = 0;
          let newheight = 0;
          for (let j = 0; j < childrens.length; j++) {
            height = height + childrens[j].scrollHeight;
            let str = childrens[j].style.height;
            str.replace("px", "");
            str = parseInt(str);
            newheight = str + newheight;
          }
          childrens[0].style.height = height + "px";
        }
      }
    } else {
      let parentHeight = parent.style.height;
      parentHeight = parentHeight.replace("px", "");
      parentHeight = parseInt(parentHeight);
      parent.style.height = 0 + "px";
      if (
        !parent.parentElement.parentElement.classList.contains("navbar-nav")
      ) {
        let list = parent.parentElement.parentElement.querySelectorAll(
          "li.sidebar-item"
        );
        for (let i = 0; i < list.length; i++) {
          let childrens = list[i].getElementsByClassName("sidebarcollapse");
          let height = parentHeight;
          let newheight = childrens[0].style.height;
          newheight = newheight.replace("px", "");
          newheight = parseInt(newheight);
          let collapse = parentHeight - newheight;
          if (collapse === parentHeight) {
            collapse = 0;
          }
          childrens[0].style.height = collapse + "px";
          let menuHeight =
            parent.parentElement.parentElement.parentElement.style.height;
          menuHeight = menuHeight.replace("px", "");
          menuHeight = parseInt(menuHeight);
          menuHeight = menuHeight - parentHeight;
          parent.parentElement.parentElement.parentElement.style.height =
            menuHeight + "px";
        }
      }
    }
  }

  if (menu === true) {
    addLink = true;
  } else {
    addLink = false;
  }

  function activateMenu(e, sublinkId) {
    if (
      document
        .getElementById(sublinkId)
        .parentElement.parentElement.parentElement.parentElement.parentElement.childNodes[0].classList.contains(
          "sidebar-menu-item"
        )
    ) {
      let sidebarItems = document.querySelectorAll(
        "a.sidebar-menu-item.active"
      );
      for (let i = 0; i < sidebarItems.length; i++) {
        sidebarItems[i].classList.remove("active");
      }
      let singleSidebarItems = document.querySelectorAll(
        "div.singlesidebarItem"
      );
      for (let j = 0; j < singleSidebarItems.length; j++) {
        singleSidebarItems[j].classList.remove("active");
      }
      document
        .getElementById(sublinkId)
        .parentElement.parentElement.parentElement.parentElement.parentElement.childNodes[0].classList.add(
          "active"
        );
    }
    link.isActive = true;
    document.getElementById(sublinkId).parentElement.classList.add("active");
  }

  function activateSingleMenu(e, sublinkId) {
    let sidebarItems = document.querySelectorAll("a.sidebar-menu-item.active");
    for (let i = 0; i < sidebarItems.length; i++) {
      sidebarItems[i].classList.remove("active");
    }
    let singleSidebarItems = document.querySelectorAll("div.singlesidebarItem");
    for (let j = 0; j < singleSidebarItems.length; j++) {
      singleSidebarItems[j].classList.remove("active");
    }
    document.getElementById(sublinkId).parentElement.classList.add("active");
  }
</script>

<style>
  .sidebar-menu-item {
    cursor: pointer;
  }
  .sidebarcollapse {
    -moz-transition: height 0.5s;
    -ms-transition: height 0.5s;
    -o-transition: height 0.5s;
    -webkit-transition: height 0.5s;
    transition: height 0.5s;
    height: 0;
    overflow: hidden;
  }
</style>

<li
  class="nav-item {collapsed === true ? 'sidebar-item' : ''}
  {link.isActive === true ? 'active' : ''}">
  {#if menu === true}
    <a
      href="#!"
      class="sidebar-menu-item nav-link {link.isActive === true ? 'active' : ''}"
      aria-expanded={collapsed}
      data-toggle="collapse"
      aria-controls={parentId}
      aria-label="Toggle navigation"
      data-target={parentId}
      on:click={collapseSubMenu}>
      <i class={link.icon} />
      <span class="nav-link-text second">
        {link.name}
        <b class="caret" />
      </span>
    </a>
  {:else}
    <div class="d-flex flex-row nav-link p-0 m-0 singlesidebarItem {link.single === true && link.isActive === true ? "active" : ""}">
      {#if link.single === true}
        <div style="padding: .675rem 1.5rem; padding-right: 1rem;">
          <i class={link.icon} />
        </div>
      {/if}
      {#if link.single === true}
        <a
          href={link.path}
          class="sidebar-menu-item nav-link"
          on:click={e => activateSingleMenu(e, sublinkId)}
          id={sublinkId}
          style="padding-left: {link.single === true ? 0 : ''}px">
          {link.name}
        </a>
      {:else}
        <a
          href={link.path}
          class="sidebar-menu-item nav-link"
          on:click={e => activateMenu(e, sublinkId)}
          id={sublinkId}
          style="padding-left: {link.single === true ? 0 : ''}px">
          {link.name}
        </a>
      {/if}

    </div>
  {/if}
  {#if menu === true}
    <div class="sidebarcollapse" id={parentId}>
      <ul class="nav nav-sm flex-column" id={childrenId}>
        <slot />
      </ul>
    </div>
  {/if}
</li>
