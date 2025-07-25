import UserIcon from "@/../public/user-icon.svg"
import Image from "next/image"
import { invoke } from "@tauri-apps/api/core" 

export async function MenuPerfil() {
    let icone_do_perfil = UserIcon;

    await invoke('verificar_menu_do_perfil');

    return (
        <button
            className="
                bg-neutral-300/30 w-9 h-9 rounded-full ml-2 cursor-pointer hover:bg-neutral-950/30 flex justify-center items-center
            "
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