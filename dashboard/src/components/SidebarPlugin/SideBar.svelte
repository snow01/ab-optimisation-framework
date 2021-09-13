<script>
  //   export let title = "Creative Tim"; //'Sidebar title'
  //   export let shortTile = "CT"; //'Sidebar short title'
  export let logo =
    "https://raw.githubusercontent.com/creativetimofficial/public-assets/master/argon-dashboard-pro-svelte/red.png";
  export let backgroundColor = "red"; //'Sidebar background color (blue|green|orange|red|primary)'
  backgroundColor = ValidateBackground(backgroundColor);
  export let sidebarLinks = []; //"List of sidebar links as an array if you don't want to use components for these."
  //   export let autoClose = true; // 'Whether sidebar should autoclose on mobile when clicking an item'
  import SideBarItem from "./SideBarItem.svelte";

  import { createEventDispatcher } from "svelte";
  const dispatch = createEventDispatcher();

  const Clicked = () => {
    dispatch("click");
  };

  let mediaButton = false;
  export let toggle = true;

  function ValidateBackground(backgroundColor) {
    let acceptedValues = [
      "",
      "blue",
      "green",
      "orange",
      "red",
      "primary"
    ];

    if (acceptedValues.indexOf(backgroundColor) !== -1) {
      return backgroundColor;
    } else {
      return null;
    }
  }

  function toggleMenu() {
    if (toggle === true) {
      toggle = false;
      document.body.classList = "g-sidenav-hidden";
      if (window.innerWidth > 1200) {
        let collapse = document.getElementsByClassName("sidebarcollapse");
        for (var i = 0; i < collapse.length; i++) {
          collapse[i].classList.add("d-none");
        }
      } else {
        let collapse = document.getElementsByClassName("sidebarcollapse");
        for (var i = 0; i < collapse.length; i++) {
          collapse[i].classList.remove("d-none");
        }
      }
    } else {
      toggle = true;
      document.body.classList = "g-sidenav-pinned g-sidenav-show";
      if (window.innerWidth > 1200) {
        let collapse = document.getElementsByClassName("sidebarcollapse");
        for (var i = 0; i < collapse.length; i++) {
          collapse[i].classList.remove("d-none");
        }
      }
    }
    Clicked();
  }

  function hoverMenu() {
    if (window.innerWidth < 576) {
      if (toggle === false) {
        document.body.classList = "g-sidenav-pinned g-sidenav-show";
      }
      let collapse = document.getElementsByClassName("sidebarcollapse");
      for (var i = 0; i < collapse.length; i++) {
        collapse[i].classList.remove("d-none");
      }
    } else {
      if (toggle === false) {
        document.body.classList = "g-sidenav-pinned g-sidenav-show";
        let collapse = document.getElementsByClassName("sidebarcollapse");
        for (var i = 0; i < collapse.length; i++) {
          collapse[i].classList.remove("d-none");
        }
      }
    }
  }

  function blurMenu() {
    if (toggle === false) {
      if (window.innerWidth > 1200) {
        document.body.classList = "g-sidenav-hidden";
        let collapse = document.getElementsByClassName("sidebarcollapse");
        for (var i = 0; i < collapse.length; i++) {
          collapse[i].classList.add("d-none");
        }
      } else {
        let collapse = document.getElementsByClassName("sidebarcollapse");
        for (var i = 0; i < collapse.length; i++) {
          collapse[i].classList.remove("d-none");
        }
      }
    }
  }

  if (window.innerWidth < 576) {
    mediaButton = true;
    let collapse = document.getElementsByClassName("sidebarcollapse");
    for (var i = 0; i < collapse.length; i++) {
      collapse[i].classList.remove("d-none");
    }
  } else {
    mediaButton = false;
  }

  if (window.innerWidth < 1200) {
    let collapse = document.getElementsByClassName("sidebarcollapse");
    for (var i = 0; i < collapse.length; i++) {
      collapse[i].classList.remove("d-none");
    }
  }

  if (document.body.classList.contains("g-sidenav-hidden")) {
    toggle = false;
  } else {
    toggle = true;
  }

  window.addEventListener("resize", function() {
    if (window.innerWidth < 576) {
      mediaButton = true;
      let collapse = document.getElementsByClassName("sidebarcollapse");
      for (var i = 0; i < collapse.length; i++) {
        collapse[i].classList.remove("d-none");
      }
    } else {
      mediaButton = false;
    }

    if (window.innerWidth < 1200) {
      let collapse = document.getElementsByClassName("sidebarcollapse");
      for (var i = 0; i < collapse.length; i++) {
        collapse[i].classList.remove("d-none");
      }
    }

    if (document.body.classList.contains("g-sidenav-hidden")) {
      toggle = false;
      let collapse = document.getElementsByClassName("sidebarcollapse");
      for (var i = 0; i < collapse.length; i++) {
        collapse[i].classList.add("d-none");
      }
    } else {
      toggle = true;
      let collapse = document.getElementsByClassName("sidebarcollapse");
      for (var i = 0; i < collapse.length; i++) {
        collapse[i].classList.remove("d-none");
      }
    }
  });
</script>

<style>
  .scrollbar-inner {
    overflow-y: scroll;
    overflow-x: hidden;
  }

  .scrollbar-inner:hover::-webkit-scrollbar-thumb {
    background-color: #c6ccd2;
  }

  .scrollbar-inner::-webkit-scrollbar-track {
    border: 1px solid #ffffff;
    padding: 2px 0;
    background-color: #ffffff;
  }

  .scrollbar-inner::-webkit-scrollbar {
    width: 6.5px;
  }

  .scrollbar-inner::-webkit-scrollbar-thumb {
    border-radius: 10px;
    background-color: #fff;
    min-height: 550px;
  }
</style>

<div
  class="sidenav navbar navbar-vertical fixed-left navbar-expand-xs navbar-light
  bg-white"
  on:mouseenter={hoverMenu}
  on:mouseleave={blurMenu}
  data={backgroundColor}>
  <div class="scrollbar-inner" ref="sidebarScrollArea">
    <div class="sidenav-header d-flex align-items-center">
      <a href="/" class="navbar-brand">
        <img src={logo} alt="Sidebar logo" class="navbar-brand-img" />
      </a>
      <div class="ml-auto">
        <div
          class="sidenav-toggler {mediaButton === true ? 'd-block' : 'd-none'}
          d-xl-block {toggle === true ? 'active' : ''}"
          on:click={toggleMenu}>
          <div class="sidenav-toggler-inner">
            <i class="sidenav-toggler-line" />
            <i class="sidenav-toggler-line" />
            <i class="sidenav-toggler-line" />
          </div>
        </div>
      </div>
    </div>
    <slot />
    <div class="navbar-inner">
      <slot name="links">
        {#each sidebarLinks as link, index}
          <SideBarItem key={link.name + index} {link}>
            {#each link.children as subLink, index}
              <SideBarItem key={subLink.name + index} link={subLink} />
            {/each}
          </SideBarItem>
        {/each}
      </slot>
      <slot name="links-after" />
    </div>
  </div>
</div>
