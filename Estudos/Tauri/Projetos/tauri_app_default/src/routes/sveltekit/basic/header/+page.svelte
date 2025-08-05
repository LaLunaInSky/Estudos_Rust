<script
    lang="ts"
>
    import MenuNavegação from "../components/headers_and_cookies/menu_navegação.svelte";

    let { data } = $props();
</script>

<div>
    <MenuNavegação />
    <div
        class="
            *:text-neutral-50 *:select-none flex justify-center mt-10 *:text-2xl
        "
    >
        <h1>
            Hello {data.visited ? "Friend" : "Stranger"}!
        </h1>
    </div>
</div>