import { Client, Stronghold } from "@tauri-apps/plugin-stronghold";
import { appDataDir } from "@tauri-apps/api/path";

let stronghold: Stronghold | null = null;
let client: Client | null = null;

const VAULT_NAME = "quferek-vault";
const INDEX_KEY = "__quferek-index__";

export async function init_stronghold(
  user_name: string,
  user_password: string,
): Promise<void> {
  const appDir = await appDataDir();
  const vaultPath = `${appDir}/\"${user_name}\"/vault.hold`;

  stronghold = await Stronghold.load(vaultPath, user_password);

  try {
    client = await stronghold.loadClient(user_name);
  } catch {
    client = await stronghold.createClient(user_name);
  }
}

async function get_index(): Promise<string[]> {
  if (!client || !stronghold) throw new Error("Stronghold not initialized");

  let store = client.getStore();

  try {
    const raw = await store.get(INDEX_KEY);
    if (!raw) {
      return [];
    }
    const json = new TextDecoder().decode(new Uint8Array(raw));
    return JSON.parse(json);
  } catch {
    return [];
  }
}

async function set_index(keys: string[]): Promise<void> {
  const store = client!.getStore();
  const encoded = Array.from(new TextEncoder().encode(JSON.stringify(keys)));
  try {
    await store.remove(INDEX_KEY);
  } catch {}
  await store.insert(INDEX_KEY, encoded);
}

export async function list_logins(): Promise<string[]> {
  return get_index();
}

export async function save_password(key: string, value: string): Promise<void> {
  if (!client || !stronghold) throw new Error("Stronghold not initialized");

  const store = client.getStore();
  const encoded = Array.from(new TextEncoder().encode(value));

  await store.insert(key, encoded);
  await stronghold.save();
}

export async function get_password(key: string): Promise<string | null> {
  if (!client) throw new Error("Stronghold not initialized");

  const store = client.getStore();

  try {
    const raw = await store.get(key);
    if (!raw) return null;
    return new TextDecoder().decode(new Uint8Array(raw));
  } catch {
    return null;
  }
}

export async function remove_password(key: string): Promise<void> {
  if (!client || !stronghold) throw new Error("Stronghold not initialized");

  const store = client.getStore();
  await store.remove(key);
  await stronghold.save();
}

export function is_initialized(): boolean {
  return stronghold !== null && client !== null;
}
