<script lang="ts">
    import { goto } from "$app/navigation";
    import { vault } from "$lib/vault.svelte";

    let search = $state("");

    const filtered = $derived(
        vault.keys.filter((k) =>
            k.toLowerCase().includes(search.toLowerCase()),
        ),
    );

    const grouped = $derived(() => {
        const map: Record<string, string[]> = {};
        for (const key of filtered) {
            const letter = key[0].toUpperCase();
            (map[letter] ??= []).push(key);
        }
        return Object.entries(map).sort(([a], [b]) => a.localeCompare(b));
    });
</script>

<div class="max-w-2xl mx-auto flex flex-col gap-5">
    <!-- Header -->
    <div class="flex items-center justify-between">
        <h1 class="text-2xl font-bold">All Passwords</h1>
        <a href="/vault/new" class="btn btn-primary btn-sm gap-2">
            <span>➕</span> Add New
        </a>
    </div>

    <!-- Search -->
    <label class="input w-full flex items-center gap-2">
        <svg
            class="h-4 w-4 opacity-50"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
        >
            <circle cx="11" cy="11" r="8" /><path d="m21 21-4.35-4.35" />
        </svg>
        <input
            class="grow"
            type="search"
            placeholder="Search passwords…"
            bind:value={search}
        />
        {#if search}
            <button class="btn btn-ghost btn-xs" onclick={() => (search = "")}
                >✕</button
            >
        {/if}
    </label>

    <!-- Empty state -->
    {#if vault.keys.length === 0}
        <div class="card bg-base-100 shadow">
            <div class="card-body items-center gap-3 py-16">
                <span class="text-5xl">🗝️</span>
                <p class="text-base-content/60">No passwords saved yet.</p>
                <a href="/vault/new" class="btn btn-primary btn-sm"
                    >Add your first password</a
                >
            </div>
        </div>
    {:else if filtered.length === 0}
        <div class="card bg-base-100 shadow">
            <div class="card-body items-center py-12">
                <p class="text-base-content/60">
                    No results for "<strong>{search}</strong>"
                </p>
            </div>
        </div>
    {:else}
        {#each grouped() as [letter, keys]}
            <div>
                <div
                    class="text-xs font-bold text-base-content/40 uppercase tracking-widest mb-2 px-1"
                >
                    {letter}
                </div>
                <div class="card bg-base-100 shadow divide-y divide-base-200">
                    {#each keys as key (key)}
                        <button
                            class="flex items-center gap-4 px-5 py-4 hover:bg-base-200 transition-colors w-full text-left"
                            onclick={() =>
                                goto(`/vault/${encodeURIComponent(key)}`)}
                        >
                            <div class="avatar placeholder">
                                <div
                                    class="bg-primary/15 text-primary rounded-full w-10 h-10 text-sm font-bold"
                                >
                                    <span>{key[0].toUpperCase()}</span>
                                </div>
                            </div>
                            <div class="flex-1 min-w-0">
                                <p class="font-medium truncate">{key}</p>
                                <p class="text-xs text-base-content/40">
                                    ••••••••••
                                </p>
                            </div>
                            <svg
                                class="h-4 w-4 opacity-30"
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                            >
                                <path d="m9 18 6-6-6-6" />
                            </svg>
                        </button>
                    {/each}
                </div>
            </div>
        {/each}

        <p class="text-center text-xs text-base-content/30 pb-2">
            {filtered.length} of {vault.keys.length} entries
        </p>
    {/if}
</div>
