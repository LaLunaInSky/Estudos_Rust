<script
    lang="ts"
>
    let { name, version, description, website } = $props();
</script>

<div
    class="
        *:text-neutral-50 w-100 
    "
>
    <p>
        The <code>{name}</code> package is {description}. Download version {version} from 
        <a 
            href="https://www.npmjs.com/package/{name}"
            target="_blank"
            class="
                underline underline-offset-4 hover:bg-neutral-50/60 hover:text-slate-800 px-1 rounded-sm hover:no-underline transition-colors hover:transition-colors
            "
        >
            npm
        </a>
        and
        <a 
            href={website}
            target="_blank"
            class="
                underline underline-offset-4 hover:bg-neutral-50/60 hover:text-slate-800 px-1 rounded-sm hover:no-underline transition-colors hover:transition-colors
            "
        >
            learn more here
        </a>
    </p>
</div>