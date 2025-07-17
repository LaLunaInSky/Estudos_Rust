<script lang="ts">
    import "../app.css";

    let name = 'Svelte';
</script>

<main
    class="
        bg-gray-700 h-svh flex justify-center items-center
    "
>
    <h1
        class="
            text-2xl text-neutral-200 font-bold select-none
        "
    >
        Hello {name}!
    </h1>
</main>

<style lang="postcss">
    @reference "tailwindcss";
</style>