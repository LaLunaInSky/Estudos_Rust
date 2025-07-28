<script
    lang="ts"
>
    import { counter } from "./shared.svelte";
</script>

<button
    onclick={() => counter.count += 1}
    class="
        text-neutral-50 font-bold text-lg bg-indigo-700/30 border-3 border-indigo-600/30 px-2 rounded-2xl hover:bg-indigo-600/60 cursor-pointer
    "
>
    clicks: { counter.count }
</button>