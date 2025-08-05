<script
    lang="ts"
>
    let { data } = $props();
</script>

<div
    class="
        flex justify-center items-center flex-col gap-5
    "
>
    <h2
        class="
            uppercase text-xl text-slate-400/70 font-bold
        "
    >
        blog
    </h2>
    <ul
        class="
            *:text-indigo-400 flex flex-col justify-center items-stretch gap-5 *:hover:underline *:hover:underline-offset-4 *:transition *:hover:transition list-disc
        "
    >
        {#each data.summaries as { slug, title }}
            <li>
                <a 
                    href={`/sveltekit/basic/blog/${slug}`}
                    target="_self"
                >
                    { title.toUpperCase() }
                </a>
            </li>
        {/each}
    </ul>
</div>