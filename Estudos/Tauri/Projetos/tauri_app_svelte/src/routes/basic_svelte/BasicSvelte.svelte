<script
    lang="ts"
>
    import Nested from "./components/Nested.svelte";
    import Counted from "./components/Counted.svelte";
    import Number from "./components/Number.svelte";
    import Interval from "./components/Interval.svelte";
    import CountLoop from "./components/CountLoop.svelte";
    import Separador from "./components/Separador.svelte";
    import Each from "./components/Each.svelte";
</script>

<div
    class="
        flex flex-col justify-center items-center bg-emerald-500/30 backdrop-blur-2xl w-90 p-3 mt-3 rounded-md border-2 border-slate-400/50
    "
>
    <h2
        class="
            text-2xl font-black text-neutral-50/90 uppercase
        "
    >
        Basic Svelte
    </h2>

    <Separador />
    <Nested />
    <Nested answer="teste a"/>
    
    <Separador />
    <Counted />

    <Separador />
    <Number />

    <Separador />
    <Interval />

    <Separador />
    <CountLoop />

    <Separador />
    <Each />
</div>