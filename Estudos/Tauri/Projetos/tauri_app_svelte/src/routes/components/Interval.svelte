<script 
    lang="ts"
>
    let elapsed = $state(0);
    let interval = $state(1000);

    function increment(event: Event) {
        event.preventDefault();

        interval += 100;
    }

    function decrement(event: Event) {
        event.preventDefault();

        if (interval > 100) {
            interval -= 100;
        }
    }

    $effect(() => {
        const id = setInterval(() => {
            elapsed += 1;
        }, interval);

        return () => {
            clearInterval(id);
        };
    })
</script>

<div
    class="
        text-neutral-200 mt-3
    "
>
    <p>
        <strong>
            Elapsed:
        </strong>
        {elapsed}
    </p>
    <p>
        <strong>
            Interval:
        </strong>
        {interval}
    </p>
    <div
        class="
            *:cursor-pointer flex justify-center items-center gap-3 text-2xl *:bg-slate-500 *:w-10 *:rounded-md *:hover:bg-sky-600/70 mt-3
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