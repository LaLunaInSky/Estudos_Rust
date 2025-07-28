<script
    lang="ts"
>
    let elapsed = $state(0);
    let interval = $state(1000);

    $effect(() => {
        const id = setInterval(() => {
            elapsed += 1;
        }, interval);
    
        return () => {
            clearInterval(id);
        };
    });
</script>

<div>
    <p
        class="
            text-neutral-300 flex justify-center items-center
        "
    >
        elapsed: { elapsed }
        <br>
        interval: { interval }
    </p>
    <div
        class="
            *:bg-indigo-700/50 *:px-2 *:py-1 *:border-3 *:border-indigo-600/40 *:text-neutral-50 *:rounded-lg flex gap-2 *:cursor-pointer *:hover:bg-indigo-600/60 mt-3
        "
    >
        <button
            onclick={() => { interval /= 2 }}
        >
            speed up
        </button>
        <button
            onclick={() => { interval *= 2 }}
        >
            slow down
        </button>
    </div>
</div>