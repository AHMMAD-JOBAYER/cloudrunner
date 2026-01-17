<script lang="ts">
  import { authStore } from '$lib/stores/auth';

  let isAuthenticated = false;

  authStore.subscribe(state => {
    isAuthenticated = state.isAuthenticated;
  });
</script>

<svelte:head>
  <title>University NixOS Portal - Manage Your Configurations</title>
</svelte:head>

<div class="min-h-screen bg-gradient-to-br from-blue-50 via-white to-purple-50">
  <!-- Navigation -->
  <nav class="bg-white shadow-sm">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="flex justify-between items-center h-16">
        <div class="flex items-center">
          <h1 class="text-2xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
            NixOS Portal
          </h1>
        </div>
        <div class="flex items-center space-x-4">
          {#if isAuthenticated}
            <a
              href="/dashboard"
              class="bg-blue-600 text-white px-6 py-2 rounded-lg hover:bg-blue-700 transition-colors font-medium"
            >
              Dashboard
            </a>
          {:else}
            <a
              href="/login"
              class="text-gray-700 hover:text-blue-600 px-4 py-2 rounded-lg transition-colors font-medium"
            >
              Login
            </a>
            <a
              href="/register"
              class="bg-blue-600 text-white px-6 py-2 rounded-lg hover:bg-blue-700 transition-colors font-medium"
            >
              Get Started
            </a>
          {/if}
        </div>
      </div>
    </div>
  </nav>

  <!-- Hero Section -->
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 pt-20 pb-16">
    <div class="text-center">
      <h2 class="text-5xl md:text-6xl font-extrabold text-gray-900 mb-6">
        Manage Your NixOS Configurations
        <span class="block text-blue-600 mt-2">Seamlessly in the Cloud</span>
      </h2>
      <p class="text-xl text-gray-600 max-w-3xl mx-auto mb-10">
        A powerful platform designed for university teachers to store, manage, and version control
        their NixOS configuration files with ease and security.
      </p>
      <div class="flex justify-center space-x-4">
        {#if isAuthenticated}
          <a
            href="/dashboard"
            class="bg-blue-600 text-white px-8 py-4 rounded-lg text-lg font-semibold hover:bg-blue-700 transition-colors shadow-lg hover:shadow-xl"
          >
            Go to Dashboard →
          </a>
        {:else}
          <a
            href="/register"
            class="bg-blue-600 text-white px-8 py-4 rounded-lg text-lg font-semibold hover:bg-blue-700 transition-colors shadow-lg hover:shadow-xl"
          >
            Get Started Free →
          </a>
          <a
            href="/login"
            class="bg-white text-blue-600 px-8 py-4 rounded-lg text-lg font-semibold hover:bg-gray-50 transition-colors border-2 border-blue-600"
          >
            Sign In
          </a>
        {/if}
      </div>
    </div>
  </div>

  <!-- Features Section -->
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
    <h3 class="text-3xl font-bold text-center text-gray-900 mb-12">
      Why Choose NixOS Portal?
    </h3>
    
    <div class="grid md:grid-cols-3 gap-8 mb-16">
      <!-- Feature 1 -->
      <div class="bg-white rounded-xl shadow-md p-8 hover:shadow-xl transition-shadow">
        <div class="w-12 h-12 bg-blue-100 rounded-lg flex items-center justify-center mb-4">
          <svg class="w-6 h-6 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
          </svg>
        </div>
        <h4 class="text-xl font-semibold text-gray-900 mb-3">Secure Storage</h4>
        <p class="text-gray-600">
          Your NixOS configurations are encrypted and securely stored with enterprise-grade security. 
          Only you can access your files with JWT-based authentication.
        </p>
      </div>

      <!-- Feature 2 -->
      <div class="bg-white rounded-xl shadow-md p-8 hover:shadow-xl transition-shadow">
        <div class="w-12 h-12 bg-purple-100 rounded-lg flex items-center justify-center mb-4">
          <svg class="w-6 h-6 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4" />
          </svg>
        </div>
        <h4 class="text-xl font-semibold text-gray-900 mb-3">Version Control</h4>
        <p class="text-gray-600">
          Track changes with automatic timestamps. View when configurations were created and last modified.
          Never lose track of your system setup evolution.
        </p>
      </div>

      <!-- Feature 3 -->
      <div class="bg-white rounded-xl shadow-md p-8 hover:shadow-xl transition-shadow">
        <div class="w-12 h-12 bg-green-100 rounded-lg flex items-center justify-center mb-4">
          <svg class="w-6 h-6 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
          </svg>
        </div>
        <h4 class="text-xl font-semibold text-gray-900 mb-3">Lightning Fast</h4>
        <p class="text-gray-600">
          Built with modern technologies like Rust and SvelteKit for blazing-fast performance.
          Access your configurations instantly, anywhere, anytime.
        </p>
      </div>

      <!-- Feature 4 -->
      <div class="bg-white rounded-xl shadow-md p-8 hover:shadow-xl transition-shadow">
        <div class="w-12 h-12 bg-yellow-100 rounded-lg flex items-center justify-center mb-4">
          <svg class="w-6 h-6 text-yellow-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
          </svg>
        </div>
        <h4 class="text-xl font-semibold text-gray-900 mb-3">Easy Editing</h4>
        <p class="text-gray-600">
          Edit your configurations directly in the browser with our intuitive interface.
          Syntax highlighting and validation coming soon!
        </p>
      </div>

      <!-- Feature 5 -->
      <div class="bg-white rounded-xl shadow-md p-8 hover:shadow-xl transition-shadow">
        <div class="w-12 h-12 bg-red-100 rounded-lg flex items-center justify-center mb-4">
          <svg class="w-6 h-6 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 15a4 4 0 004 4h9a5 5 0 10-.1-9.999 5.002 5.002 0 10-9.78 2.096A4.001 4.001 0 003 15z" />
          </svg>
        </div>
        <h4 class="text-xl font-semibold text-gray-900 mb-3">Cloud Accessible</h4>
        <p class="text-gray-600">
          Access your configurations from any device with an internet connection.
          No more USB drives or email attachments between your machines.
        </p>
      </div>

      <!-- Feature 6 -->
      <div class="bg-white rounded-xl shadow-md p-8 hover:shadow-xl transition-shadow">
        <div class="w-12 h-12 bg-indigo-100 rounded-lg flex items-center justify-center mb-4">
          <svg class="w-6 h-6 text-indigo-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4" />
          </svg>
        </div>
        <h4 class="text-xl font-semibold text-gray-900 mb-3">Multi-Config Support</h4>
        <p class="text-gray-600">
          Manage multiple NixOS configurations for different machines or setups.
          Perfect for managing lab computers, personal workstations, and servers.
        </p>
      </div>
    </div>
  </div>

  <!-- How It Works Section -->
  <div class="bg-white py-16">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <h3 class="text-3xl font-bold text-center text-gray-900 mb-12">
        How It Works
      </h3>
      
      <div class="grid md:grid-cols-4 gap-8">
        <div class="text-center">
          <div class="w-16 h-16 bg-blue-600 text-white rounded-full flex items-center justify-center text-2xl font-bold mx-auto mb-4">
            1
          </div>
          <h4 class="text-lg font-semibold mb-2">Sign Up</h4>
          <p class="text-gray-600">Create your free account with your university email</p>
        </div>

        <div class="text-center">
          <div class="w-16 h-16 bg-blue-600 text-white rounded-full flex items-center justify-center text-2xl font-bold mx-auto mb-4">
            2
          </div>
          <h4 class="text-lg font-semibold mb-2">Upload Configs</h4>
          <p class="text-gray-600">Upload your NixOS configuration files securely</p>
        </div>

        <div class="text-center">
          <div class="w-16 h-16 bg-blue-600 text-white rounded-full flex items-center justify-center text-2xl font-bold mx-auto mb-4">
            3
          </div>
          <h4 class="text-lg font-semibold mb-2">Edit & Manage</h4>
          <p class="text-gray-600">Edit configurations directly in your browser</p>
        </div>

        <div class="text-center">
          <div class="w-16 h-16 bg-blue-600 text-white rounded-full flex items-center justify-center text-2xl font-bold mx-auto mb-4">
            4
          </div>
          <h4 class="text-lg font-semibold mb-2">Access Anywhere</h4>
          <p class="text-gray-600">Download and deploy from any device</p>
        </div>
      </div>
    </div>
  </div>

  <!-- CTA Section -->
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
    <div class="bg-gradient-to-r from-blue-600 to-purple-600 rounded-2xl shadow-2xl overflow-hidden">
      <div class="px-8 py-16 text-center">
        <h3 class="text-3xl md:text-4xl font-bold text-white mb-4">
          Ready to Get Started?
        </h3>
        <p class="text-xl text-blue-100 mb-8 max-w-2xl mx-auto">
          Join university teachers worldwide who trust NixOS Portal to manage their configurations
        </p>
        {#if !isAuthenticated}
          <a
            href="/register"
            class="inline-block bg-white text-blue-600 px-8 py-4 rounded-lg text-lg font-semibold hover:bg-gray-100 transition-colors shadow-lg"
          >
            Create Free Account →
          </a>
        {/if}
      </div>
    </div>
  </div>

  <!-- Footer -->
  <footer class="bg-gray-900 text-white py-12">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="grid md:grid-cols-3 gap-8">
        <div>
          <h4 class="text-lg font-semibold mb-4">About NixOS Portal</h4>
          <p class="text-gray-400">
            A secure platform for managing NixOS configurations, built specifically for educational institutions.
          </p>
        </div>
        <div>
          <h4 class="text-lg font-semibold mb-4">Quick Links</h4>
          <ul class="space-y-2 text-gray-400">
            <li><a href="/login" class="hover:text-white transition-colors">Login</a></li>
            <li><a href="/register" class="hover:text-white transition-colors">Register</a></li>
            {#if isAuthenticated}
              <li><a href="/dashboard" class="hover:text-white transition-colors">Dashboard</a></li>
            {/if}
          </ul>
        </div>
        <div>
          <h4 class="text-lg font-semibold mb-4">Technology</h4>
          <ul class="space-y-2 text-gray-400">
            <li>Rust + Axum Backend</li>
            <li>SvelteKit + Deno Frontend</li>
            <li>PostgreSQL Database</li>
            <li>JWT Authentication</li>
          </ul>
        </div>
      </div>
      <div class="border-t border-gray-800 mt-8 pt-8 text-center text-gray-400">
        <p>&copy; 2024 University NixOS Portal. Built with ❤️ for the NixOS community.</p>
      </div>
    </div>
  </footer>
</div>
