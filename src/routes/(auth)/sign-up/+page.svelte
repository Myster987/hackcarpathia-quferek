<script lang="ts">
    import { goto } from "$app/navigation";
    import logo from "$lib/assets/logo.png";
    import { user } from "$lib/state.svelte";
    import { Eye, EyeOff } from "@lucide/svelte";

    let blad = $state("");
    let username = $state("");
    let password = $state("");
    let passwordConfirm = $state("");
    let showPassword = $state(false);

    async function signUp(e: Event) {
        e.preventDefault();

        if (password !== passwordConfirm) {
            blad = "Hasła muszą być identyczne";
            return 0;
        }
        blad = "";

        user.signUp(username, password).catch((err) => (blad = err));
    }
</script>

<div class="flex bg flex-1 items-center justify-center">
    <div class="w-100">
        <div
            class="card card-body card-border border-base-300 bg-base-200 p-6 flex gap-4"
        >
            <div class="flex flex-col justify-center items-center gap-6">
                <img src={logo} alt="Logo" class="size-36" />

                <h2 class="text-4xl text-center font-bold">Rejstracja</h2>
            </div>

            <form onsubmit={signUp} class="flex flex-col gap-4 items-center">
                <input
                    type="text"
                    bind:value={username}
                    placeholder="Nazwa użytkownika..."
                    class="input input-primary"
                />

                {#if blad !== ""}
                    <p class="text-error">{blad}</p>
                {/if}

                <input
                    type={showPassword ? "text" : "password"}
                    bind:value={password}
                    placeholder="Hasło..."
                    class="input input-primary"
                />
                <input
                    type={showPassword ? "text" : "password"}
                    bind:value={passwordConfirm}
                    placeholder="Powtórz hasło..."
                    class="input input-primary"
                />

                <button
                    type="button"
                    onclick={() => {
                        showPassword = !showPassword;
                    }}
                    class="btn btn-ghost btn-xs"
                    aria-label="Toggle visibility"
                >
                    {#if showPassword}
                        <Eye />
                    {:else}
                        <EyeOff />
                    {/if}
                </button>
                <div class="flex flex-col gap-2">
                    <button type="submit" class="btn btn-secondary btn-xl w-80">
                        Zarejestruj
                    </button>

                    <div class="text-center">Lub</div>

                    <button
                        type="button"
                        class="btn btn-secondary btn-xl"
                        onclick={() => goto("/sign-in")}
                        >Zaloguj się
                    </button>
                </div>
            </form>
        </div>
    </div>
</div>

<style>
    .bg {
        width: 100%;
        height: 100vh;
        background-size: 300% 300%;
        background-image: linear-gradient(
            -45deg,
            #ffe600 0%,
            #ffb800 20%,
            #ff357f 55%,
            #6a00ff 100%
        );
        animation: AnimateBG 20s ease infinite;
    }

    @keyframes AnimateBG {
        0% {
            background-position: 0% 50%;
        }
        50% {
            background-position: 100% 50%;
        }
        100% {
            background-position: 0% 50%;
        }
    }
</style>
