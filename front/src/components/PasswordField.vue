<script setup>
import { ref, useId } from 'vue'

defineProps({
  modelValue: {
    type: String,
    default: '',
  },
  label: {
    type: String,
    default: '密码',
  },
  placeholder: {
    type: String,
    default: '请输入密码',
  },
  autocomplete: {
    type: String,
    default: 'current-password',
  },
  required: {
    type: Boolean,
    default: true,
  },
  disabled: {
    type: Boolean,
    default: false,
  },
})

const emit = defineEmits(['update:modelValue'])

const id = useId()
const visible = ref(false)
</script>

<template>
  <div class="field">
    <label class="field__label" :for="id">{{ label }}</label>
    <span class="password-control">
      <input
        :id="id"
        :type="visible ? 'text' : 'password'"
        :value="modelValue"
        :placeholder="placeholder"
        :autocomplete="autocomplete"
        :required="required"
        :disabled="disabled"
        @input="emit('update:modelValue', $event.target.value)"
      />
      <button
        type="button"
        class="password-control__toggle"
        :aria-label="visible ? '隐藏密码' : '显示密码'"
        :title="visible ? '隐藏密码' : '显示密码'"
        :disabled="disabled"
        @click="visible = !visible"
      >
        <svg v-if="!visible" viewBox="0 0 20 20" fill="none" aria-hidden="true">
          <path
            d="M2.6 10s2.6-4.4 7.4-4.4S17.4 10 17.4 10s-2.6 4.4-7.4 4.4S2.6 10 2.6 10Z"
            stroke="currentColor"
            stroke-width="1.5"
            stroke-linejoin="round"
          />
          <circle cx="10" cy="10" r="2.1" stroke="currentColor" stroke-width="1.5" />
        </svg>
        <svg v-else viewBox="0 0 20 20" fill="none" aria-hidden="true">
          <path
            d="M3.2 3.2 16.8 16.8"
            stroke="currentColor"
            stroke-width="1.5"
            stroke-linecap="round"
          />
          <path
            d="M8.2 8.4a2.1 2.1 0 0 0 2.9 2.9M5.1 6.2C3.5 7.4 2.6 10 2.6 10s2.6 4.4 7.4 4.4c1.2 0 2.3-.3 3.3-.7M8.4 5.8c.5-.1 1-.2 1.6-.2 4.8 0 7.4 4.4 7.4 4.4s-.7 1.2-1.9 2.3"
            stroke="currentColor"
            stroke-width="1.5"
            stroke-linecap="round"
            stroke-linejoin="round"
          />
        </svg>
      </button>
    </span>
  </div>
</template>
