<script lang="ts">
    let numbers = $state([1, 2, 3, 4]);
    
    function add_number() {
        numbers[numbers.length] = numbers.length + 1;
    }
    
    function add_number_2() {
        numbers.push(numbers.length + 2);
        console.log($state.snapshot(numbers));
    }
    
    add_number();
    add_number_2();

    let total = $derived(numbers.reduce((t, n) => t + n, 0));

    $inspect(numbers).with(console.trace);
</script>

<div
    class="
        text-neutral-200 *:mt-2 flex flex-col justify-center items-center
    "
>
    <p>
        {numbers.join(", ")}
    </p>
    <p>
        {numbers.join(" + ")} = {total}
    </p>
</div>