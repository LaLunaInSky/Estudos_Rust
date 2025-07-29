<script
    lang="ts"
>
    let { data } = $props();
</script>

<div
    class="
        flex flex-col gap-7 w-100 justify-center items-center
    "
>
    <h1
        class="
            text-neutral-200 text-center font-bold text-2xl
        "
    >
        { data.post?.title }
    </h1>
    <div
        class="
            *:text-center *:text-neutral-400 *:text-lg
        "
    >
        {@html data.post?.content}
    </div>
</div>