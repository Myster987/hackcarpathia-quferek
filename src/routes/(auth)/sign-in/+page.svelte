<script lang="ts">
    import logo from "$lib/assets/logo.png";
    import { session } from "$lib/vault.svelte";

    let username = $state("");
    let password = $state("");
    let show_password = $state(false);

    async function signIn(e: Event) {
        e.preventDefault();

        await session.login(username, password);
    }
</script>

<div class="flex flex-1 items-center justify-center">
    <div class="w-2/5 md:w-1/5">
        <div
            class="card card-body card-border border-base-300 bg-base-200 p-6 flex gap-4"
        >
            <div class="flex justify-center items-center gap-6">
                <img src={logo} alt="Logo" class="size-36" />

                <h2 class="text-4xl text-center font-bold">Logowanie</h2>
            </div>

            <form onsubmit={signIn} class="flex flex-col gap-4 items-center">
                <input
                    type="text"
                    bind:value={username}
                    placeholder="Nazwa użytkownika..."
                    class="input input-primary"
                />
                <input
                    type={show_password ? "text" : "password"}
                    bind:value={password}
                    placeholder="Hasło..."
                    class="input input-primary"
                />

                <button type="submit" class="btn btn-secondary">Zaloguj</button>
            </form>
        </div>
    </div>
</div>
