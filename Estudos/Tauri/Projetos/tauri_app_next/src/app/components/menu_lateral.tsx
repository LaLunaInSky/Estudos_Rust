import Link from "next/link"

export function MenuLateral() {
    return  (
        <div
            className="
                bg-neutral-50/30 fixed w-40 h-svh
            "
        >
            
            <menu 
                type="toolbar"
            >
                <ul>
                    <li>
                        <Link
                            href="/minha-pagina"
                        >
                            Minha Página
                        </Link>
                    </li>
                </ul>
            </menu>
        </div>
    )
}