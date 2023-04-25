import { writable } from "svelte/store";
import type { ApiModuleType } from "./ApiTypes";

export const userModules = writable<ApiModuleType[]>([]);
