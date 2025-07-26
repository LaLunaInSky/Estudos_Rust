'use client'

import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export function JanelaDoPerfilDoUsuário() {
    let [perfil_mostrar, set_perfil_mostrar] = useState(false);

    
    return (
        <div
            className={`${!perfil_mostrar ? "hidden" : "fixed"}
                bg-neutral-300/95 w-100 h-svh
            `}
        >

        </div>
    )
}