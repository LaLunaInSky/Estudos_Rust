<script
    lang="ts"
>
    /* Introction */
    import DynamicAttributes from "./components/introduction/dynamic_attributes.svelte";
    import YourFirstComponent from "./components/introduction/your_first_component.svelte";
    import Styling from "./components/introduction/styling.svelte";

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
        <YourFirstComponent />
        <div
            class="
                flex flex-wrap gap-4
            "
        >
            <DynamicAttributes />
            <Styling />
        </div>
    </div>
</div>