export function JanelaDoPerfilDoUsuário(
{mostrar_menu} : {mostrar_menu: boolean}
) {
    return (
        <div
            className={`
                ${!mostrar_menu ? "hidden" : "fixed"} bg-neutral-300/60 w-50 h-svh  absolute right-0 transition
            `}
        >

        </div>
    )
}