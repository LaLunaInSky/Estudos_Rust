<script
    lang="ts"
>
    /* Introduction */
    import DynamicAttributes from "./components/introduction/dynamic_attributes.svelte";
    import YourFirstComponent from "./components/introduction/your_first_component.svelte";
    import Styling from "./components/introduction/styling.svelte";
    import HtmlTags from "./components/introduction/html_tags.svelte";

    /* Reactivity */
    import State from "./components/reactivity/state.svelte";
    import DeepState from "./components/reactivity/deep_state.svelte";
    import Effects from "./components/reactivity/effects.svelte";
    import UniversalReactivity from "./components/reactivity/universal_reactivity.svelte";

    /* Props */
    import DeclaringProps from "./components/props/declaring_props.svelte";
</script>

<div
    class="
        w-full pt-5
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
                flex flex-wrap gap-7
            "
        >
            <DynamicAttributes />
            <Styling />
            <HtmlTags />
        </div>
        <h1
            class="
                uppercase text-slate-300 text-3xl my-5
            "
        >
            reactivity    
        </h1>
        <div
            class="
                flex flex-wrap gap-7
            "
        >
            <State />
            <DeepState />
            <Effects />
            <UniversalReactivity />
        </div>
        <h1
            class="
                uppercase text-slate-300 text-3xl mt-7 mb-3
            "
        >
            props
        </h1>
        <div
            class="
                flex flex-wrap gap-7
            "
        >
            <DeclaringProps 
                answer={42}
            />
            <DeclaringProps />
        </div>
    </div>
</div>