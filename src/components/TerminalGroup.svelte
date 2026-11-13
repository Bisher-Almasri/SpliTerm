<script lang="ts">
  import Terminal from "./Terminal.svelte";

  let {...rest} = $props()

  let terms = $state([{ id: 1 }]);

  let activeId = $state(1);
  let nextId = 2;

  type PaneGeometry = {
    left: number;
    top: number;
    width: number;
    height: number;
  };

  function getPaneGeometry(index: number, count: number): PaneGeometry {
    let left = 0;
    let top = 0;
    let width = 100;
    let height = 100;
    let depth = 0;

    while (count > 1) {
      const currentWidth = width / 2;
      const currentHeight = height / 2;

      if (index === 0) {
        if (depth % 2 === 0) {
          width = currentWidth;
        } else {
          height = currentHeight;
        }
        break;
      }

      if (depth % 2 === 0) {
        left += currentWidth;
        width = currentWidth;
      } else {
        top += currentHeight;
        height = currentHeight;
      }

      index--;
      count--;
      depth++;
    }

    return { left, top, width, height };
  }

  function getPaneStyle(index: number, count: number) {
    const pane = getPaneGeometry(index, count);
    return `left: ${pane.left}%; top: ${pane.top}%; width: ${pane.width}%; height: ${pane.height}%;`;
  }

  function moveActive(direction: "left" | "right" | "up" | "down") {
    const currentIndex = terms.findIndex((term) => term.id === activeId);
    if (currentIndex < 0 || terms.length < 2) return;

    const current = getPaneGeometry(currentIndex, terms.length);
    const currentCenter = {
      x: current.left + current.width / 2,
      y: current.top + current.height / 2,
    };

    let bestIndex = -1;
    let bestDistance = Infinity;

    terms.forEach((_, index) => {
      if (index === currentIndex) return;

      const pane = getPaneGeometry(index, terms.length);
      const center = {
        x: pane.left + pane.width / 2,
        y: pane.top + pane.height / 2,
      };
      const deltaX = center.x - currentCenter.x;
      const deltaY = center.y - currentCenter.y;
      const isInDirection =
        direction === "left" ? deltaX < 0 :
        direction === "right" ? deltaX > 0 :
        direction === "up" ? deltaY < 0 : deltaY > 0;

      if (!isInDirection) return;

      const primaryDistance =
        direction === "left" || direction === "right"
          ? Math.abs(deltaX)
          : Math.abs(deltaY);
      const secondaryDistance =
        direction === "left" || direction === "right"
          ? Math.abs(deltaY)
          : Math.abs(deltaX);
      const distance = primaryDistance * 1000 + secondaryDistance;

      if (distance < bestDistance) {
        bestDistance = distance;
        bestIndex = index;
      }
    });

    if (bestIndex < 0) return;

    const reordered = [...terms];
    [reordered[currentIndex], reordered[bestIndex]] = [
      reordered[bestIndex],
      reordered[currentIndex],
    ];
    terms = reordered;
  }

  function handleKeydown(event: KeyboardEvent) {
    if (!event.ctrlKey && !event.metaKey) return;

    const direction = {
      ArrowLeft: "left",
      ArrowRight: "right",
      ArrowUp: "up",
      ArrowDown: "down",
    }[event.key] as "left" | "right" | "up" | "down" | undefined;

    if (!direction) return;

    event.preventDefault();
    moveActive(direction);
  }

  export function spawnTerminal() {
    terms.push({ id: nextId });
    activeId = nextId;
    nextId++;
  }

  export function closeTerminal(id?: number) {
    if (!id) id = activeId
    terms = terms.filter((w) => w.id != id);
    if (activeId === id && terms.length > 0) {
      activeId = terms[terms.length - 1].id;
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="container" {...rest}>
  <main class="workspace">
    {#each terms as win, index (win.id)}
      <div
        class="window"
        class:active={activeId === win.id}
        style={getPaneStyle(index, terms.length)}
        onclick={() => (activeId = win.id)}
        onmouseenter={() => (activeId = win.id)}
        role="button"
        tabindex="0"
        onkeydown={(e) => e.key === "Enter" && (activeId = win.id)}
      >
        <Terminal active={activeId === win.id} />
      </div>
    {:else}
      <div class="empty-state">No active windows. Click "+ Spawn Window".</div>
    {/each}
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
    position: relative;
    flex: 1;
    gap: 10px;
    min-height: 0;
  }

  .window {
    position: absolute;
    box-sizing: border-box;
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
