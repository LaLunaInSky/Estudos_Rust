<script
    lang="ts"
>
    let { data } = $props();
</script>

<div
    class="
        flex flex-col gap-7 justify-center items-center
    "
>
    <h1
        class="
            text-neutral-200 text-center font-bold text-2xl uppercase mb-3 mx-2
        "
    >
        { data.post?.title }
    </h1>
    <div
        class="
            *:text-center *:text-neutral-400 *:text-xl w-130
        "
    >
        {@html data.post?.content}
    </div>
</div>