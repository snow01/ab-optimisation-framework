<script>
  export let userImage = "img/theme/team-1.jpg";
  export let userName = "Michael Lewis";
  export let text = "Cras sit amet nibh libero nulla vel metus scelerisque ante sollicitudin. Cras purus odio vestibulum in vulputate viverra turpis.";
  export let likeCount = 0;
  export let shareCount = 0;
</script>

<div>
  <div class="media media-comment">
    <img
      alt="Image placeholder"
      rounded="circle"
      class="avatar avatar-lg media-comment-avatar"
      src={userImage} />
    <div class="media-body">
      <div class="media-comment-text">
        <h6 class="h5 mt-0">{ userName }</h6>
        <p class="text-sm lh-160">{text}</p>
        <div class="icon-actions">
          <a href="/" class="like active">
            <i class="ni ni-like-2" />
            <span class="text-muted">{ likeCount } likes</span>
          </a>
          <a href="/">
            <i class="ni ni-curved-next" />
            <span class="text-muted">{ shareCount } shares</span>
          </a>
        </div>
      </div>
    </div>
  </div>
</div>
