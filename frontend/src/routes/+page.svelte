<script lang="ts">
  import type { PageData } from "./$types";

  // Get todos from page load
  export let data: PageData;
  let todos = data.todos;

  // Edit todo
  let editingTodo: Todo | null = null;

  function startEditing(todo: Todo) {
    editingTodo = todo;
  }

  function saveEdit() {
    if (editingTodo) {
      // Update the local todo without making a server request
      editingTodo = null;
    }
  }

  // Update todo
  async function updateTodo(todo: Todo) {
    if (editingTodo === todo) {
      // Make a server request only if it's the edited todo
      await fetch(`http://0.0.0.0:8000/update?id=${todo.id}&description=${todo.description}&done=${todo.done}`);
      editingTodo = null;
    }
  }
</script>

<div class="container mx-auto mt-16">
  <h1 class="h1 text-center">TODO 'S'</h1>

  <div class="max-w-screen-md mx-auto">
    <form action="http://0.0.0.0:8080/create" method="POST">
      <input
        class="input p-4 my-8"
        name="description"
        type="text"
        placeholder="What needs to be done?"
        autocomplete="off"
      />
    </form>

    <div class="space-y-4">
      {#each todos as todo}
        <div class="flex items-center justify-between p-4 bg-surface-800 rounded-lg gap-4">
          <input
            class="checkbox"
            type="checkbox"
            bind:checked={todo.done}
            on:change={updateTodo(todo)}
          />
          {#if editingTodo === todo}
            <input class="input" type="text" bind:value={todo.description} on:blur={saveEdit} />
            <button class="btn variant-filled-primary" on:click={updateTodo(todo)}>Update</button>
          {:else}
            <input class="input" type="text" bind:value={todo.description} disabled={todo.done} />
            <button class="btn variant-filled-secondary" on:click={() => startEditing(todo)}>Edit</button>
          {/if}
        </div>
      {/each}
    </div>
  </div>
</div>
