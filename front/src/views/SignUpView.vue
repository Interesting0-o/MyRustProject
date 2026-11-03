<script setup>
import { onBeforeUnmount, reactive, ref, watch } from 'vue'
import { RouterLink, useRoute, useRouter } from 'vue-router'

import { signUp as signUpApi } from '@/api'
import AuthShell from '@/components/AuthShell.vue'
import BaseAlert from '@/components/BaseAlert.vue'
import PasswordField from '@/components/PasswordField.vue'
import RoleSwitch from '@/components/RoleSwitch.vue'

const route = useRoute()
const router = useRouter()

const role = ref(route.query.role === 'librarian' ? 'librarian' : 'reader')
const form = reactive({
  name: '',
  account: '',
  password: '',
  confirmPassword: '',
})
const submitting = ref(false)
const feedback = ref(null)
let redirectTimer = null

watch(role, () => {
  feedback.value = null
})

onBeforeUnmount(() => {
  if (redirectTimer) {
    clearTimeout(redirectTimer)
  }
})

async function handleSubmit() {
  if (submitting.value) return

  feedback.value = null

  const name = form.name.trim()
  const account = form.account.trim()

  if (!name || !account || !form.password || !form.confirmPassword) {
    feedback.value = { type: 'error', text: '请完整填写姓名、账号和密码' }
    return
  }

  if (form.password !== form.confirmPassword) {
    feedback.value = { type: 'error', text: '两次输入的密码不一致' }
    return
  }

  submitting.value = true
  const result = await signUpApi(role.value, {
    name,
    account,
    password: form.password,
  })

  if (!result.success) {
    submitting.value = false
    feedback.value = { type: 'error', text: result.messages }
    return
  }

  feedback.value = { type: 'success', text: result.messages || '注册成功' }
  redirectTimer = window.setTimeout(() => {
    router.replace({
      name: 'signin',
      query: { role: role.value, account },
    })
  }, 900)
}
</script>

<template>
  <AuthShell>
    <div class="auth-form">
      <header class="auth-form__head">
        <p class="eyebrow">JOIN THE LIBRARY</p>
        <h2>创建账号</h2>
        <p>留下你的信息，开启一段新的阅读旅程。</p>
      </header>

      <RoleSwitch v-model="role" />

      <form class="auth-form__body" @submit.prevent="handleSubmit">
        <div class="field">
          <label class="field__label" for="signup-name">姓名</label>
          <input
            id="signup-name"
            v-model="form.name"
            type="text"
            name="name"
            autocomplete="name"
            placeholder="请输入姓名"
            required
          />
        </div>

        <div class="field">
          <label class="field__label" for="signup-account">账号</label>
          <input
            id="signup-account"
            v-model="form.account"
            type="text"
            name="account"
            autocomplete="username"
            placeholder="设置一个登录账号"
            required
          />
        </div>

        <PasswordField
          v-model="form.password"
          label="密码"
          placeholder="设置登录密码"
          autocomplete="new-password"
        />

        <PasswordField
          v-model="form.confirmPassword"
          label="确认密码"
          placeholder="再次输入密码"
          autocomplete="new-password"
        />

        <BaseAlert :type="feedback?.type" :message="feedback?.text" />

        <button class="primary-button" type="submit" :disabled="submitting">
          <span v-if="submitting" class="spinner" aria-hidden="true"></span>
          {{ submitting ? '正在创建…' : '创建账号' }}
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
          已经有账号？
          <RouterLink :to="{ name: 'signin', query: { role } }">直接登录</RouterLink>
        </span>
      </p>
    </div>
  </AuthShell>
</template>
