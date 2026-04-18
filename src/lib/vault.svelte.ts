import { goto } from "$app/navigation";
import { invoke } from "@tauri-apps/api/core";

export type VaultError = string;

export async function register(
  username: string,
  password: string,
): Promise<void> {
  await invoke("register", { username, password });
}

export async function login(username: string, password: string): Promise<void> {
  await invoke("login", { username, password });
}

export async function logout(): Promise<void> {
  await invoke("logout");
}

export async function saveEntry(
  entryName: string,
  secret: string,
): Promise<void> {
  await invoke("save_entry", { entryName, secret });
}

interface Session {
  username: string;
  loggedInAt: Date;
}

function createSession() {
  let current = $state<Session | null>(null);

  return {
    get current() {
      return current;
    },
    get isAuthenticated() {
      return current !== null;
    },
    get username() {
      return current?.username ?? "";
    },

    async register(username: string, password: string) {
      await register(username, password);
      current = { username, loggedInAt: new Date() };
      goto("/vault");
    },

    async login(username: string, password: string) {
      await login(username, password);
      current = { username, loggedInAt: new Date() };
      goto("/vault");
    },

    async logout() {
      await logout();
      current = null;
      goto("/auth");
    },
  };
}

export const session = createSession();
