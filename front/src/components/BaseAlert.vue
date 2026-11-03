<script setup>
defineProps({
  type: {
    type: String,
    default: 'info',
  },
  message: {
    type: String,
    default: '',
  },
})
</script>

<template>
  <Transition name="alert">
    <div v-if="message" class="alert" :class="`alert--${type}`" role="status" aria-live="polite">
      <svg
        v-if="type === 'success'"
        viewBox="0 0 20 20"
        fill="none"
        width="18"
        height="18"
        aria-hidden="true"
      >
        <circle cx="10" cy="10" r="7.4" stroke="currentColor" stroke-width="1.5" />
        <path
          d="m6.8 10.2 2.2 2.1 4.2-4.4"
          stroke="currentColor"
          stroke-width="1.6"
          stroke-linecap="round"
          stroke-linejoin="round"
        />
      </svg>
      <svg
        v-else-if="type === 'error'"
        viewBox="0 0 20 20"
        fill="none"
        width="18"
        height="18"
        aria-hidden="true"
      >
        <circle cx="10" cy="10" r="7.4" stroke="currentColor" stroke-width="1.5" />
        <path d="M10 6.2v4.6" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" />
        <circle cx="10" cy="13.6" r=".9" fill="currentColor" />
      </svg>
      <svg v-else viewBox="0 0 20 20" fill="none" width="18" height="18" aria-hidden="true">
        <circle cx="10" cy="10" r="7.4" stroke="currentColor" stroke-width="1.5" />
        <path d="M10 9v4.4" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" />
        <circle cx="10" cy="6.3" r=".9" fill="currentColor" />
      </svg>
      <span>{{ message }}</span>
    </div>
  </Transition>
</template>
