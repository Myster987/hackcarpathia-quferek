<script lang="ts">
    import { goto } from "$app/navigation";
    import { page } from "$app/stores";
    import { vault } from "$lib/vault.svelte";

    let { children } = $props();

    $effect(() => {
        if (!vault.initialized) goto("/");
    });

    const navLinks = [
        { href: "/vault", label: "All Passwords", icon: "🗝️" },
        { href: "/vault/new", label: "Add New", icon: "➕" },
    ];

    function isActive(href: string) {
        return $page.url.pathname === href;
    }
</script>

<div class="drawer lg:drawer-open">
    <input id="nav-drawer" type="checkbox" class="drawer-toggle" />

    <!-- Page content -->
    <div class="drawer-content flex flex-col min-h-screen">
        <!-- Top navbar (mobile) -->
        <nav
            class="navbar bg-base-100 border-b border-base-200 lg:hidden sticky top-0 z-10"
        >
            <div class="flex-none">
                <label
                    for="nav-drawer"
                    class="btn btn-square btn-ghost drawer-button"
                >
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 24 24"
                        class="inline-block h-5 w-5 stroke-current"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M4 6h16M4 12h16M4 18h16"
                        />
                    </svg>
                </label>
            </div>
            <div class="flex-1">
                <span class="text-lg font-bold">🔐 Vault</span>
            </div>
            <div class="flex-none">
                {#if vault.loading}
                    <span class="loading loading-spinner loading-sm mr-2"
                    ></span>
                {/if}
            </div>
        </nav>

        <!-- Toast notifications -->
        {#if vault.success || vault.error}
            <div class="toast toast-top toast-end z-50">
                {#if vault.success}
                    <div class="alert alert-success shadow-lg">
                        <span>✓ {vault.success}</span>
                    </div>
                {/if}
                {#if vault.error}
                    <div class="alert alert-error shadow-lg">
                        <span>✗ {vault.error}</span>
                    </div>
                {/if}
            </div>
        {/if}

        <!-- Main content -->
        <main class="flex-1 p-6 bg-base-200">
            {@render children()}
        </main>
    </div>

    <!-- Sidebar -->
    <div class="drawer-side z-20">
        <label
            for="nav-drawer"
            aria-label="close sidebar"
            class="drawer-overlay"
        ></label>
        <aside
            class="w-64 min-h-screen bg-base-100 border-r border-base-200 flex flex-col"
        >
            <!-- Brand -->
            <div class="p-5 border-b border-base-200">
                <div class="flex items-center gap-3">
                    <span class="text-3xl">🔐</span>
                    <div>
                        <p class="font-bold text-lg leading-none">Vault</p>
                        <p class="text-xs text-base-content/50 mt-0.5">
                            {vault.keys.length} password{vault.keys.length !== 1
                                ? "s"
                                : ""}
                        </p>
                    </div>
                </div>
            </div>

            <!-- Navigation -->
            <nav class="flex-1 p-3">
                <ul class="menu menu-md gap-1">
                    {#each navLinks as link}
                        <li>
                            href={link.href}
                            class={isActive(link.href) ? "active" : ""}
                            >
                            <span>{link.icon}</span>
                            {link.label}
                        </li>
                    {/each}
                </ul>
            </nav>

            <!-- Footer: lock button -->
            <div class="p-3 border-t border-base-200">
                <button
                    class="btn btn-ghost btn-sm w-full justify-start gap-2 text-error"
                    onclick={() => goto("/")}
                >
                    🔒 Lock Vault
                </button>
            </div>
        </aside>
    </div>
</div>
