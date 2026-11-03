<script setup>
defineProps({
  tone: {
    type: String,
    default: 'dark',
  },
})
</script>

<template>
  <span class="logo" :class="`logo--${tone}`">
    <span class="logo__mark" aria-hidden="true">
      <svg viewBox="0 0 32 32" fill="none">
        <path
          d="M6 8.5c4.2-1.7 7.3-1.5 10 .6 2.7-2.1 5.8-2.3 10-.6v15c-4.2-1.7-7.3-1.5-10 .6-2.7-2.1-5.8-2.3-10-.6v-15Z"
          stroke="currentColor"
          stroke-width="1.6"
          stroke-linejoin="round"
        />
        <path d="M16 9.1v15" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" />
        <path
          d="M9.5 13h3.4M9.5 17h3.4M19.1 13h3.4M19.1 17h3.4"
          stroke="currentColor"
          stroke-width="1.4"
          stroke-linecap="round"
          opacity=".7"
        />
      </svg>
    </span>
    <span class="logo__text">
      <strong>书海</strong>
      <small>BOOK MANAGER</small>
    </span>
  </span>
</template>

<style scoped>
.logo {
  display: inline-flex;
  align-items: center;
  gap: 12px;
  color: var(--ink);
}

.logo__mark {
  display: grid;
  place-items: center;
  width: 44px;
  height: 44px;
  border-radius: 14px;
  background: var(--forest);
  color: var(--amber-soft);
  box-shadow:
    inset 0 0 0 1px rgba(255, 255, 255, 0.1),
    0 12px 26px -18px rgba(13, 44, 36, 0.8);
}

.logo__mark svg {
  width: 26px;
  height: 26px;
}

.logo__text {
  display: flex;
  flex-direction: column;
  line-height: 1.05;
}

.logo__text strong {
  font-family: var(--font-serif);
  font-size: 1.18rem;
  font-weight: 600;
  letter-spacing: 0.08em;
}

.logo__text small {
  margin-top: 4px;
  color: var(--ink-muted);
  font-size: 0.56rem;
  letter-spacing: 0.24em;
}

.logo--light {
  color: #f6f0e4;
}

.logo--light .logo__mark {
  background: rgba(246, 240, 228, 0.09);
  color: var(--amber-soft);
  box-shadow: inset 0 0 0 1px rgba(246, 240, 228, 0.18);
}

.logo--light .logo__text small {
  color: rgba(246, 240, 228, 0.46);
}
</style>
