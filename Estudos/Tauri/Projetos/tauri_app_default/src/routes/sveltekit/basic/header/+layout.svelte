<script
    lang="ts"
>
    let { children } = $props();
</script>

<div
    class="
        w-full pt-10 flex justify-center items-center 
    "
>
    {@render children()}
</div>