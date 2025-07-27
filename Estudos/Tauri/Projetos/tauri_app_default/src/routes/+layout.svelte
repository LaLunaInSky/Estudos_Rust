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
            bg-red-500 w-full h-full mt-12
        "
    >
        {@render children()}
    </div>
</div>

