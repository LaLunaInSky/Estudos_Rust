<script
    lang="ts"
>
    let m = $state({ x: 0, y: 0 });

    function onpointermove(event: any) {
        m.x = event.clientX;
        m.y = event.clientY;

    }
</script>

<div 
    {onpointermove}
    class="
        h-25 pt-3 w-full flex justify-center items-center text-neutral-50
    "    
>
    The pointer is at {Math.round(m.x)} x {Math.round(m.y)}
</div>