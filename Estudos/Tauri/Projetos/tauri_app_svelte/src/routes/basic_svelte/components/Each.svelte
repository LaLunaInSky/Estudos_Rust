<script
    lang="ts"
>
    const colors = ['red', 'orange', 'yellow', 'green', 'blue'];
    let selected = $state(colors[0]);
</script>

<div
    class="
        flex flex-col justify-center items-center mt-3
    "
>
    <h1
        style="color: {selected}"
        class="
            text-2xl font-bold
        "
    >
        Pick a colour
    </h1>
    <div
        class="
            flex gap-3 mt-3 *:cursor-pointer
        "
    >
        {#each colors as color, i}
        <button
            style="background: {color}"
            aria-label={color}
            aria-current={selected === color}
            onclick={() => selected = color}
            class="
                w-10 h-10 rounded-full text-neutral-700 font-black
            "
        >
            {i + 1}
        </button>
        {/each}
    </div>
</div>