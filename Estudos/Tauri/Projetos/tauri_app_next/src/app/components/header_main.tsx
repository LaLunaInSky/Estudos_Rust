"use client"

import { useState } from "react";
import { BotãoPerfil } from "./header_main_components/botão_perfil";
import { JanelaDoPerfilDoUsuário } from "./header_main_components/janela_do_perfil_do_usuário";
import { MenuPerfilContext } from "./header_main_components/contexts/menu_perfil_context";

export function HeaderMain() {
        let [mostrar_menu_usuário, set_mostrar_menu_usuário] = useState(false);

    return (
        <MenuPerfilContext.Provider 
            value={
                set_mostrar_menu_usuário
            }
        >
            <div
                className="
                    fixed w-full z-10 -mt-12
                "
            >
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
                            ${!mostrar_menu_usuário ? "bg-neutral-50/0" : "bg-neutral-300/30"} pl-3 pr-2 py-1.5 -my-1 transition rounded-tl-2xl
                        `}
                    >
                        <BotãoPerfil 
                            mostrar_menu = {
                                mostrar_menu_usuário
                            }
                        />
                    </div>
                </div>
                <JanelaDoPerfilDoUsuário 
                    mostrar_menu = {
                        mostrar_menu_usuário
                    }
                />
            </div>
        </MenuPerfilContext.Provider>
    )
}