<script
    lang="ts"
>
    const opções_de_navegações = [
        {
            link: "/sveltekit/basic/header",
            nome: "header"
        },
        {
            link: "/sveltekit/basic",
            nome: "home"
        }
    ]
</script>

<div
    class="
        flex gap-5 bg-slate-800/60 p-4 rounded-lg *:select-none
    "
>
    {#each opções_de_navegações.reverse() as opção }    
        <h2
            class="
                text-neutral-300 *:hover:bg-slate-300 *:hover:text-slate-800 *:p-3 *:rounded-lg *:font-bold
            "
        >
            <a 
                href={opção.link}
            >
                {opção.nome.toUpperCase()}
            </a>
        </h2>
    {/each}
</div>