import { BotãoPerfil } from "./header_main_components/botão_perfil";
import { JanelaDoPerfilDoUsuário } from "./header_main_components/janela_do_perfil_do_usuário";

export function HeaderMain() {
    return (
        <div>
            <div
                className="
                bg-neutral-50/30 py-1 flex justify-start items-center
                "
            >
                <div
                    className="
                        flex justify-center items-center w-full
                    "
                >
                    <div
                        className="
                            bg-neutral-50/30 rounded-3xl w-1/3 h-8 cursor-pointer hover:bg-neutral-50/50 py-1 px-3
                        "
                    >
                        <input
                            type="search"
                            name="" id=""
                            className="
                                w-full h-full outline-0
                            "
                        />
                    </div>
                </div>
                <div
                    className={`
                        ${"bg-neutral-300/70"} px-3 py-1 -my-1
                    `}
                >
                    <BotãoPerfil />
                </div>
            </div>
            <JanelaDoPerfilDoUsuário />
        </div>
    )
}