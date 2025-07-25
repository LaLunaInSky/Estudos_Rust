import { MenuPerfil } from "./menu_perfil";

export function HeaderMain() {
    return (
        <header
            className="
            bg-neutral-50/30 py-1 flex justify-start items-center
            "
        >
            <MenuPerfil />
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
        </header>
    )
}