import { MenuLateral } from "./components/menu_lateral";
import { MenuButton } from "./components/menu_button";

export default function Page() {
  
  return (
    <div>
      <header
        className="
          bg-neutral-50/30 py-1 flex justify-start items-start
        "
      >
        <MenuButton />
      </header>
      <main>

      </main>
    </div>
  );
}
