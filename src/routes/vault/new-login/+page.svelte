<script lang="ts">
    import { goto } from "$app/navigation";
    import { generatePassword } from "$lib/passwordGenerator";
    import { insertLogin } from "$lib/state.svelte";
    import { ArrowLeft, Dices } from "@lucide/svelte";

    let name = $state("");
    let password = $state("");
    let showPassword = $state(false);
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

    let copied = $state(false);

    async function copyPassword(password: string) {
        await navigator.clipboard.writeText(password);
        copied = true;
        setTimeout(() => (copied = false), 2000);
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
                    <div class="flex gap-2">
                        <input
                            type={showPassword ? "text" : "password"}
                            placeholder="Hasło..."
                            class="input input-bordered w-full"
                            bind:value={password}
                            onkeydown={(e) =>
                                e.key === "Enter" && handleSubmit()}
                            disabled={status === "loading"}
                        />
                        <button
                            class="btn btn-bordered join-item border border-base-300"
                            onclick={() => {
                                showPassword = true;
                                password = generatePassword(16);
                            }}><Dices /></button
                        >
                        <button
                            class="btn btn-bordered join-item border border-base-300"
                            onclick={() => (showPassword = !showPassword)}
                            type="button"
                        >
                            {#if showPassword}
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    class="size-4"
                                    fill="none"
                                    viewBox="0 0 24 24"
                                    stroke="currentColor"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.88 9.88l-3.29-3.29m7.532 7.532l3.29 3.29M3 3l3.59 3.59m0 0A9.953 9.953 0 0112 5c4.478 0 8.268 2.943 9.543 7a10.025 10.025 0 01-4.132 5.411m0 0L21 21"
                                    />
                                </svg>
                            {:else}
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    class="size-4"
                                    fill="none"
                                    viewBox="0 0 24 24"
                                    stroke="currentColor"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
                                    />
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"
                                    />
                                </svg>
                            {/if}
                        </button>

                        <button
                            class="btn btn-bordered join-item border border-base-300"
                            onclick={() => copyPassword(password ?? "")}
                            type="button"
                        >
                            {#if copied}
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    class="size-4 text-success"
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
                            {:else}
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    class="size-4"
                                    fill="none"
                                    viewBox="0 0 24 24"
                                    stroke="currentColor"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"
                                    />
                                </svg>
                            {/if}
                        </button>
                    </div>
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
