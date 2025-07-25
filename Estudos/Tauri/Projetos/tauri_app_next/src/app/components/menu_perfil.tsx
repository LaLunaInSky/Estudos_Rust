import UserIcon from "@/../public/user-icon.svg"
import Image from "next/image"

export function MenuPerfil() {
    let icone_do_perfil = UserIcon;

    return (
        <button
            className="
                bg-neutral-300/30 w-10 h-10 rounded-full ml-2 cursor-pointer hover:bg-neutral-950/30 flex justify-center items-center
            "
        >
            <Image 
                src={icone_do_perfil}
                width={30}
                height={30}
                alt="Icone Menu Perfil do Usuário"

            />
        </button>
    )
}