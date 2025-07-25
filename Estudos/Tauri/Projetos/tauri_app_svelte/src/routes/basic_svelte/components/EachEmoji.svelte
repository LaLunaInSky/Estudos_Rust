<script
    lang="ts"
>
    let things = $state([
        { id: 1, name: "apple", emoji: "🍎"},
        { id: 2, name: "banana", emoji: "🍌"},
        { id: 3, name: "carrot", emoji: "🥕"},
        { id: 4, name: "doughnut", emoji: "🍩"},
        { id: 5, name: "egg", emoji: "🥚"}
    ])
</script>

<div
    class="
        mt-3
    "
>
    <button
        class="
            bg-slate-900/60 px-2 py-1 text-neutral-50 rounded-sm hover:bg-slate-600 cursor-pointer
        "
        onclick={() => things.shift()}
    >
        Remove first thing
    </button>
    <div
        class="
            text-center mt-2 text-neutral-50 font-bold
        "
    >
        {#each things as thing}
            <p>
              {thing.emoji} = {thing.name}
            </p>
        {/each}
    </div>
</div>