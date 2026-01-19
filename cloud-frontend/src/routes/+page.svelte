<script lang="ts">
  import { authStore } from '$lib/stores/auth';

  let isAuthenticated = false;

  const unsubscribe = authStore.subscribe(state => {
    isAuthenticated = state.isAuthenticated;
  });

  // optional cleanup
  // onDestroy(unsubscribe);
</script>

<svelte:head>
  <title>VDaaS – Instant Cloud Desktops in Your Browser</title>
  <meta
    name="description"
    content="Launch fully configured virtual desktops in seconds. No setup, no downloads, accessible from anywhere."
  />
</svelte:head>

<div class="min-h-screen bg-gradient-to-br from-indigo-50 via-white to-blue-50">

  <!-- NAVBAR -->
  <nav class="bg-white border-b border-gray-200 sticky top-0 z-50">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="flex justify-between items-center h-16">
        <div class="flex items-center space-x-3">
          <div class="w-10 h-10 bg-gradient-to-br from-indigo-600 to-blue-600 rounded-lg flex items-center justify-center">
            <svg class="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
            </svg>
          </div>
          <span class="text-xl font-bold text-gray-900">VDaaS</span>
        </div>

        <div class="flex items-center space-x-4">
          {#if isAuthenticated}
            <a href="/dashboard"
              class="bg-indigo-600 text-white px-6 py-2 rounded-lg hover:bg-indigo-700 transition font-medium">
              Dashboard
            </a>
          {:else}
            <a href="/login" class="text-gray-700 hover:text-indigo-600 font-medium">
              Sign In
            </a>
            <a href="/register"
              class="bg-indigo-600 text-white px-6 py-2 rounded-lg hover:bg-indigo-700 transition font-medium">
              Get Started
            </a>
          {/if}
        </div>
      </div>
    </div>
  </nav>

  <!-- HERO -->
  <section class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 pt-20 pb-24 text-center">
    <h1 class="text-4xl md:text-6xl font-extrabold text-gray-900 mb-6">
      Instant Virtual Desktops
      <span class="block bg-gradient-to-r from-indigo-600 to-blue-600 bg-clip-text text-transparent">
        Ready in Seconds
      </span>
    </h1>

    <p class="text-xl text-gray-600 max-w-3xl mx-auto mb-10 leading-relaxed">
      Launch fully configured cloud desktops directly in your browser.
      No installation, no compatibility issues, no wasted setup time.
    </p>

    <div class="flex justify-center gap-4 flex-wrap">
      {#if isAuthenticated}
        <a href="/dashboard"
          class="bg-indigo-600 text-white px-8 py-4 rounded-lg text-lg font-semibold hover:bg-indigo-700 shadow-lg">
          Open Dashboard →
        </a>
      {:else}
        <a href="/register"
          class="bg-indigo-600 text-white px-8 py-4 rounded-lg text-lg font-semibold hover:bg-indigo-700 shadow-lg">
          Launch Your Desktop →
        </a>
        <a href="/login"
          class="bg-white text-indigo-600 px-8 py-4 rounded-lg text-lg font-semibold border-2 border-indigo-600 hover:bg-gray-50">
          Sign In
        </a>
      {/if}
    </div>
  </section>

  <!-- WHY -->
  <section class="bg-white py-20 border-y border-gray-200">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="text-center mb-14">
        <h2 class="text-3xl font-bold text-gray-900 mb-4">
          Why Teams Choose VDaaS
        </h2>
        <p class="text-gray-600 max-w-2xl mx-auto">
          Traditional setups slow people down. VDaaS removes friction completely.
        </p>
      </div>

      <div class="grid md:grid-cols-3 gap-8">
        <div class="rounded-xl p-8 bg-indigo-50 border border-indigo-100">
          <h3 class="text-xl font-semibold mb-3">Zero Setup</h3>
          <p class="text-gray-700">
            Forget installing tools or fixing dependencies.
            Your desktop is ready the moment you open it.
          </p>
        </div>

        <div class="rounded-xl p-8 bg-blue-50 border border-blue-100">
          <h3 class="text-xl font-semibold mb-3">Work Anywhere</h3>
          <p class="text-gray-700">
            Access your desktop from any device using a browser.
            No powerful hardware required.
          </p>
        </div>

        <div class="rounded-xl p-8 bg-green-50 border border-green-100">
          <h3 class="text-xl font-semibold mb-3">Cost Efficient</h3>
          <p class="text-gray-700">
            Desktops automatically shut down when inactive.
            You only pay for what you actually use.
          </p>
        </div>
      </div>
    </div>
  </section>

  <!-- FEATURES -->
  <section class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-20">
    <div class="text-center mb-14">
      <h2 class="text-3xl font-bold text-gray-900 mb-4">
        Built for Productivity
      </h2>
      <p class="text-gray-600 max-w-2xl mx-auto">
        Everything you need to start working instantly.
      </p>
    </div>

    <div class="grid md:grid-cols-3 gap-8">
      <div class="bg-white p-8 rounded-xl shadow border">
        <h3 class="text-xl font-semibold mb-3">Launch in Seconds</h3>
        <p class="text-gray-600">
          Start a fully configured desktop in under a minute.
          No waiting, no manual steps.
        </p>
      </div>

      <div class="bg-white p-8 rounded-xl shadow border">
        <h3 class="text-xl font-semibold mb-3">Runs in Your Browser</h3>
        <p class="text-gray-600">
          No downloads or plugins required.
          Works on Chrome, Edge, Safari, and more.
        </p>
      </div>

      <div class="bg-white p-8 rounded-xl shadow border">
        <h3 class="text-xl font-semibold mb-3">Private by Default</h3>
        <p class="text-gray-600">
          Each session is isolated and temporary.
          Your data stays private and secure.
        </p>
      </div>
    </div>
  </section>

  <!-- CTA -->
  <section class="bg-gradient-to-r from-indigo-600 to-blue-600 py-24 text-center">
    <div class="max-w-3xl mx-auto px-4">
      <h2 class="text-4xl font-extrabold text-white mb-6">
        Your Desktop. Anywhere. Instantly.
      </h2>
      <p class="text-indigo-100 text-lg mb-10">
        Stop wasting time on setup. Start working immediately.
      </p>

      {#if isAuthenticated}
        <a href="/dashboard"
          class="bg-white text-indigo-600 px-10 py-4 rounded-lg text-lg font-bold hover:bg-gray-100">
          Go to Dashboard →
        </a>
      {:else}
        <a href="/register"
          class="bg-white text-indigo-600 px-10 py-4 rounded-lg text-lg font-bold hover:bg-gray-100">
          Get Started Free →
        </a>
      {/if}
    </div>
  </section>

  <!-- FOOTER -->
  <footer class="bg-gray-900 text-gray-400 py-10">
    <div class="max-w-7xl mx-auto px-4 text-center">
      <h3 class="text-white text-lg font-semibold mb-2">VDaaS</h3>
      <p class="text-sm">
        Instant cloud desktops. Zero setup. Work from anywhere.
      </p>
      <p class="text-xs mt-4">
        © 2025 VDaaS. All rights reserved.
      </p>
    </div>
  </footer>

</div>
