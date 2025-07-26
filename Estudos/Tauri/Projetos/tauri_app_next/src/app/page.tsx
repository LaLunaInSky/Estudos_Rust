import { HeaderMain } from "./components/header_main";
import { JanelaDoPerfilDoUsuário } from "./components/janela_do_perfil_do_usuário";

export default function Page() {  
  return (
    <div>
      <HeaderMain />
      <main>
        <JanelaDoPerfilDoUsuário />
        <div
          className="
            px-3
          "
        >

        </div>
      </main>
    </div>
  );
}
