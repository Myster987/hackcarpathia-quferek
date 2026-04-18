import {
  init_stronghold,
  list_logins,
  save_password,
  get_password,
  remove_password,
  is_initialized,
} from "./stronghold";

// Svelte 5 runes work in .svelte.ts files
let _initialized = $state(false);
let _keys = $state<string[]>([]);
let _loading = $state(false);
let _error = $state("");
let _success = $state("");

function setStatus(type: "error" | "success", msg: string) {
  _error = type === "error" ? msg : "";
  _success = type === "success" ? msg : "";
  setTimeout(() => {
    _error = "";
    _success = "";
  }, 3000);
}

export const vault = {
  get initialized() {
    return _initialized;
  },
  get keys() {
    return _keys;
  },
  get loading() {
    return _loading;
  },
  get error() {
    return _error;
  },
  get success() {
    return _success;
  },

  async unlock(user: string, password: string): Promise<boolean> {
    _loading = true;
    _error = "";
    try {
      await init_stronghold(user, password);
      _initialized = true;
      await vault.refresh();
      return true;
    } catch (e) {
      setStatus("error", `Wrong password or corrupt vault: ${e}`);
      return false;
    } finally {
      _loading = false;
    }
  },

  async refresh() {
    _keys = await list_logins();
  },

  async save(key: string, value: string): Promise<boolean> {
    _loading = true;
    try {
      await save_password(key, value);
      await vault.refresh();
      setStatus("success", `Saved "${key}"`);
      return true;
    } catch (e) {
      setStatus("error", `${e}`);
      return false;
    } finally {
      _loading = false;
    }
  },

  async reveal(key: string): Promise<string> {
    const val = await get_password(key);
    return val ?? "";
  },

  async remove(key: string): Promise<void> {
    _loading = true;
    try {
      await remove_password(key);
      await vault.refresh();
      setStatus("success", `Deleted "${key}"`);
    } catch (e) {
      setStatus("error", `${e}`);
    } finally {
      _loading = false;
    }
  },
};
