import Link from "next/link"

export function JanelaDoPerfilDoUsuário(
{mostrar_menu} : {mostrar_menu: boolean}
) {
    let usuário = "LaLunaInSky"

    return (
        <div
            className={`
                ${!mostrar_menu ? "hidden" : "fixed"} bg-neutral-300/60 w-50 h-[90svh] rounded-b-lg  absolute right-0 transition flex flex-col items-center pt-3 overflow-hidden justify-between 
            `}
        >
            <div
                className="
                    flex flex-col justify-center items-center w-full
                "
            >
                <h1
                    className="
                        text-indigo-950 font-black text-lg
                    "
                >
                    @{usuário}
                </h1>
                <Link
                    href={`/${usuário}`}
                    className="
                        cursor-pointer text-sm font-extralight text-neutral-100/70 hover:text-neutral-950
                    "
                >
                    ver seu perfil
                </Link>
                <hr 
                    className="
                        border-2 w-7/8  rounded-full mt-3 border-neutral-200/60
                    "
                />
            </div>
            <button
                className="
                    cursor-pointer hover:text-indigo-950 font-bold bg-neutral-700 w-full py-3 hover:bg-indigo-400/80
                "
            >
                Sair da Conta
            </button>
        </div>
    )
}