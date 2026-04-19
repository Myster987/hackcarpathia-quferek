<script lang="ts">
    import { user } from "$lib/state.svelte";
    import { LogOut } from "@lucide/svelte";

    let { children } = $props();
</script>

<div class="bg-base-200 flex flex-col flex-1 p-6">
    <nav class="flex flex-1 justify-start">
        <button
            onclick={() => user.signOut()}
            class="flex gap-2 btn-md btn btn-secondary mr-auto"
            ><LogOut /> Wyloguj</button
        >
    </nav>

    <div class="flex-1">
        {@render children()}
    </div>
</div>
