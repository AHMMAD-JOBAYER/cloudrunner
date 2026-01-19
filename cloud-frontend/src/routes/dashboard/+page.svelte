<script lang="ts">
  import { onMount } from 'svelte';
  import { api, type NixOsConfig } from '$lib/api/client';

  let configs: NixOsConfig[] = [];
  let loading = true;
  let error = '';
  let showUploadModal = false;
  let showEditModal = false;
  let editingConfig: NixOsConfig | null = null;

  // Upload form
  let uploadFilename = '';
  let uploadContent = '';
  let uploadError = '';
  let uploading = false;

  // Edit form
  let editContent = '';
  let editError = '';
  let updating = false;

  onMount(async () => {
    await loadConfigs();
  });

  async function loadConfigs() {
    loading = true;
    error = '';
    try {
      const response = await api.listConfigs();
      configs = response.data || [];
    } catch (e: any) {
      error = e.message || 'Failed to load configurations';
    } finally {
      loading = false;
    }
  }

  async function handleUpload() {
    if (!uploadFilename.endsWith('.nix')) {
      uploadError = 'Filename must end with .nix';
      return;
    }

    if (!uploadContent.trim()) {
      uploadError = 'Content cannot be empty';
      return;
    }

    uploading = true;
    uploadError = '';

    try {
      await api.uploadConfig(uploadFilename, uploadContent);
      showUploadModal = false;
      uploadFilename = '';
      uploadContent = '';
      await loadConfigs();
    } catch (e: any) {
      uploadError = e.message || 'Upload failed';
    } finally {
      uploading = false;
    }
  }

  function openEditModal(config: NixOsConfig) {
    editingConfig = config;
    editContent = config.content;
    editError = '';
    showEditModal = true;
  }

  async function handleUpdate() {
    if (!editingConfig) return;

    if (!editContent.trim()) {
      editError = 'Content cannot be empty';
      return;
    }

    updating = true;
    editError = '';

    try {
      await api.updateConfig(editingConfig.id, editContent);
      showEditModal = false;
      editingConfig = null;
      editContent = '';
      await loadConfigs();
    } catch (e: any) {
      editError = e.message || 'Update failed';
    } finally {
      updating = false;
    }
  }

  async function handleDelete(id: string, filename: string) {
    if (!confirm(`Are you sure you want to delete ${filename}?`)) {
      return;
    }

    try {
      await api.deleteConfig(id);
      await loadConfigs();
    } catch (e: any) {
      alert(e.message || 'Delete failed');
    }
  }

  function formatDate(dateStr: string): string {
    return new Date(dateStr).toLocaleString();
  }

  function formatFileSize(bytes: number): string {
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(2)} KB`;
    return `${(bytes / (1024 * 1024)).toFixed(2)} MB`;
  }
</script>

<div class="px-4 py-6">
  <div class="flex justify-between items-center mb-6">
    <h2 class="text-2xl font-bold text-gray-800">NixOS Configurations</h2>
    <button
      on:click={() => showUploadModal = true}
      class="bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500"
    >
      + Upload Configuration
    </button>
  </div>

  {#if error}
    <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
      {error}
    </div>
  {/if}

  {#if loading}
    <div class="text-center py-12">
      <div class="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
      <p class="mt-4 text-gray-600">Loading configurations...</p>
    </div>
  {:else if configs.length === 0}
    <div class="bg-white rounded-lg shadow-md p-12 text-center">
      <p class="text-gray-600 mb-4">No configurations yet</p>
      <button
        on:click={() => showUploadModal = true}
        class="bg-blue-600 text-white px-6 py-3 rounded-md hover:bg-blue-700"
      >
        Upload Your First Configuration
      </button>
    </div>
  {:else}
    <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
      {#each configs as config}
        <div class="bg-white rounded-lg shadow-md p-6 hover:shadow-lg transition-shadow">
          <div class="flex justify-between items-start mb-3">
            <h3 class="text-lg font-semibold text-gray-800 truncate" title={config.filename}>
              {config.filename}
            </h3>
            <span class="text-xs text-gray-500 whitespace-nowrap ml-2">
              {formatFileSize(config.file_size)}
            </span>
          </div>

          <div class="mb-4">
            <div class="bg-gray-100 rounded p-3 max-h-32 overflow-y-auto">
              <pre class="text-xs text-gray-700 font-mono">{config.content.substring(0, 200)}{config.content.length > 200 ? '...' : ''}</pre>
            </div>
          </div>

          <div class="text-xs text-gray-500 mb-4">
            <p>Created: {formatDate(config.created_at)}</p>
            <p>Updated: {formatDate(config.updated_at)}</p>
          </div>

          <div class="flex space-x-2">
            <button
              on:click={() => openEditModal(config)}
              class="flex-1 bg-green-600 text-white px-3 py-2 rounded text-sm hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-green-500"
            >
              Edit
            </button>
            <button
              on:click={() => handleDelete(config.id, config.filename)}
              class="flex-1 bg-red-600 text-white px-3 py-2 rounded text-sm hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-red-500"
            >
              Delete
            </button>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<!-- Upload Modal -->
{#if showUploadModal}
  <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50">
    <div class="bg-white rounded-lg shadow-xl max-w-2xl w-full max-h-[90vh] overflow-y-auto">
      <div class="p-6">
        <h3 class="text-xl font-bold mb-4">Upload NixOS Configuration</h3>

        {#if uploadError}
          <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
            {uploadError}
          </div>
        {/if}

        <form on:submit|preventDefault={handleUpload} class="space-y-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">
              Filename
            </label>
            <input
              type="text"
              bind:value={uploadFilename}
              placeholder="configuration.nix"
              required
              class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">
              Content
            </label>
            <textarea
              bind:value={uploadContent}
              rows="15"
              required
              class="w-full px-3 py-2 border border-gray-300 rounded-md font-mono text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder={`{ config, pkgs, ... }:

{
  # Your NixOS configuration here
}`}
            ></textarea>
          </div>

          <div class="flex space-x-3">
            <button
              type="submit"
              disabled={uploading}
              class="flex-1 bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {uploading ? 'Uploading...' : 'Upload'}
            </button>
            <button
              type="button"
              on:click={() => { showUploadModal = false; uploadFilename = ''; uploadContent = ''; uploadError = ''; }}
              class="flex-1 bg-gray-300 text-gray-700 px-4 py-2 rounded-md hover:bg-gray-400"
            >
              Cancel
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
{/if}

<!-- Edit Modal -->
{#if showEditModal && editingConfig}
  <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50">
    <div class="bg-white rounded-lg shadow-xl max-w-2xl w-full max-h-[90vh] overflow-y-auto">
      <div class="p-6">
        <h3 class="text-xl font-bold mb-4">Edit {editingConfig.filename}</h3>

        {#if editError}
          <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
            {editError}
          </div>
        {/if}

        <form on:submit|preventDefault={handleUpdate} class="space-y-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">
              Content
            </label>
            <textarea
              bind:value={editContent}
              rows="15"
              required
              class="w-full px-3 py-2 border border-gray-300 rounded-md font-mono text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
            ></textarea>
          </div>

          <div class="flex space-x-3">
            <button
              type="submit"
              disabled={updating}
              class="flex-1 bg-green-600 text-white px-4 py-2 rounded-md hover:bg-green-700 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {updating ? 'Saving...' : 'Save Changes'}
            </button>
            <button
              type="button"
              on:click={() => { showEditModal = false; editingConfig = null; editContent = ''; editError = ''; }}
              class="flex-1 bg-gray-300 text-gray-700 px-4 py-2 rounded-md hover:bg-gray-400"
            >
              Cancel
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
{/if}
