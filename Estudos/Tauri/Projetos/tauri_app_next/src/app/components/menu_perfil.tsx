'use client';

import UserIcon from "@/../public/user-icon.svg"
import Image from "next/image"
import { invoke } from "@tauri-apps/api/core" 
import { useEffect, useState } from "react";

export function MenuPerfil() {
    let icone_do_perfil = UserIcon;
    // let [menu_mostrar, set_menu_mostrar] = useState(false);

    // const setar_menu_mostrar = () => {
    //     if (menu_mostrar) {
    //         set_menu_mostrar(false);
    //     } else {
    //         set_menu_mostrar(true);
    //     }
    // }

    // useEffect(()=>{
    //     console.log(menu_mostrar)
    // })

    return (
        <button
            className="
                bg-neutral-300/30 w-9 h-9 rounded-full ml-2 cursor-pointer hover:bg-neutral-950/30 flex justify-center items-center
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