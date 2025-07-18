<script lang="ts">
    let count = $state(0);

    function increment(event: Event) {
        event.preventDefault();

        count += 1;
    }

    function decrement(event: Event) {
        event.preventDefault();

        if (count > 0) {
            count -= 1;
        }
    }

</script>

<div
    class="
        flex justify-center items-center flex-col border-2 py-3 px-4 rounded-lg border-slate-500/50 bg-slate-700/70 backdrop-blur-2xl mt-2
    "
>
    <p
        class="
            text-xl text-slate-50
        "
    >
        The count is: {count}
    </p>
    <div
        class="
            *:cursor-pointer *:bg-slate-400 *:px-2 *:text-xl *:font-bold *:hover:bg-slate-900 *:hover:text-slate-100 *:rounded-md mt-2
        "
    >
        <button 
            type="button"
            onclick={decrement}
        >
            -
        </button>
        <button
            type="button"
            onclick={increment}
        >
            +
        </button>
    </div>
</div>