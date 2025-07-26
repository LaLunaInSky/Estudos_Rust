'use client'

import { useEffect, useState } from "react";

export function JanelaDoPerfilDoUsuário() {
    let [perfil_mostrar, set_perfil_mostrar] = useState(true);

    
    return (
        <div
            className={`${!perfil_mostrar ? "hidden" : "fixed"}
                bg-neutral-300/80 w-50 h-svh  absolute right-0
            `}
        >

        </div>
    )
}