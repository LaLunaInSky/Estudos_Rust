<script
    lang="ts"
>
    import DynamicAttributes from "./components/introduction/dynamic_attributes.svelte";

    const name = "Svelte";
</script>

<div
    class="
        w-full h-50 pt-5
    "
>
    <div
        class="
            flex flex-col justify-center items-center gap-3 cursor-default select-none
        "
    >
        <h1
            class="
                uppercase text-slate-300 text-3xl
            "
        >
            introduction
        </h1>
        <h2
            class="
                text-neutral-50 text-center font-bold cursor-default select-none
            "
        >
            Hello, World!
            <br>
            I'm { name.toUpperCase() }!
        </h2>
        <div
            class="
                flex flex-wrap gap-3
            "
        >
            <DynamicAttributes />
        </div>
    </div>
</div>