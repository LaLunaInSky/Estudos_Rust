<script
    lang="ts"
>
    let numbers = $state([1, 2, 3, 4]);

    /* derived state */
    let total = $derived(
        numbers.reduce((t, n) => t + n, 0)
    );

    function addNumber() {
        numbers.push(numbers.length + 1);
    }
</script>

<div
    class="
        text-neutral-300 flex flex-col gap-2 w-40
    "
>
    <div
        class="
            flex justify-center items-center
        "
    >
        <button
            class="
                bg-indigo-600/30 px-3 py-1 font-bold cursor-pointer hover:bg-indigo-400/30 rounded-lg border-2 border-indigo-300/20 hover:text-indigo-200/40 
            "
            onclick={addNumber}
        >
            Add a number
        </button>
    </div>
    <p
        class="
            text-center
        "
    >
        {numbers.join(" + ")} = { total }
    </p>
</div>