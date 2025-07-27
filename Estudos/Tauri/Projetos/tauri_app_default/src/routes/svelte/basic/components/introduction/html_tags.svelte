<script
    lang="ts"
>
    const string = `This string contains some <strong>HTML!!!</strong>`;
</script>

<div
    class="
        flex justify-center items-center
    "
>
    <p
        class="
            text-neutral-50
        "
    >
      { @html string }  
    </p>
</div>