<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { authStore } from '$lib/stores/auth';
  import { api } from '$lib/api/client';

  let isAuthenticated = false;
  let teacher: any = null;

  authStore.subscribe(state => {
    isAuthenticated = state.isAuthenticated;
    teacher = state.teacher;
  });

  onMount(() => {
    if (!isAuthenticated) {
      goto('/login');
    }
  });

  async function handleLogout() {
    await api.logout();
    authStore.logout();
    goto('/login');
  }
</script>

{#if isAuthenticated}
  <div class="min-h-screen bg-gray-100">
    <nav class="bg-white shadow-md">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex justify-between h-16">
          <div class="flex items-center">
            <h1 class="text-xl font-bold text-gray-800">
              University NixOS Portal
            </h1>
          </div>
          
          <div class="flex items-center space-x-4">
            <a 
              href="/dashboard" 
              class="text-gray-700 hover:text-blue-600 px-3 py-2 rounded-md text-sm font-medium"
            >
              Configurations
            </a>
            
            <div class="flex items-center space-x-2">
              <span class="text-sm text-gray-600">
                {teacher?.full_name || 'User'}
              </span>
              <button
                on:click={handleLogout}
                class="bg-red-600 text-white px-4 py-2 rounded-md text-sm hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-red-500"
              >
                Logout
              </button>
            </div>
          </div>
        </div>
      </div>
    </nav>

    <main class="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
      <slot />
    </main>
  </div>
{:else}
  <div class="min-h-screen flex items-center justify-center">
    <div class="text-gray-600">Loading...</div>
  </div>
{/if}
