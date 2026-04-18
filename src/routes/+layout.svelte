<script lang="ts">
    import "../app.css";
    import { session } from "$lib/vault.svelte";
    import { goto } from "$app/navigation";
    import { page } from "$app/state";

    const PUBLIC_ROUTES = ["/sign-in", "/sign-up"];

    $effect(() => {
        if (
            !session.isAuthenticated &&
            !PUBLIC_ROUTES.includes(page.url.pathname)
        ) {
            goto("/sign-in");
        }
    });

    let { children } = $props();
</script>

<div class="flex min-h-screen">
    {@render children()}
</div>
