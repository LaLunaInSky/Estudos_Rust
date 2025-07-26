'use client';

import UserIcon from "@/../public/user-icon.svg"
import Image from "next/image"
import { invoke } from "@tauri-apps/api/core"

export function BotãoPerfil() {
    let icone_do_perfil = UserIcon;

    

    return (
        <button
            className="
                bg-neutral-300/30 w-9 h-9 rounded-full cursor-pointer hover:bg-neutral-950/30 flex justify-center items-center
            "
            // onClick={setar_menu_mostrar}
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