<script setup>
import { reactive, ref, watch } from 'vue'
import { RouterLink, useRoute, useRouter } from 'vue-router'

import AuthShell from '@/components/AuthShell.vue'
import BaseAlert from '@/components/BaseAlert.vue'
import PasswordField from '@/components/PasswordField.vue'
import RoleSwitch from '@/components/RoleSwitch.vue'
import { useAuthStore } from '@/stores/auth'

const route = useRoute()
const router = useRouter()
const auth = useAuthStore()

const role = ref(route.query.role === 'librarian' ? 'librarian' : 'reader')
const form = reactive({
  account: typeof route.query.account === 'string' ? route.query.account : '',
  password: '',
})
const submitting = ref(false)
const feedback = ref(null)

watch(role, () => {
  feedback.value = null
})

function resolveRedirect() {
  const redirect = route.query.redirect
  if (typeof redirect === 'string' && redirect.startsWith('/') && !redirect.startsWith('//')) {
    return redirect
  }

  return '/app'
}

async function handleSubmit() {
  if (submitting.value) return

  feedback.value = null

  const account = form.account.trim()

  if (!account || !form.password) {
    feedback.value = { type: 'error', text: '请输入账号和密码' }
    return
  }

  submitting.value = true
  const result = await auth.signIn(role.value, {
    account,
    password: form.password,
  })

  if (!result.success) {
    submitting.value = false
    feedback.value = { type: 'error', text: result.messages }
    return
  }

  await router.replace(resolveRedirect())
}
</script>

<template>
  <AuthShell>
    <div class="auth-form">
      <header class="auth-form__head">
        <p class="eyebrow">WELCOME BACK</p>
        <h2>欢迎回来</h2>
        <p>选择你的身份，继续今天的借阅工作。</p>
      </header>

      <RoleSwitch v-model="role" />

      <form class="auth-form__body" @submit.prevent="handleSubmit">
        <div class="field">
          <label class="field__label" for="signin-account">账号</label>
          <input
            id="signin-account"
            v-model="form.account"
            type="text"
            name="account"
            autocomplete="username"
            placeholder="请输入账号"
            required
          />
        </div>

        <PasswordField
          v-model="form.password"
          label="密码"
          placeholder="请输入登录密码"
          autocomplete="current-password"
        />

        <BaseAlert :type="feedback?.type" :message="feedback?.text" />

        <button class="primary-button" type="submit" :disabled="submitting">
          <span v-if="submitting" class="spinner" aria-hidden="true"></span>
          {{ submitting ? '正在登录…' : '登录' }}
          <svg
            v-if="!submitting"
            class="primary-button__arrow"
            viewBox="0 0 20 20"
            fill="none"
            aria-hidden="true"
          >
            <path
              d="M4 10h11.4M11.6 6.2 15.4 10l-3.8 3.8"
              stroke="currentColor"
              stroke-width="1.6"
              stroke-linecap="round"
              stroke-linejoin="round"
            />
          </svg>
        </button>
      </form>

      <p class="auth-form__footer">
        <span class="auth-form__row">
          还没有账号？
          <RouterLink :to="{ name: 'signup', query: { role } }">创建账号</RouterLink>
        </span>
      </p>
    </div>
  </AuthShell>
</template>
