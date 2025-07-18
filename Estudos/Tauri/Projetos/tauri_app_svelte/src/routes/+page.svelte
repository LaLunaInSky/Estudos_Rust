<script lang="ts">
    import "../app.css";
    import Nested from "./components/Nested.svelte";
    import Counted from "./components/Counted.svelte";
    import Number from "./components/Number.svelte";

    let name = 'Svelte';
    let src = "/svelte.svg";
</script>

<main
    class="
        bg-gray-700 h-svh flex flex-col justify-center items-center *:select-none
    "
>
    <div
        class="
            flex 
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
            flex flex-col justify-center items-center
        "
    >
        <Nested />
        <Counted />
        <Number />
    </div>
</main>

<style lang="postcss">
    @reference "tailwindcss";
</style>