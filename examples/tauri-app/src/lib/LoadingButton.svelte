<script>
  let { 
    onclick, 
    disabled = false, 
    loading = false, 
    children,
    variant = 'primary',
    class: className = ''
  } = $props();

  let isProcessing = $state(false);

  async function handleClick(e) {
    if (disabled || isProcessing) return;
    
    isProcessing = true;
    try {
      await onclick?.(e);
    } finally {
      isProcessing = false;
    }
  }

  const isDisabled = $derived(disabled || isProcessing);
</script>

<button 
  onclick={handleClick} 
  disabled={isDisabled}
  class="loading-btn {variant} {className}"
  class:processing={isProcessing}
>
  {#if isProcessing}
    <span class="spinner"></span>
  {/if}
  <span class="btn-text" class:hidden={isProcessing && loading}>
    {@render children()}
  </span>
</button>

<style>
  .loading-btn {
    padding: 0.5rem 1rem;
    border: 1px solid #61dafb;
    border-radius: 4px;
    background: transparent;
    color: #61dafb;
    cursor: pointer;
    font-family: inherit;
    transition: all 0.2s;
    position: relative;
    min-width: 120px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
  }

  .loading-btn:hover:not(:disabled) {
    background: #61dafb;
    color: #1a1a1a;
  }

  .loading-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .loading-btn.processing {
    cursor: wait;
  }

  .loading-btn.secondary {
    border-color: #888;
    color: #888;
  }

  .loading-btn.secondary:hover:not(:disabled) {
    background: #888;
    color: #1a1a1a;
  }

  .spinner {
    width: 14px;
    height: 14px;
    border: 2px solid currentColor;
    border-top-color: transparent;
    border-radius: 50%;
    animation: spin 0.6s linear infinite;
    flex-shrink: 0;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .btn-text {
    transition: opacity 0.2s;
  }

  .btn-text.hidden {
    opacity: 0;
  }
</style>
