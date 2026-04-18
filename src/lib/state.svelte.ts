import { goto } from "$app/navigation";
import { invoke } from "@tauri-apps/api/core";

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
