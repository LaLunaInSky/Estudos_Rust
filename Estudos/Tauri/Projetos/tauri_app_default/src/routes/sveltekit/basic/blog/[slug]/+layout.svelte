<script
    lang="ts"
>
    let { data, children } = $props();
</script>

<div
    class="
        flex gap-4 justify-center w-full items-start
    "
>
    <div
        class="
            w-170 bg-slate-400/15 rounded-2xl flex justify-center items-center py-6 px-10
        "
    >
        {@render children()}
    </div>
    <aside
        class="
            text-neutral-50 w-50 ml-5
        "
    >
        <h2
            class="
                font-black text-center text-xl mb-2
            "
        >
            More Posts
        </h2>
        <ul
            class="
                list-disc *:mb-2 font-light text-md
            "
        >
            {#each data.summaries as { slug, title }}
                <li
                    class="
                        *:hover:underline *:hover:underline-offset-4 *:hover:text-indigo-300/70
                    "
                >
                    <a 
                        href={`/sveltekit/basic/blog/${slug}`}
                    >
                        {title}
                    </a>
                </li>
            {/each}
        </ul>
    </aside>
</div>