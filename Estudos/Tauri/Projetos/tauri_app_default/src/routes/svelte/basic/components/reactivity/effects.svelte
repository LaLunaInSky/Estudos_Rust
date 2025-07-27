<script
    lang="ts"
>
    let elapsed = $state(0);
    let interval = $state(1000);

    $effect(() => {
        setInterval(() => {
            elapsed += 1;
        }, interval);
    });
</script>

<div>
    <p
        class="
            text-neutral-300 flex justify-center items-center
        "
    >
        elapsed: { elapsed }
    </p>
    <div>
        <button>
            speed up
        </button>
        <button>
            slow down
        </button>
    </div>
</div>