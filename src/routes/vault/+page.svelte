<script lang="ts">
    import { goto } from "$app/navigation";
    import { getAllLogins } from "$lib/state.svelte";
    let login_list_permission = $state(getAllLogins());
</script>

<div
    class="min-h-screen bg-base-200 flex items-start justify-center flex-1 p-8"
>
    <div class="w-full max-w-lg">
        <div class="flex flex-col gap-4 mb-6">
            <h1 class="text-3xl font-bold text-base-content">
                Wszystkie twoje loginy
            </h1>
            <p class="text-base-content/50 text-sm mt-1">
                Zarządzaj swoimi zapisanymi loginami
            </p>

            <button
                onclick={() => goto("/vault/new-login")}
                class="btn btn-primary btn-xl">Nowy login</button
            >
        </div>

        <div class="card bg-base-100 shadow-sm">
            <div class="card-body p-0">
                {#await login_list_permission}
                    <div class="flex justify-center items-center py-12">
                        <span
                            class="loading loading-spinner loading-md text-primary"
                        ></span>
                    </div>
                {:then login_list}
                    {#if login_list.length === 0}
                        <div
                            class="flex flex-col items-center justify-center py-12 text-base-content/40"
                        >
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                class="size-10 mb-3"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="1.5"
                                    d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"
                                />
                            </svg>
                            <p class="text-sm">Brak zapisanych loginów</p>
                        </div>
                    {:else}
                        <ul class="divide-y divide-base-200">
                            {#each login_list as login, i}
                                <li
                                    class="flex items-center gap-3 px-5 py-3.5 hover:bg-base-200/60 transition-colors"
                                >
                                    <div class="avatar placeholder">
                                        <div
                                            class="bg-primary/10 text-primary rounded-full size-8 text-xs font-semibold"
                                        >
                                            <span
                                                >{String(login)
                                                    .at(0)
                                                    ?.toUpperCase() ??
                                                    "#"}</span
                                            >
                                        </div>
                                    </div>
                                    <span
                                        class="text-sm text-base-content flex-1 truncate"
                                        >{login}</span
                                    >
                                    <span
                                        class="badge badge-ghost badge-sm text-base-content/30"
                                        >#{i + 1}</span
                                    >
                                </li>
                            {/each}
                        </ul>

                        <div class="px-5 py-3 border-t border-base-200">
                            <p class="text-xs text-base-content/30">
                                {login_list.length}
                                {login_list.length === 1 ? "login" : "loginów"}
                            </p>
                        </div>
                    {/if}
                {:catch error}
                    <div class="alert alert-error m-4">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="size-5 shrink-0"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
                            />
                        </svg>
                        <span class="text-sm">Błąd: {error.message}</span>
                    </div>
                {/await}
            </div>
        </div>
    </div>
</div>
