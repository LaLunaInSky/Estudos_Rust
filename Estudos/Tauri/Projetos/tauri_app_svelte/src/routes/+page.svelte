<script 
    lang="ts"
>
    import "../app.css";
    import BasicSvelte from "./basic_svelte/BasicSvelte.svelte";
    import BasicSvelteKit from "./basic_sveltekit/BasicSvelteKit.svelte";

    let name = 'Svelte';
    let src = "/svelte.svg";
</script>

<main
    class="
        bg-gray-700 py-10 flex flex-col justify-center items-center  *:select-none
    "
>
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
</main>

<style 
    lang="postcss"
>
    @reference "tailwindcss";
</style>