<script
    lang="ts"
>
    let { data } = $props();
</script>

<div
    class="
        flex mb-3 flex-col gap-3
    "
>
    <h1
        class="
            text-2xl text-neutral-200 font-bold text-center
        "
    >
        Page Blog!
    </h1>
    <div>
        <ul>
            {#each data.summaries as { title, slug }}
            <li>
                <a 
                    href="/blog/{slug}"
                >
                    {title}
                </a>
            </li>
            {/each}
        </ul>
    </div>
</div>