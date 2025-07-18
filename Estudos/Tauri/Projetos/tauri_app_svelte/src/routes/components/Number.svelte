<script lang="ts">
    let numbers = $state([1, 2, 3, 4]);

    function add_number() {
        numbers[numbers.length] = numbers.length + 1;
    }

    function add_number_2() {
        numbers.push(numbers.length + 2);
    }

    add_number();
    add_number_2();
</script>

<p>
    {numbers}
</p>