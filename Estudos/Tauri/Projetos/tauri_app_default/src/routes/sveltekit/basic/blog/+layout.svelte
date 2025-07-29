<script
    lang="ts"
>
    let { children } = $props();

    import BarraDeNavegação from "../components/routing/barra_de_navegação.svelte";
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
        <BarraDeNavegação />
        <div>
            {@render children()}
        </div>
    </div>
</div>