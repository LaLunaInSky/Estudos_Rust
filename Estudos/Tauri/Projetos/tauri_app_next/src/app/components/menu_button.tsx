import MenuIcon from "../../../public/menu-icon.svg"
import Image from "next/image"

export function MenuButton() {
    return (
        <button
            className="
                w-10 h-10 rounded-full ml-2 cursor-pointer hover:bg-neutral-950/50 flex justify-center items-center
            "
            
        >
            <Image 
                src={MenuIcon}
                width={30}
                height={30}
                alt="Icone do Menu Lateral"
                className="
                    invert
                "
            />
        </button>
    )
}