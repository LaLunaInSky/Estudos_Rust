export default function Page() {
  let total_de_contatos = 10;
  
  return (
    <main
      className="
        flex justify-end
      "
    >
      <div
        className="
          bg-neutral-50/30 backdrop-blur-2xl w-1/3 h-svh
        "
      >
        <div
          className="
            bg-neutral-50/20 w-full h-1/10 flex justify-center items-center
          "
        >
          for (let count = 1; count < {total_de_contatos}; count++) {
            <div
              className="
                bg-neutral-700/30 backdrop-blur-lg w-15 h-15 rounded-full cursor-pointer hover:bg-neutral-700/80
              "
            >
            
            </div>
          }        
        </div>
      </div>
    </main>
  );
}
