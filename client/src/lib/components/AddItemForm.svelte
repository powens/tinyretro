<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { Textarea } from "$lib/components/ui/textarea";

  let {
    laneId,
    onAddItem,
  }: {
    laneId: string;
    onAddItem: (laneId: string, content: string) => void;
  } = $props();

  function handleSubmit(e: SubmitEvent) {
    e.preventDefault();
    const form = e.target as HTMLFormElement;
    const formData = new FormData(form);
    const content = formData.get("content") as string;
    if (content && content.trim()) {
      onAddItem(laneId, content);
      form.reset();
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      const textarea = e.target as HTMLTextAreaElement;
      if (textarea.form) {
        textarea.form.requestSubmit();
      }
    }
  }
</script>

<div
  class="border-2 border-dashed border-gray-300 dark:border-gray-600 rounded-lg p-3"
>
  <form onsubmit={handleSubmit}>
    <Textarea
      name="content"
      placeholder="Add a new item..."
      class="border-0 bg-transparent resize-none focus-visible:ring-0 p-2"
      rows={2}
      onkeydown={handleKeydown}
    />
    <div class="flex justify-end mt-2">
      <Button type="submit" size="sm">Add Item</Button>
    </div>
  </form>
</div>
