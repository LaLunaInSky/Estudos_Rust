import { createContext, Dispatch, SetStateAction } from "react";

export const MenuPerfilContext = createContext<Dispatch<SetStateAction<boolean>>>(false);