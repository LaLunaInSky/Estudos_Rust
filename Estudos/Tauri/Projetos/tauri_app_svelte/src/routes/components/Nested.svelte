<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";

    let mensagem = "";

    async function get_mensagem() {
        mensagem = await invoke("my_custom_command_2", { your_name: "Luna" });
    }

    get_mensagem();

</script>

<p
    class="
        mt-5 text-slate-950 font-black
    "
>
    {mensagem}
</p>