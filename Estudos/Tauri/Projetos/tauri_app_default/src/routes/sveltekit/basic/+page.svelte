<script
    lang="ts"
>
    /* Routing */
    import BarraDeNavegação from "./components/routing/barra_de_navegação.svelte";

    /* Headers and Cookies */
    import MenuNavegação from "./components/headers_and_cookies/menu_navegação.svelte";
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
                uppercase text-slate-300 text-3xl mb-3
            "
        >
            routing
        </h1>
        <BarraDeNavegação />
        <h2
            class="
                uppercase text-xl text-slate-400/70 font-bold
            "
        >
            home
        </h2>
        <!-- <div
            class="
                flex flex-wrap gap-7
            "
        >
            
        </div> -->
        <h1
            class="
                uppercase text-slate-300 text-3xl my-3
            "
        >
            headers and cookies
        </h1>
        <div
            class="
                flex flex-wrap gap-7
            "
        >
            <MenuNavegação />
        </div>
    </div>
</div>