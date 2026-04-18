<script lang="ts">
    import { goto } from "$app/navigation";
    import { vault } from "$lib/vault.svelte";

    let user = $state("");
    let password = $state("");
    let showPassword = $state(false);

    async function unlock() {
        const ok = await vault.unlock(user, password);
        if (ok) {
            password = "";
            goto("/vault");
        }
    }
</script>

<div class="min-h-screen flex items-center justify-center bg-base-200">
    <div class="card w-full max-w-sm bg-base-100 shadow-xl">
        <div class="card-body gap-5">
            <div class="flex flex-col items-center gap-2">
                <div class="text-5xl">🔐</div>
                <h1 class="card-title text-2xl">Password Vault</h1>
                <p class="text-base-content/50 text-sm text-center">
                    Enter your master password to unlock your vault
                </p>
            </div>

            {#if vault.error}
                <div role="alert" class="alert alert-error text-sm">
                    <span>{vault.error}</span>
                </div>
            {/if}

            <div class="flex flex-col items-center gap-2">
                <input
                    class="input"
                    type="text"
                    placeholder="User"
                    bind:value={user}
                />
                <input
                    class="input"
                    type={showPassword ? "text" : "password"}
                    placeholder="Master password"
                    bind:value={password}
                    onkeydown={(e) => e.key === "Enter" && unlock()}
                    autocomplete="current-password"
                />
                <button
                    class="btn btn-ghost btn-xs"
                    type="button"
                    onclick={() => (showPassword = !showPassword)}
                    aria-label="Toggle visibility"
                >
                    {showPassword ? "🙈" : "👁"}
                </button>
            </div>

            <button
                class="btn btn-primary w-full"
                onclick={unlock}
                disabled={vault.loading || !password}
            >
                {#if vault.loading}
                    <span class="loading loading-spinner loading-sm"></span>
                    Unlocking...
                {:else}
                    Unlock Vault
                {/if}
            </button>
        </div>
    </div>
</div>
