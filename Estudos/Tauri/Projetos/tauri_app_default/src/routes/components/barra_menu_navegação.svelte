<script
    lang="ts"
>
    const rotas_menus = [
        {
            link: "/",
            nome: "home",
        },
        {
            link: "/svelte/basic",
            nome: "basic svelte",
        },
        // {
        //     link: "/svelte/advanced",
        //     nome: "advanced svelte",
        // },
        {
            link: "/sveltekit/basic",
            nome: "basic sveltekit",
        },
        // {
        //     link: "/sveltekit/advanced",
        //     nome: "advanced sveltekit",
        // }
    ]
</script>

<div
    class="
        flex justify-center items-center h-14 fixed w-full -mt-12
    "
>
    <div
        class="
            bg-slate-600/60 w-10/11 rounded-b-xl h-14 flex gap-6 justify-center items-center uppercase text-neutral-200 text-md font-bold *:hover:bg-neutral-50/70 *:hover:text-neutral-800 fixed *:p-2 *:transition-colors *:hover:transition-colors *:rounded-lg *:hover:underline-2 *:hover:underline-offset-4 select-none
        "
    >
        {#each rotas_menus as rota}
            <a
                href={rota.link}
                target=""
            >
                {rota.nome}
            </a>
        {/each}
    </div>
</div>