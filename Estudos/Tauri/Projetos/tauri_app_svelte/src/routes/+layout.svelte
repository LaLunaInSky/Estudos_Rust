<script lang="ts">
    import "../app.css"

    let { children } = $props();

    let name = 'Svelte';
    let src = "/svelte.svg";

    let páginas = [
        {name: "home", link: "/"},
        {name: "about", link: "/about"},
        {name: "blog", link: "/blog"}
    ]
</script>

<header
    class="
        h-svh bg-slate-50/30 w-60 pt-2 fixed
    "
>
    <div
        class="
            flex mb-3 justify-center items-center cursor-default
        "
    >
        <h1
            class="
                text-xl text-neutral-200 font-bold
            "
        >
            Hello {name}!
        </h1>
        <img
            {src}
            alt="image {name} icon"
            class="
                ml-5
            "
        >
    </div>
    <nav
        class="
            flex flex-col gap-2 ml-2 uppercase *:font-bold *:text-neutral-200 *:hover:text-indigo-900
        "
    >
        {#each páginas as página }
            <a href={página.link}>
                {página.name}
            </a>
        {/each}
    </nav>
</header>
<main
    class="
        w-full py-10 flex flex-col justify-center items-center  *:select-none ml-60
    "
>
    {@render children()}
</main>

<style 
    lang="postcss"
>
    @reference "tailwindcss";
</style>