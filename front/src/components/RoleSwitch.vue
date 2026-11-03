<script setup>
defineProps({
  modelValue: {
    type: String,
    default: 'reader',
  },
})

defineEmits(['update:modelValue'])
</script>

<template>
  <div class="role-switch" role="tablist" aria-label="选择身份">
    <span
      class="role-switch__thumb"
      :class="{ 'role-switch__thumb--librarian': modelValue === 'librarian' }"
      aria-hidden="true"
    ></span>
    <button
      type="button"
      role="tab"
      class="role-switch__option"
      :class="{ 'is-active': modelValue === 'reader' }"
      :aria-selected="modelValue === 'reader'"
      @click="$emit('update:modelValue', 'reader')"
    >
      <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
        <circle cx="10" cy="6.8" r="3.1" stroke="currentColor" stroke-width="1.5" />
        <path
          d="M4.4 16.2c.5-2.9 2.8-4.6 5.6-4.6s5.1 1.7 5.6 4.6"
          stroke="currentColor"
          stroke-width="1.5"
          stroke-linecap="round"
        />
      </svg>
      读者
    </button>
    <button
      type="button"
      role="tab"
      class="role-switch__option"
      :class="{ 'is-active': modelValue === 'librarian' }"
      :aria-selected="modelValue === 'librarian'"
      @click="$emit('update:modelValue', 'librarian')"
    >
      <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
        <path
          d="M3.5 4.6c2.6-1 4.6-.9 6.5.4 1.9-1.3 3.9-1.4 6.5-.4v10.9c-2.6-1-4.6-.9-6.5.4-1.9-1.3-3.9-1.4-6.5-.4V4.6Z"
          stroke="currentColor"
          stroke-width="1.5"
          stroke-linejoin="round"
        />
        <path d="M10 5v10.9" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
      </svg>
      馆员
    </button>
  </div>
</template>

<style scoped>
.role-switch {
  position: relative;
  display: grid;
  grid-template-columns: 1fr 1fr;
  padding: 5px;
  border: 1px solid var(--line);
  border-radius: 999px;
  background: rgba(20, 61, 50, 0.055);
}

.role-switch__thumb {
  position: absolute;
  top: 5px;
  bottom: 5px;
  left: 5px;
  width: calc(50% - 5px);
  border-radius: 999px;
  background: var(--surface);
  box-shadow: 0 5px 16px -8px rgba(13, 44, 36, 0.36);
  transition: transform 0.32s cubic-bezier(0.2, 0.7, 0.2, 1);
}

.role-switch__thumb--librarian {
  transform: translateX(100%);
}

.role-switch__option {
  position: relative;
  z-index: 1;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  height: 40px;
  padding: 0;
  border: 0;
  border-radius: 999px;
  background: transparent;
  color: var(--ink-muted);
  font: inherit;
  font-size: 0.88rem;
  font-weight: 600;
  cursor: pointer;
  transition: color 0.2s ease;
}

.role-switch__option.is-active {
  color: var(--forest);
}

.role-switch__option svg {
  width: 17px;
  height: 17px;
}
</style>
