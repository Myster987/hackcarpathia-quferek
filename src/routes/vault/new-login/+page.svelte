<script lang="ts">
    import { goto } from "$app/navigation";
    import { insertLogin } from "$lib/state.svelte";
    import { ArrowLeft } from "@lucide/svelte";

    let name = $state("");
    let password = $state("");
    let status: "idle" | "loading" | "success" | "error" = $state("idle");

    async function handleSubmit() {
        if (!name.trim() || !password.trim()) return;
        status = "loading";
        try {
            await insertLogin(name.trim(), password);
            name = "";
            password = "";
            status = "success";
            setTimeout(() => (status = "idle"), 2000);
        } catch {
            status = "error";
            setTimeout(() => (status = "idle"), 3000);
        }
    }
</script>

<div
    class="min-h-screen bg-base-200 flex items-start justify-center p-8 flex-1"
>
    <div class="w-full max-w-lg">
        <div class="justify-between flex">
            <div class="mb-6">
                <h1 class="text-3xl font-bold text-base-content">
                    Dodaj login
                </h1>
                <p class="text-base-content/50 text-sm mt-1">
                    Zapisz nowy login do swojej listy
                </p>
            </div>

            <button class="btn btn-ghost" onclick={() => goto("/vault")}
                ><ArrowLeft /></button
            >
        </div>

        <div class="card bg-base-100 shadow-sm">
            <div class="card-body gap-4">
                <label class="form-control">
                    <div class="label">
                        <span
                            class="label-text text-base-content/60 text-xs uppercase tracking-wide"
                            >Nazwa</span
                        >
                    </div>
                    <input
                        type="text"
                        placeholder="np. jan.kowalski"
                        class="input input-bordered w-full"
                        bind:value={name}
                        disabled={status === "loading"}
                    />
                </label>

                <label class="form-control">
                    <div class="label">
                        <span
                            class="label-text text-base-content/60 text-xs uppercase tracking-wide"
                            >Hasło</span
                        >
                    </div>
                    <input
                        type="password"
                        placeholder="••••••••"
                        class="input input-bordered w-full"
                        bind:value={password}
                        onkeydown={(e) => e.key === "Enter" && handleSubmit()}
                        disabled={status === "loading"}
                    />
                </label>

                <button
                    class="btn btn-primary w-full"
                    onclick={handleSubmit}
                    disabled={!name.trim() ||
                        !password.trim() ||
                        status === "loading"}
                >
                    {#if status === "loading"}
                        <span class="loading loading-spinner loading-sm"></span>
                        Zapisywanie…
                    {:else}
                        Dodaj login
                    {/if}
                </button>

                {#if status === "success"}
                    <div class="alert alert-success py-2.5">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="size-4 shrink-0"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M5 13l4 4L19 7"
                            />
                        </svg>
                        <span class="text-sm">Login został dodany.</span>
                    </div>
                {/if}

                {#if status === "error"}
                    <div class="alert alert-error py-2.5">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="size-4 shrink-0"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M6 18L18 6M6 6l12 12"
                            />
                        </svg>
                        <span class="text-sm"
                            >Coś poszło nie tak. Spróbuj ponownie.</span
                        >
                    </div>
                {/if}
            </div>
        </div>
    </div>
</div>
