<script
    lang="ts"
>
    let { data } = $props();
</script>

<div
    class="
        flex flex-col gap-3 px-5 justify-center items-center
    "
>
    <h1
        class="
            font-bold text-neutral-50
        "
    >
        {data.post.title}
    </h1>
    <div
        class="
            w-140 text-center
        "
    >
        {@html data.post.content}
    </div>
</div>