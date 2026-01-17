<script lang="ts">
  import { goto } from '$app/navigation';
  import { api } from '$lib/api/client';

  let email = '';
  let password = '';
  let confirmPassword = '';
  let fullName = '';
  let department = '';
  let error = '';
  let success = '';
  let loading = false;

  async function handleRegister() {
    if (!email || !password || !fullName) {
      error = 'Please fill in all required fields';
      return;
    }

    if (password.length < 8) {
      error = 'Password must be at least 8 characters';
      return;
    }

    if (password !== confirmPassword) {
      error = 'Passwords do not match';
      return;
    }

    loading = true;
    error = '';
    success = '';

    try {
      await api.register({
        email,
        password,
        full_name: fullName,
        department: department || undefined,
      });
      
      success = 'Registration successful! Redirecting to login...';
      setTimeout(() => goto('/login'), 2000);
    } catch (e: any) {
      error = e.message || 'Registration failed';
    } finally {
      loading = false;
    }
  }
</script>

<div class="min-h-screen flex items-center justify-center bg-gray-100 py-12">
  <div class="max-w-md w-full bg-white rounded-lg shadow-md p-8">
    <h1 class="text-3xl font-bold text-center mb-8 text-gray-800">
      University NixOS Portal
    </h1>
    
    <h2 class="text-xl font-semibold mb-6 text-gray-700">Register</h2>

    {#if error}
      <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
        {error}
      </div>
    {/if}

    {#if success}
      <div class="bg-green-100 border border-green-400 text-green-700 px-4 py-3 rounded mb-4">
        {success}
      </div>
    {/if}

    <form on:submit|preventDefault={handleRegister} class="space-y-4">
      <div>
        <label for="fullName" class="block text-sm font-medium text-gray-700 mb-1">
          Full Name <span class="text-red-500">*</span>
        </label>
        <input
          id="fullName"
          type="text"
          bind:value={fullName}
          required
          class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          placeholder="Dr. Jane Smith"
        />
      </div>

      <div>
        <label for="email" class="block text-sm font-medium text-gray-700 mb-1">
          Email <span class="text-red-500">*</span>
        </label>
        <input
          id="email"
          type="email"
          bind:value={email}
          required
          class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          placeholder="teacher@university.edu"
        />
      </div>

      <div>
        <label for="department" class="block text-sm font-medium text-gray-700 mb-1">
          Department
        </label>
        <input
          id="department"
          type="text"
          bind:value={department}
          class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          placeholder="Computer Science"
        />
      </div>

      <div>
        <label for="password" class="block text-sm font-medium text-gray-700 mb-1">
          Password <span class="text-red-500">*</span>
        </label>
        <input
          id="password"
          type="password"
          bind:value={password}
          required
          class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          placeholder="••••••••"
        />
        <p class="text-xs text-gray-500 mt-1">Minimum 8 characters</p>
      </div>

      <div>
        <label for="confirmPassword" class="block text-sm font-medium text-gray-700 mb-1">
          Confirm Password <span class="text-red-500">*</span>
        </label>
        <input
          id="confirmPassword"
          type="password"
          bind:value={confirmPassword}
          required
          class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          placeholder="••••••••"
        />
      </div>

      <button
        type="submit"
        disabled={loading}
        class="w-full bg-blue-600 text-white py-2 px-4 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed"
      >
        {loading ? 'Registering...' : 'Register'}
      </button>
    </form>

    <div class="mt-6 text-center">
      <a href="/login" class="text-blue-600 hover:text-blue-800">
        Already have an account? Login
      </a>
    </div>
  </div>
</div>
