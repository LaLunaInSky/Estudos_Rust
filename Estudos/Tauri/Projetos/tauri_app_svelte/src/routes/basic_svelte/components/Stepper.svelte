<script
    lang="ts"
>
    let { increment, decrement } = $props();
</script>

<div
    class="
        *:w-10 *:bg-slate-900 *:text-neutral-50 flex justify-center items-center gap-5 *:cursor-pointer *:hover:bg-slate-500 *:py-1 *:rounded-md mt-3
    "
>
    <button
        onclick={decrement}
    >
        -1
    </button>
    <button
        onclick={increment}
    >
        +1
    </button>
</div>