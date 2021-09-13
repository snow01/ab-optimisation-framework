<script>
  import { slide } from "svelte/transition";
  import Notification from "./Notification.svelte";
  export let notification = {};
  import { getNotificationsContext } from "svelte-notifications";


  const {
    addNotification,
    removeNotification,
    clearNotifications,
    subscribe
  } = getNotificationsContext();

  const handleButtonClick = (id) => {
    removeNotification(id);
  };

</script>

<style>
  .top-right-notifications.icon {
    min-height: 75px!important;
  }
</style>

<div class="top-right-notifications {notification.icon ? "icon" : "" }" transition:slide>
  <Notification
    on:remove={handleButtonClick(notification.id)}
    dismissible={true}
    type={notification.type !== "" ? notification.type : "info"}
    icon={notification.icon ? notification.icon : ""}
    dataNotify={true}
    notifyClassNames={notification.notifyClassNames} id={notification.id}>
    <span>{notification.text}</span>
  </Notification>
</div>
