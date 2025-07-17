<script lang="ts">
    import "../app.css";

    let name = 'Svelte';
    let src = "/svelte.svg";
</script>

<main
    class="
        bg-gray-700 h-svh flex justify-center items-center *:select-none
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
</main>

<style lang="postcss">
    @reference "tailwindcss";
</style>