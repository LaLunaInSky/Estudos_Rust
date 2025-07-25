import { HeaderMain } from "./components/header_main";
import { JanelaDoPerfilDoUsuário } from "./components/janela_do_perfil_do_usuário";

export default function Page() {  
  return (
    <div>
      <HeaderMain />
      <main>
        <JanelaDoPerfilDoUsuário />
        <h1>
            Lorem ipsum dolor sit amet consectetur adipisicing elit. Consequatur dolores pariatur iste rem sit minima laudantium eligendi enim deserunt quas ut delectus, officia, est odio incidunt optio? Modi, repellat aliquid!
        </h1>
      </main>
    </div>
  );
}
