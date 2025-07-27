<script
    lang="ts"
>
    import DynamicAttributes from "./components/dynamic_attributes.svelte";

    const name = "Svelte";
</script>

<div
    class="
        w-full h-50 pt-5
    "
>
    <h1
        class="
            text-neutral-50 text-center font-bold cursor-default select-none
        "
    >
        Hello, World! 
        <br>
        I'm { name.toUpperCase() }!
    </h1>

    <div
        class="
            flex flex-wrap gap-3 
        "
    >
        <DynamicAttributes />
    </div>
</div>