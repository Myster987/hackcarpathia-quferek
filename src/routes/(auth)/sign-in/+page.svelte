<script lang="ts">
    import { session } from "$lib/vault.svelte";

    let username = $state("");
    let password = $state("");
    let show_password = $state(false);

    async function signIn(e: Event) {
        e.preventDefault();

        await session.login(username, password);
    }
</script>

<div class="flex flex-1 items-center justify-center p-4">
    <div class="card w-2/5">
        <form onsubmit={signIn} class="flex flex-col gap-4">
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
