import UserIcon from "@/../public/user-icon.svg"
import Image from "next/image"
import { useContext } from "react";
import { MenuPerfilContext } from "./contexts/menu_perfil_context";

export function BotãoPerfil(
{mostrar_menu} : {mostrar_menu: boolean}
) {
    let icone_do_perfil = UserIcon;
    let set_mostrar_menu_usuário = useContext(MenuPerfilContext);

    function verificar_mostrar_menu_usuário() {
        if (!mostrar_menu) {
            set_mostrar_menu_usuário(true);
        } else {
            set_mostrar_menu_usuário(false);
        }
    }

    return (
        <button
            className={`
                ${mostrar_menu ? "bg-neutral-900/30 hover:bg-indigo-400/60": "bg-neutral-300/30 hover:bg-indigo-300/40"} transition w-9 h-9 rounded-full cursor-pointer  flex justify-center items-center
            `}
            onClick={verificar_mostrar_menu_usuário}
        >
            <Image 
                src={icone_do_perfil}
                width={25}
                height={25}
                alt="Icone Menu Perfil do Usuário"
            />
        </button>
    )
}