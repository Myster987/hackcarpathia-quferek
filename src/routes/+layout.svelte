<script lang="ts">
    import "../app.css";
    import { user } from "$lib/state.svelte";
    import { goto } from "$app/navigation";
    import { page } from "$app/state";

    const PUBLIC_ROUTES = ["/sign-in", "/sign-up"];

    $effect(() => {
        if (
            !user.isAuthenticated &&
            !PUBLIC_ROUTES.includes(page.url.pathname)
        ) {
            goto("/sign-in");
        }
    });

    let { children } = $props();
</script>

<div class="flex min-h-screen bg-200">
    {@render children()}
</div>
