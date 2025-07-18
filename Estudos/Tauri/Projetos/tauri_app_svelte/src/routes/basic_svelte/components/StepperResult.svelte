<script
    lang="ts"
>
    import Stepper from "./Stepper.svelte";

    let value = $state(0);
</script>

<div
    class="
        mt-3
    "
>
    <p>
        The current value is {value}
    </p>
    <Stepper 
        increment = {() => value += 1}
        decrement = {() => value -= 1}
    />
</div>