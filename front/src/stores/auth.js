import { computed, ref } from 'vue'
import { defineStore } from 'pinia'

import { AUTH_STORAGE_KEY, login as loginApi } from '@/api'

const STORAGE_KEY = AUTH_STORAGE_KEY
const ROLES = ['reader', 'librarian']

function readSession() {
  try {
    const raw = localStorage.getItem(STORAGE_KEY)
    if (!raw) return null

    const session = JSON.parse(raw)
    if (
      !session ||
      typeof session !== 'object' ||
      typeof session.token !== 'string' ||
      typeof session.account !== 'string' ||
      !ROLES.includes(session.role) ||
      !session.token ||
      !session.account
    ) {
      return null
    }

    return session
  } catch {
    return null
  }
}

export const useAuthStore = defineStore('auth', () => {
  const session = readSession()
  const token = ref(session?.token ?? '')
  const account = ref(session?.account ?? '')
  const role = ref(session?.role ?? '')
  const name = ref(typeof session?.name === 'string' ? session.name : '')

  const isLoggedIn = computed(() => Boolean(token.value && account.value && role.value))
  const roleLabel = computed(() => (role.value === 'librarian' ? '馆员' : '读者'))
  const isLibrarian = computed(() => role.value === 'librarian')

  function persist() {
    try {
      if (!isLoggedIn.value) {
        localStorage.removeItem(STORAGE_KEY)
        return
      }

      localStorage.setItem(
        STORAGE_KEY,
        JSON.stringify({
          token: token.value,
          account: account.value,
          role: role.value,
          name: name.value,
        }),
      )
    } catch {
      return
    }
  }

  async function signIn(selectedRole, credentials) {
    const result = await loginApi(selectedRole, credentials)

    if (!result.success || !result.token) {
      return {
        success: false,
        messages: result.messages || '登录失败，请稍后重试',
      }
    }

    token.value = result.token
    account.value = credentials.account
    role.value = selectedRole
    name.value = typeof result.name === 'string' ? result.name : ''
    persist()

    return { success: true, messages: result.messages }
  }

  function setName(value) {
    name.value = value
    persist()
  }

  function signOut() {
    token.value = ''
    account.value = ''
    role.value = ''
    name.value = ''
    persist()
  }

  return {
    token,
    account,
    role,
    name,
    isLoggedIn,
    isLibrarian,
    roleLabel,
    signIn,
    setName,
    signOut,
  }
})
