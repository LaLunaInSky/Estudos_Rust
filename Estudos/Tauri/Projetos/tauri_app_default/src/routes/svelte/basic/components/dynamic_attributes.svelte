<script
    lang="ts"
>

    const name = "Rick Astley";
    const src = "https://svelte.dev/tutorial/image.gif";
</script>

<div>
    <img 
        {src} 
        alt="{name} dance"
        class="
            border-4 border-slate-400/50 rounded-2xl
        "
    />
</div>