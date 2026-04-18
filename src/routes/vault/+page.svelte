<script lang="ts">
    import { session } from "../../lib/vault.svelte";

    // let login_list = $state(session.listLogins());
</script>

<p>Vault</p>

<!-- {#await login_list then list}
    {#each list as login}
        <p>Login: {login}</p>
    {/each}
{/await} -->
