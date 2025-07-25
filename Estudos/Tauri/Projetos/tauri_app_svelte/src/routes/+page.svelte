<script 
    lang="ts"
>
    import BasicSvelte from "./basic_svelte/BasicSvelte.svelte";
    import BasicSvelteKit from "./basic_sveltekit/BasicSvelteKit.svelte";
</script>

<div
    class="
        flex mb-3 
    "
>
    <h1
        class="
            text-2xl text-neutral-200 font-bold
        "
    >
        Page Home!
    </h1>
</div>
<div
    class="
        flex flex-wrap gap-4
    "
>
    <div>
        <BasicSvelte />
    </div>
    <div>
        <BasicSvelteKit />
    </div>
</div>