import { goto } from "$app/navigation";
import { invoke } from "@tauri-apps/api/core";

interface Login {
  name: string;
  password: string;
}

export async function getAllLogins(): Promise<string[]> {
  return invoke<string[]>("get_all_logins");
}

export async function getLogin(name: string): Promise<Login> {
  return invoke<Login>("get_login", { name });
}

export async function insertLogin(
  name: string,
  password: string,
): Promise<void> {
  return invoke<void>("insert_new_login", { name, password });
}

export async function deleteLogin(name: string): Promise<void> {
  return invoke<void>("delete_login", { name });
}

const sign_up = (username: string, password: string): Promise<void> =>
  invoke("sign_up", { username, password });

const sign_in = (username: string, password: string): Promise<number[]> =>
  invoke("login", { username, password });

const sign_out = (): Promise<void> => invoke("sign_out");

interface User {
  username: string;
  loggedInAt: Date;
}

function createUser() {
  let current = $state<User | null>(null);

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

    async signUp(username: string, password: string): Promise<void> {
      await sign_up;

      current = { username, loggedInAt: new Date() };
      goto("/vault");
    },

    async signIn(username: string, password: string): Promise<void> {
      const keyBytes = await sign_in(username, password);

      current = { username, loggedInAt: new Date() };
      goto("/vault");
    },

    async signOut(): Promise<void> {
      await sign_out();
      current = null;
      goto("/");
    },
  };
}

export const user = createUser();
