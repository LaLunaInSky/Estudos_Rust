<script
    lang="ts"
>
    let count = $state(0);

    function increment() {
        count += 1;
    }
</script>

<div
    class="
        text-neutral-300 flex justify-center items-center
    "
>
    <div
        class="
            bg-indigo-600/20 px-2 py-3 flex justify-center border-4 border-indigo-300/20 rounded-xl items-center gap-3
        "
    >
        <button
            onclick={increment}
            class="
                font-bold text-lg cursor-pointer hover:text-neutral-100/40
            "
        >
            Clicked
        </button>
        <p
            class="
                text-lg px-2
            "
        >
            { count }
        </p>
    </div>
</div>