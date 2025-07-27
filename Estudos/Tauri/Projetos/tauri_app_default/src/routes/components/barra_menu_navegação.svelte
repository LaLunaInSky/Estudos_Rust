<script
    lang="ts"
>
    const rotas_menus = [
        {
            link: "/svelte/basic",
            nome: "basic svelte",
        },
        {
            link: "/svelte/advanced",
            nome: "advanced svelte",
        },
        {
            link: "/sveltekit/basic",
            nome: "basic sveltekit",
        },
        {
            link: "/sveltekit/advanced",
            nome: "advanced sveltekit",
        }
    ]
</script>

<div
    class="
        bg-slate-500/50 w-full h-12 flex gap-6 justify-center items-center uppercase text-neutral-200 text-sm font-bold *:hover:text-indigo-500/40 fixed -mt-12
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