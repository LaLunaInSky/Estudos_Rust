<script
    lang="ts"
>
    let count = 0;

    function increment(event: Event) {
        event.preventDefault();

        count += 1;
    }
</script>

<div
    class="
        bg-neutral-400/50 backdrop-blur-2xl p-4 mt-3 rounded-md border-2 border-neutral-300/30 flex flex-col justify-center items-center text-neutral-950 font-bold
    "
>
    <button 
        type="button"
        onclick={increment}
        class="
            cursor-pointer hover:text-neutral-700
        "
    >
        Clicked {count}
        {count <= 1 ? "time" : "times"}
    </button>

    {#if count > 10}
        <p>
            {count} is greater than 10
        </p>
    {:else if count < 5}
        <p>
            {count} is less than 5
        </p>
    {:else}
        <p>
            {count} is between 5 and 10
        </p>
    {/if}
</div>