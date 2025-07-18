<script lang="ts">
    import "../app.css"

    let { children } = $props();

    let name = 'Svelte';
    let src = "/svelte.svg";
</script>

<header
    class="
        min-h-svh max-h-full bg-slate-50/30 w-60 pt-2
    "
>
    <div
        class="
            flex mb-3 justify-center items-center cursor-default
        "
    >
        <h1
            class="
                text-xl text-neutral-200 font-bold
            "
        >
            Hello {name}!
        </h1>
        <img
            {src}
            alt="image {name} icon"
            class="
                ml-5
            "
        >
    </div>
</header>
<main
    class="
        w-full py-10 flex flex-col justify-center items-center  *:select-none
    "
>
    {@render children()}
</main>

<style 
    lang="postcss"
>
    @reference "tailwindcss";
</style>