<script
    lang="ts"
>
    let { children } = $props();

    import "../app.css";

    import BarraMenuNavegação from "./components/barra_menu_navegação.svelte";
</script>

<div>
    <BarraMenuNavegação />
    <div
        class="
            bg-slate-950 w-full px-2 mt-12
        "
    >
        {@render children()}
    </div>
</div>

