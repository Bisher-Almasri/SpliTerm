<script lang="ts">
  import Terminal from "./Terminal.svelte";

  let terms = $state([{ id: 1 }, { id: 2 }, { id: 3 }, { id: 4 }, { id: 5 }]);

  let activeId = $state(5);
  let nextId = 6;

  function spawnTerminal() {
    terms.push({ id: nextId });
    activeId = nextId;
    nextId++;
  }

  function closeTerminal(id: number) {
    terms = terms.filter((w) => w.id != id);
    if (activeId === id && terms.length > 0) {
      activeId = terms[terms.length - 1].id;
    }
  }
</script>

{#snippet dwindleTree(items: { id: number }[], depth = 0)}
  {#if items.length === 1}
    {@const win = items[0]}
    <div
      class="window"
      class:active={activeId === win.id}
      onclick={() => (activeId = win.id)}
      onmouseenter={() => (activeId = win.id)}
      role="button"
      tabindex="0"
      onkeydown={(e) => e.key === "Enter" && (activeId = win.id)}
    >
      <Terminal active={activeId == win.id} />
    </div>
  {:else if items.length >= 1}
    {@const current = items[0]}
    {@const remaining = items.slice(1)}

    <div class="split {depth % 2 == 0 ? 'row' : 'col'}">
      <div
        class="window"
        class:active={activeId === current.id}
        onmouseenter={() => (activeId = current.id)}
        onclick={() => (activeId = current.id)}
        role="button"
        tabindex="0"
        onkeydown={(e) => e.key === "Enter" && (activeId = current.id)}
      >
        <Terminal active={activeId == current.id} />
      </div>

      {@render dwindleTree(remaining, depth + 1)}
    </div>
  {/if}
{/snippet}

<div class="container">
  <main class="workspace">
    {#if terms.length > 0}
      {@render dwindleTree(terms, 0)}
    {:else}
      <div class="empty-state">No active windows. Click "+ Spawn Window".</div>
    {/if}
  </main>
</div>

<style>
  .container {
    display: flex;
    flex-direction: column;
    height: 100%;
    width: 100%;
    padding: 10px;
    box-sizing: border-box;
    gap: 10px;
  }

  .workspace {
    display: flex;
    flex: 1;
    gap: 10px;
    min-height: 0;
  }

  .split {
    display: flex;
    flex: 1;
    gap: 10px;
    min-width: 0;
    min-height: 0;
  }

  .split.row {
    flex-direction: row;
  }

  .split.col {
    flex-direction: column;
  }

  .window {
    position: relative;
    flex: 1;
    border: 2px solid #45475a;
    border-radius: 10px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    user-select: none;
    transition: border-color 0.15s ease;
    min-width: 0;
    min-height: 0;
  }

  .window.active {
    border-color: #89b4fa;
  }

  .empty-state {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    color: #6c7086;
    border: 2px dashed #313244;
    border-radius: 10px;
  }
</style>
