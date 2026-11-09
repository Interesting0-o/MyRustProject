<script setup>
import { computed, nextTick, onBeforeUnmount, onMounted, reactive, ref } from 'vue'
import { useRouter } from 'vue-router'

import {
  borrowBook,
  findBookByBid,
  findBookByName,
  returnBook,
  updateBookName,
  updateBookNum,
  updateBookPrice,
  updateReader as updateReaderApi,
} from '@/api'
import AppLogo from '@/components/AppLogo.vue'
import BaseAlert from '@/components/BaseAlert.vue'
import PasswordField from '@/components/PasswordField.vue'
import { useActivityStore } from '@/stores/activity'
import { useAuthStore } from '@/stores/auth'

const router = useRouter()
const auth = useAuthStore()
const activity = useActivityStore()

const borrowForm = reactive({ bid: '' })
const returnForm = reactive({ bid: '' })
const feedback = reactive({ borrow: null, return: null })
const pending = reactive({ borrow: false, return: false })

const searchMode = ref('name')
const searchQuery = ref('')
const searchResults = ref([])
const searchMessage = ref('')
const searching = ref(false)
const hasSearched = ref(false)
const borrowCardRef = ref(null)
const borrowBidInput = ref(null)
const editNameInput = ref(null)

const editOpen = ref(false)
const editPending = ref(false)
const editFeedback = ref(null)
const editOriginal = ref(null)
const editForm = reactive({ bid: '', name: '', price: '', num: '' })

const profileOpen = ref(false)
const profilePending = ref(false)
const profileFeedback = ref(null)
const profileNameInput = ref(null)
const profileForm = reactive({
  name: '',
  currentPassword: '',
  newPassword: '',
  confirmPassword: '',
})

const isLibrarian = computed(() => auth.isLibrarian)
const initials = computed(() => auth.account.slice(0, 2).toUpperCase())

const greeting = computed(() => {
  const hour = new Date().getHours()
  const period =
    hour < 6
      ? '夜深了'
      : hour < 12
        ? '早上好'
        : hour < 14
          ? '中午好'
          : hour < 18
            ? '下午好'
            : '晚上好'

  return `${period}，${auth.account}`
})

const heroDescription = computed(() =>
  isLibrarian.value
    ? '查询馆藏、维护图书信息，借阅与归还由读者本人办理。'
    : '管理你的借阅进度，输入图书编号即可借阅新书或归还已读书籍。',
)

const heroTip = computed(() =>
  isLibrarian.value
    ? '可先按书名或编号查询馆藏，在搜索结果中维护图书信息。'
    : '可先在馆藏查询中确认图书编号与库存，再办理借阅或归还。',
)

const searchPlaceholder = computed(() =>
  searchMode.value === 'name' ? '输入书名，例如 红楼梦' : '输入图书编号，例如 B001',
)

const timeFormatter = new Intl.DateTimeFormat('zh-CN', {
  month: '2-digit',
  day: '2-digit',
  hour: '2-digit',
  minute: '2-digit',
  hour12: false,
})

function formatTime(value) {
  return timeFormatter.format(new Date(value))
}

function formatPrice(cents) {
  return `¥${(cents / 100).toFixed(2)}`
}

function switchSearchMode(mode) {
  if (searchMode.value === mode) return

  searchMode.value = mode
  searchResults.value = []
  searchMessage.value = ''
  hasSearched.value = false
}

async function handleSearch() {
  if (searching.value) return

  const query = searchQuery.value.trim()
  searchResults.value = []
  searchMessage.value = ''
  hasSearched.value = true

  if (!query) {
    searchMessage.value = searchMode.value === 'name' ? '请输入要查询的书名' : '请输入图书编号'
    return
  }

  searching.value = true
  const result =
    searchMode.value === 'name' ? await findBookByName(query) : await findBookByBid(query)
  searching.value = false

  searchResults.value = Array.isArray(result.books) ? result.books : []
  searchMessage.value = result.messages || ''

  if (!result.success) {
    searchMessage.value = result.messages || '查询失败，请稍后重试'
  }
}

function useInBorrowForm(bid) {
  borrowForm.bid = bid
  feedback.borrow = null
  borrowBidInput.value?.focus({ preventScroll: true })
  borrowCardRef.value?.scrollIntoView({ behavior: 'smooth', block: 'center' })
}

function updateSearchStock(bid, delta) {
  const book = searchResults.value.find((item) => item.bid === bid)
  if (book) {
    book.num = Math.max(0, book.num + delta)
  }
}

function parsePriceToCents(value) {
  const text = String(value).trim()

  if (!/^\d+(\.\d{1,2})?$/.test(text)) {
    return null
  }

  const [yuan, fraction = ''] = text.split('.')
  const cents = Number(yuan) * 100 + Number(fraction.padEnd(2, '0'))

  return Number.isSafeInteger(cents) ? cents : null
}

function openBookEditor(book) {
  if (!isLibrarian.value) return

  editForm.bid = book.bid
  editForm.name = book.name
  editForm.price = (book.price / 100).toFixed(2)
  editForm.num = String(book.num)
  editOriginal.value = { ...book }
  editFeedback.value = null
  editOpen.value = true

  nextTick(() => editNameInput.value?.focus())
}

function closeBookEditor() {
  if (editPending.value) return

  editOpen.value = false
}

function handleEditorKeydown(event) {
  if (event.key !== 'Escape') return

  if (profileOpen.value) {
    closeReaderEditor()
    return
  }

  if (editOpen.value) {
    closeBookEditor()
  }
}

onMounted(() => window.addEventListener('keydown', handleEditorKeydown))
onBeforeUnmount(() => window.removeEventListener('keydown', handleEditorKeydown))

async function submitBookEdit() {
  if (!isLibrarian.value || editPending.value || !editOriginal.value) return

  const original = editOriginal.value
  const bid = editForm.bid
  const name = editForm.name.trim()
  const price = parsePriceToCents(editForm.price)
  const numText = String(editForm.num).trim()

  if (!name) {
    editFeedback.value = { type: 'error', text: '请输入书名' }
    return
  }

  if (price === null) {
    editFeedback.value = { type: 'error', text: '请输入有效价格，最多两位小数' }
    return
  }

  if (
    !/^\d+$/.test(numText) ||
    !Number.isSafeInteger(Number(numText)) ||
    Number(numText) > 4294967295
  ) {
    editFeedback.value = { type: 'error', text: '请输入有效的库存数量' }
    return
  }

  const num = Number(numText)
  const updates = []

  if (name !== original.name) {
    updates.push({
      label: '书名',
      patch: { name },
      request: () => updateBookName(bid, name),
    })
  }

  if (price !== original.price) {
    updates.push({
      label: '价格',
      patch: { price },
      request: () => updateBookPrice(bid, price),
    })
  }

  if (num !== original.num) {
    updates.push({
      label: '库存',
      patch: { num },
      request: () => updateBookNum(bid, num),
    })
  }

  if (!updates.length) {
    editFeedback.value = { type: 'info', text: '内容没有变化' }
    return
  }

  editPending.value = true
  editFeedback.value = null

  const results = []

  for (const update of updates) {
    results.push({ ...update, result: await update.request() })
  }

  editPending.value = false

  const succeeded = results.filter((item) => item.result.success)
  const failed = results.filter((item) => !item.result.success)

  for (const { patch } of succeeded) {
    Object.assign(original, patch)

    const book = searchResults.value.find((item) => item.bid === bid)
    if (book) {
      Object.assign(book, patch)
    }
  }

  if (failed.length) {
    const detail = failed
      .map((item) => `${item.label}：${item.result.messages || '更新失败'}`)
      .join('；')

    editFeedback.value = { type: 'error', text: detail }
  } else {
    editFeedback.value = { type: 'success', text: '图书信息已更新' }
  }

  const message = failed.length
    ? `部分更新失败：${failed.map((item) => `${item.label}更新失败`).join('，')}`
    : '图书信息更新成功'

  activity.record({
    action: '维护',
    bid,
    success: failed.length === 0,
    message,
  })
}

function openReaderEditor() {
  if (isLibrarian.value) return

  profileForm.name = auth.name || ''
  profileForm.currentPassword = ''
  profileForm.newPassword = ''
  profileForm.confirmPassword = ''
  profileFeedback.value = null
  profileOpen.value = true

  nextTick(() => profileNameInput.value?.focus())
}

function closeReaderEditor() {
  if (profilePending.value) return

  profileOpen.value = false
}

async function submitReaderProfile() {
  if (isLibrarian.value || profilePending.value) return

  const name = profileForm.name.trim()
  const currentPassword = profileForm.currentPassword
  const newPassword = profileForm.newPassword
  const confirmPassword = profileForm.confirmPassword
  const wantsPasswordChange = Boolean(newPassword || confirmPassword)

  profileFeedback.value = null

  if (!name) {
    profileFeedback.value = { type: 'error', text: '请输入姓名' }
    return
  }

  if (wantsPasswordChange) {
    if (!currentPassword) {
      profileFeedback.value = { type: 'error', text: '请输入当前密码' }
      return
    }

    if (!newPassword) {
      profileFeedback.value = { type: 'error', text: '请输入新密码' }
      return
    }

    if (newPassword !== confirmPassword) {
      profileFeedback.value = { type: 'error', text: '两次输入的新密码不一致' }
      return
    }
  }

  const nameChanged = name !== (auth.name || '')

  if (!nameChanged && !wantsPasswordChange) {
    profileFeedback.value = { type: 'info', text: '内容没有变化' }
    return
  }

  profilePending.value = true
  const result = await updateReaderApi({
    name: nameChanged ? name : undefined,
    currentPassword: wantsPasswordChange ? currentPassword : undefined,
    newPassword: wantsPasswordChange ? newPassword : undefined,
  })
  profilePending.value = false

  if (!result.success) {
    const message = result.messages || '更新失败，请稍后重试'
    profileFeedback.value = { type: 'error', text: message }
    activity.record({ action: '资料', success: false, message })
    return
  }

  if (nameChanged) {
    auth.setName(name)
  }

  profileForm.currentPassword = ''
  profileForm.newPassword = ''
  profileForm.confirmPassword = ''
  profileFeedback.value = { type: 'success', text: result.messages || '个人资料已更新' }
  activity.record({ action: '资料', success: true, message: '个人资料更新成功' })
}

async function submitBorrow() {
  if (pending.borrow) return

  feedback.borrow = null

  const bid = borrowForm.bid.trim()

  if (!bid) {
    feedback.borrow = { type: 'error', text: '请输入图书编号' }
    return
  }

  pending.borrow = true
  const result = await borrowBook(bid)
  pending.borrow = false

  const message = result.messages || (result.success ? '借书成功' : '借书失败，请稍后重试')
  feedback.borrow = { type: result.success ? 'success' : 'error', text: message }
  activity.record({
    action: '借阅',
    bid,
    success: result.success,
    message,
  })

  if (result.success) {
    borrowForm.bid = ''
    updateSearchStock(bid, -1)
  }
}

async function submitReturn() {
  if (pending.return) return

  feedback.return = null

  const bid = returnForm.bid.trim()

  if (!bid) {
    feedback.return = { type: 'error', text: '请输入图书编号' }
    return
  }

  pending.return = true
  const result = await returnBook(bid)
  pending.return = false

  const message = result.messages || (result.success ? '还书成功' : '还书失败，请稍后重试')
  feedback.return = { type: result.success ? 'success' : 'error', text: message }
  activity.record({
    action: '归还',
    bid,
    success: result.success,
    message,
  })

  if (result.success) {
    returnForm.bid = ''
    updateSearchStock(bid, 1)
  }
}

function handleSignOut() {
  activity.clear()
  auth.signOut()
  router.replace({ name: 'signin' })
}
</script>

<template>
  <div class="dashboard-page">
    <header class="topbar">
      <AppLogo />

      <div class="topbar__meta">
        <span class="role-chip">{{ auth.roleLabel }}</span>
        <span class="topbar__account">{{ auth.account }}</span>
        <button class="ghost-button" type="button" @click="handleSignOut">
          <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
            <path
              d="M8.2 4.2H5.5a1.6 1.6 0 0 0-1.6 1.6v8.4a1.6 1.6 0 0 0 1.6 1.6h2.7M12.3 6.6 15.7 10l-3.4 3.4M7.8 10h7.6"
              stroke="currentColor"
              stroke-width="1.5"
              stroke-linecap="round"
              stroke-linejoin="round"
            />
          </svg>
          退出
        </button>
      </div>
    </header>

    <main class="dashboard">
      <section class="hero">
        <div class="hero__copy">
          <p class="eyebrow">{{ isLibrarian ? 'LIBRARIAN DESK' : 'READER DESK' }}</p>
          <h1>{{ greeting }}<span class="hero__dot">.</span></h1>
          <p class="hero__sub">{{ heroDescription }}</p>
        </div>

        <div class="hero__note">
          <span>今日提示</span>
          <p>{{ heroTip }}</p>
        </div>
      </section>

      <section class="catalog-card">
        <header class="catalog-card__head">
          <div>
            <p class="eyebrow">CATALOG SEARCH</p>
            <h2>馆藏查询</h2>
            <p>
              {{
                isLibrarian
                  ? '按完整书名或图书编号查找馆藏，并在结果中维护图书信息。'
                  : '按完整书名或图书编号查找馆藏，确认库存后再办理借阅。'
              }}
            </p>
          </div>

          <div class="search-mode" role="tablist" aria-label="查询方式">
            <button
              type="button"
              role="tab"
              :class="{ 'is-active': searchMode === 'name' }"
              :aria-selected="searchMode === 'name'"
              :disabled="searching"
              @click="switchSearchMode('name')"
            >
              按书名
            </button>
            <button
              type="button"
              role="tab"
              :class="{ 'is-active': searchMode === 'bid' }"
              :aria-selected="searchMode === 'bid'"
              :disabled="searching"
              @click="switchSearchMode('bid')"
            >
              按编号
            </button>
          </div>
        </header>

        <form class="catalog-search" @submit.prevent="handleSearch">
          <div class="catalog-search__field">
            <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
              <circle cx="8.8" cy="8.8" r="5.1" stroke="currentColor" stroke-width="1.6" />
              <path
                d="m12.8 12.8 3.3 3.3"
                stroke="currentColor"
                stroke-width="1.6"
                stroke-linecap="round"
              />
            </svg>
            <input
              v-model="searchQuery"
              type="search"
              :placeholder="searchPlaceholder"
              :aria-label="searchMode === 'name' ? '按书名搜索图书' : '按编号搜索图书'"
              enterkeyhint="search"
            />
          </div>
          <button class="primary-button catalog-search__button" type="submit" :disabled="searching">
            <span v-if="searching" class="spinner" aria-hidden="true"></span>
            {{ searching ? '查询中…' : '查询馆藏' }}
          </button>
        </form>

        <div v-if="searchResults.length" class="catalog-results">
          <p class="catalog-results__meta">{{ searchResults.length }} 本相关图书</p>
          <div class="book-grid">
            <article v-for="book in searchResults" :key="book.bid" class="book-card">
              <div class="book-card__top">
                <span class="book-card__bid">{{ book.bid }}</span>
                <span class="book-card__stock" :class="{ 'is-empty': book.num === 0 }">
                  {{ book.num > 0 ? `可借 ${book.num} 本` : '暂时借完' }}
                </span>
              </div>
              <h3>{{ book.name }}</h3>
              <p class="book-card__author">{{ book.author }}</p>
              <div class="book-card__foot">
                <span class="book-card__price">{{ formatPrice(book.price) }}</span>
                <div class="book-card__actions">
                  <button
                    v-if="isLibrarian"
                    type="button"
                    class="book-card__action book-card__action--edit"
                    :aria-label="`编辑《${book.name}》`"
                    @click="openBookEditor(book)"
                  >
                    编辑
                  </button>
                  <button
                    v-if="!isLibrarian"
                    type="button"
                    class="book-card__action"
                    :disabled="book.num === 0"
                    @click="useInBorrowForm(book.bid)"
                  >
                    {{ book.num === 0 ? '暂无可借' : '填入借阅' }}
                  </button>
                </div>
              </div>
            </article>
          </div>
        </div>

        <div v-else class="catalog-empty">
          <svg viewBox="0 0 24 24" fill="none" aria-hidden="true">
            <circle cx="10.5" cy="10.5" r="6" stroke="currentColor" stroke-width="1.4" />
            <path
              d="m15 15 4.5 4.5"
              stroke="currentColor"
              stroke-width="1.4"
              stroke-linecap="round"
            />
          </svg>
          <p>
            {{
              searching
                ? '正在查询馆藏…'
                : hasSearched
                  ? searchMessage || '没有找到相关图书'
                  : '输入书名或编号开始查询'
            }}
          </p>
          <small v-if="!hasSearched">例如「红楼梦」或「B001」</small>
        </div>
      </section>

      <section class="workspace">
        <div v-if="!isLibrarian" class="action-grid">
          <form
            ref="borrowCardRef"
            class="action-card action-card--borrow"
            @submit.prevent="submitBorrow"
          >
            <div class="action-card__head">
              <span class="action-card__icon">
                <svg viewBox="0 0 24 24" fill="none" aria-hidden="true">
                  <path
                    d="M3.8 5.9c3.4-1.3 5.9-1.1 8.2.5 2.3-1.6 4.8-1.8 8.2-.5v12.3c-3.4-1.3-5.9-1.1-8.2.5-2.3-1.6-4.8-1.8-8.2-.5V5.9Z"
                    stroke="currentColor"
                    stroke-width="1.6"
                    stroke-linejoin="round"
                  />
                  <path
                    d="M12 6.4v12.3"
                    stroke="currentColor"
                    stroke-width="1.6"
                    stroke-linecap="round"
                  />
                  <path
                    d="M17.8 4.4v4.1M15.7 6.4h4.1"
                    stroke="currentColor"
                    stroke-width="1.6"
                    stroke-linecap="round"
                  />
                </svg>
              </span>
              <div>
                <h2>借阅图书</h2>
                <p>{{ isLibrarian ? '为读者登记新的借阅记录' : '找到想读的书，把它带回家' }}</p>
              </div>
            </div>

            <div class="account-note">
              <span class="account-note__dot"></span>
              以 <strong>{{ auth.account }}</strong> 的身份办理
            </div>

            <label class="field">
              <span class="field__label">图书编号</span>
              <input
                ref="borrowBidInput"
                v-model="borrowForm.bid"
                type="text"
                name="borrow-bid"
                autocomplete="off"
                placeholder="例如 B001"
              />
            </label>

            <BaseAlert :type="feedback.borrow?.type" :message="feedback.borrow?.text" />

            <button class="primary-button" type="submit" :disabled="pending.borrow">
              <span v-if="pending.borrow" class="spinner" aria-hidden="true"></span>
              {{ pending.borrow ? '正在办理…' : '确认借阅' }}
              <svg
                v-if="!pending.borrow"
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

          <form class="action-card action-card--return" @submit.prevent="submitReturn">
            <div class="action-card__head">
              <span class="action-card__icon">
                <svg viewBox="0 0 24 24" fill="none" aria-hidden="true">
                  <path
                    d="M5.1 8.2A7.8 7.8 0 1 1 4.4 12"
                    stroke="currentColor"
                    stroke-width="1.7"
                    stroke-linecap="round"
                  />
                  <path
                    d="M4.9 4.3v4.4h4.4"
                    stroke="currentColor"
                    stroke-width="1.7"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                  />
                  <path
                    d="M12 8.2V12l2.6 1.6"
                    stroke="currentColor"
                    stroke-width="1.7"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                  />
                </svg>
              </span>
              <div>
                <h2>归还图书</h2>
                <p>{{ isLibrarian ? '为读者完成归还并释放馆藏' : '读完的书，让它回到书架' }}</p>
              </div>
            </div>

            <div class="account-note">
              <span class="account-note__dot"></span>
              以 <strong>{{ auth.account }}</strong> 的身份办理
            </div>

            <label class="field">
              <span class="field__label">图书编号</span>
              <input
                v-model="returnForm.bid"
                type="text"
                name="return-bid"
                autocomplete="off"
                placeholder="例如 B001"
              />
            </label>

            <BaseAlert :type="feedback.return?.type" :message="feedback.return?.text" />

            <button class="primary-button" type="submit" :disabled="pending.return">
              <span v-if="pending.return" class="spinner" aria-hidden="true"></span>
              {{ pending.return ? '正在办理…' : '确认归还' }}
              <svg
                v-if="!pending.return"
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
        </div>
        <div v-else class="action-card">
          <div class="action-card__head">
            <div>
              <h2>借阅与归还</h2>
              <p>借阅与归还由读者使用本人账号登录后办理。</p>
            </div>
          </div>
        </div>

        <aside class="side-column">
          <section class="account-card">
            <div class="account-card__top">
              <span class="account-card__avatar">{{ initials }}</span>
              <div class="account-card__info">
                <span>{{ auth.roleLabel }}账号</span>
                <strong>{{ auth.account }}</strong>
                <small v-if="auth.name" class="account-card__name">{{ auth.name }}</small>
              </div>
              <span class="account-card__status">在线</span>
            </div>

            <dl class="account-card__meta">
              <div>
                <dt>身份</dt>
                <dd>{{ isLibrarian ? '图书馆馆员' : '读者' }}</dd>
              </div>
              <div>
                <dt>本次会话</dt>
                <dd>{{ activity.entries.length }} 条记录</dd>
              </div>
            </dl>

            <button
              v-if="!isLibrarian"
              class="account-card__edit"
              type="button"
              @click="openReaderEditor"
            >
              <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
                <path
                  d="m4.4 13.7-.7 3 3-.7 8.1-8.1a1.6 1.6 0 0 0 0-2.3l-.4-.4a1.6 1.6 0 0 0-2.3 0l-7.7 8.5Z"
                  stroke="currentColor"
                  stroke-width="1.4"
                  stroke-linejoin="round"
                />
              </svg>
              编辑个人资料
            </button>
          </section>

          <section class="activity-card">
            <header class="activity-card__head">
              <div>
                <p class="eyebrow">SESSION LOG</p>
                <h2>操作记录</h2>
              </div>
              <button
                v-if="activity.entries.length"
                class="text-button"
                type="button"
                @click="activity.clear()"
              >
                清空
              </button>
            </header>

            <ol v-if="activity.entries.length" class="activity-list">
              <li
                v-for="entry in activity.entries"
                :key="entry.id"
                :class="entry.success ? 'is-success' : 'is-error'"
              >
                <span class="activity-list__dot" aria-hidden="true"></span>
                <div class="activity-list__body">
                  <div class="activity-list__title">
                    <strong>{{
                      entry.bid ? `${entry.action} · ${entry.bid}` : entry.action
                    }}</strong>
                    <time :datetime="entry.at">{{ formatTime(entry.at) }}</time>
                  </div>
                  <p>{{ entry.message }}</p>
                </div>
              </li>
            </ol>

            <div v-else class="activity-empty">
              <svg viewBox="0 0 24 24" fill="none" aria-hidden="true">
                <path
                  d="M4 6.3c3.4-1.3 5.9-1.1 8 .5 2.1-1.6 4.6-1.8 8-.5v12.1c-3.4-1.3-5.9-1.1-8 .5-2.1-1.6-4.6-1.8-8-.5V6.3Z"
                  stroke="currentColor"
                  stroke-width="1.4"
                  stroke-linejoin="round"
                />
                <path
                  d="M12 6.8v12.1"
                  stroke="currentColor"
                  stroke-width="1.4"
                  stroke-linecap="round"
                />
              </svg>
              <p>还没有操作记录</p>
              <small>办理借阅、归还或维护馆藏后，进度会显示在这里。</small>
            </div>
          </section>
        </aside>
      </section>
    </main>

    <Transition name="editor">
      <div v-if="editOpen" class="editor-overlay" @click.self="closeBookEditor">
        <section
          class="editor-card"
          role="dialog"
          aria-modal="true"
          aria-labelledby="book-editor-title"
        >
          <header class="editor-card__head">
            <div>
              <p class="eyebrow">BOOK MAINTENANCE</p>
              <h2 id="book-editor-title">编辑图书信息</h2>
              <p>仅馆员可修改馆藏信息，保存后立即生效。</p>
            </div>
            <button
              class="editor-card__close"
              type="button"
              aria-label="关闭编辑窗口"
              :disabled="editPending"
              @click="closeBookEditor"
            >
              <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
                <path
                  d="m5.5 5.5 9 9m0-9-9 9"
                  stroke="currentColor"
                  stroke-width="1.6"
                  stroke-linecap="round"
                />
              </svg>
            </button>
          </header>

          <div class="editor-card__meta">
            <span>{{ editForm.bid }}</span>
            <span v-if="editOriginal">作者 {{ editOriginal.author }}</span>
          </div>

          <form class="editor-form" @submit.prevent="submitBookEdit">
            <label class="field">
              <span class="field__label">书名</span>
              <input
                ref="editNameInput"
                v-model="editForm.name"
                type="text"
                name="edit-book-name"
                autocomplete="off"
                :disabled="editPending"
                placeholder="请输入书名"
              />
            </label>

            <div class="editor-form__grid">
              <label class="field">
                <span class="field__label">售价（元）</span>
                <input
                  v-model="editForm.price"
                  type="text"
                  inputmode="decimal"
                  name="edit-book-price"
                  autocomplete="off"
                  :disabled="editPending"
                  placeholder="例如 39.90"
                />
              </label>

              <label class="field">
                <span class="field__label">库存数量</span>
                <input
                  v-model="editForm.num"
                  type="number"
                  min="0"
                  step="1"
                  name="edit-book-num"
                  :disabled="editPending"
                  placeholder="例如 5"
                />
              </label>
            </div>

            <BaseAlert :type="editFeedback?.type" :message="editFeedback?.text" />

            <div class="editor-form__actions">
              <button
                class="secondary-button"
                type="button"
                :disabled="editPending"
                @click="closeBookEditor"
              >
                取消
              </button>
              <button class="primary-button" type="submit" :disabled="editPending">
                <span v-if="editPending" class="spinner" aria-hidden="true"></span>
                {{ editPending ? '保存中…' : '保存修改' }}
              </button>
            </div>
          </form>
        </section>
      </div>
    </Transition>

    <Transition name="editor">
      <div v-if="profileOpen" class="editor-overlay" @click.self="closeReaderEditor">
        <section
          class="editor-card"
          role="dialog"
          aria-modal="true"
          aria-labelledby="reader-editor-title"
        >
          <header class="editor-card__head">
            <div>
              <p class="eyebrow">READER PROFILE</p>
              <h2 id="reader-editor-title">编辑个人资料</h2>
              <p>更新姓名或密码，账号作为登录标识不可修改。</p>
            </div>
            <button
              class="editor-card__close"
              type="button"
              aria-label="关闭编辑窗口"
              :disabled="profilePending"
              @click="closeReaderEditor"
            >
              <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
                <path
                  d="m5.5 5.5 9 9m0-9-9 9"
                  stroke="currentColor"
                  stroke-width="1.6"
                  stroke-linecap="round"
                />
              </svg>
            </button>
          </header>

          <div class="editor-card__meta">
            <span>{{ auth.account }}</span>
            <span>{{ auth.roleLabel }}</span>
          </div>

          <form class="editor-form" @submit.prevent="submitReaderProfile">
            <label class="field">
              <span class="field__label">姓名</span>
              <input
                ref="profileNameInput"
                v-model="profileForm.name"
                type="text"
                name="profile-name"
                autocomplete="name"
                :disabled="profilePending"
                placeholder="请输入姓名"
              />
            </label>

            <div class="profile-form__divider">
              <span>修改密码</span>
              <small>不修改密码可留空</small>
            </div>

            <PasswordField
              v-model="profileForm.currentPassword"
              label="当前密码"
              placeholder="修改密码时填写"
              autocomplete="current-password"
              :required="false"
              :disabled="profilePending"
            />

            <div class="editor-form__grid">
              <PasswordField
                v-model="profileForm.newPassword"
                label="新密码"
                placeholder="输入新密码"
                autocomplete="new-password"
                :required="false"
                :disabled="profilePending"
              />

              <PasswordField
                v-model="profileForm.confirmPassword"
                label="确认新密码"
                placeholder="再次输入新密码"
                autocomplete="new-password"
                :required="false"
                :disabled="profilePending"
              />
            </div>

            <BaseAlert :type="profileFeedback?.type" :message="profileFeedback?.text" />

            <div class="editor-form__actions">
              <button
                class="secondary-button"
                type="button"
                :disabled="profilePending"
                @click="closeReaderEditor"
              >
                取消
              </button>
              <button class="primary-button" type="submit" :disabled="profilePending">
                <span v-if="profilePending" class="spinner" aria-hidden="true"></span>
                {{ profilePending ? '保存中…' : '保存修改' }}
              </button>
            </div>
          </form>
        </section>
      </div>
    </Transition>
  </div>
</template>

<style scoped>
.dashboard-page {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
}

.topbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 20px;
  width: min(1180px, calc(100% - 40px));
  margin: 0 auto;
  padding: 26px 0 18px;
}

.topbar__meta {
  display: flex;
  align-items: center;
  gap: 14px;
}

.role-chip {
  display: inline-flex;
  align-items: center;
  gap: 7px;
  padding: 6px 12px;
  border: 1px solid var(--line);
  border-radius: 999px;
  background: rgba(53, 107, 88, 0.08);
  color: var(--forest);
  font-size: 0.75rem;
  font-weight: 600;
  letter-spacing: 0.04em;
}

.role-chip::before {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: var(--amber);
  box-shadow: 0 0 0 3px rgba(185, 119, 45, 0.15);
  content: '';
}

.topbar__account {
  color: var(--ink-soft);
  font-size: 0.85rem;
  font-weight: 600;
}

.ghost-button {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  height: 40px;
  padding: 0 16px;
  border: 1px solid var(--line-strong);
  border-radius: 999px;
  background: rgba(255, 253, 248, 0.66);
  color: var(--ink-soft);
  font-size: 0.82rem;
  font-weight: 600;
  transition:
    color 0.2s ease,
    border-color 0.2s ease,
    background-color 0.2s ease;
}

.ghost-button svg {
  width: 17px;
  height: 17px;
}

.ghost-button:hover {
  border-color: rgba(168, 64, 47, 0.28);
  background: var(--surface);
  color: var(--danger);
}

.dashboard {
  display: flex;
  flex: 1;
  flex-direction: column;
  gap: 28px;
  width: min(1180px, calc(100% - 40px));
  margin: 0 auto;
  padding: 24px 0 60px;
  animation: dashboard-in 0.62s cubic-bezier(0.2, 0.7, 0.2, 1) both;
}

.hero {
  position: relative;
  display: grid;
  grid-template-columns: minmax(0, 1fr) minmax(250px, 340px);
  gap: 28px;
  align-items: stretch;
  overflow: hidden;
  padding: clamp(28px, 4vw, 48px);
  border-radius: var(--radius-xl);
  background:
    radial-gradient(circle at 88% 12%, rgba(233, 198, 139, 0.18), transparent 32%),
    linear-gradient(140deg, #17483b, #0d2f27 70%);
  box-shadow: var(--shadow-md);
  color: #f7f1e6;
}

.hero::before {
  position: absolute;
  top: -180px;
  right: -100px;
  width: 380px;
  height: 380px;
  border: 1px solid rgba(247, 241, 230, 0.12);
  border-radius: 50%;
  content: '';
}

.hero::after {
  position: absolute;
  bottom: -220px;
  left: 28%;
  width: 420px;
  height: 420px;
  border: 1px solid rgba(247, 241, 230, 0.08);
  border-radius: 50%;
  content: '';
}

.hero > * {
  position: relative;
  z-index: 1;
}

.hero .eyebrow {
  color: rgba(247, 241, 230, 0.5);
}

.hero h1 {
  margin-top: 14px;
  font-family: var(--font-serif);
  font-size: clamp(2rem, 4.2vw, 3.4rem);
  font-weight: 600;
  letter-spacing: -0.025em;
  line-height: 1.12;
}

.hero__dot {
  color: var(--amber-soft);
}

.hero__sub {
  max-width: 520px;
  margin-top: 14px;
  color: rgba(247, 241, 230, 0.66);
  font-size: 0.94rem;
  line-height: 1.8;
}

.hero__note {
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  gap: 18px;
  padding: 22px;
  border: 1px solid rgba(247, 241, 230, 0.14);
  border-radius: 22px;
  background: rgba(247, 241, 230, 0.07);
  backdrop-filter: blur(12px);
}

.hero__note span {
  color: var(--amber-soft);
  font-size: 0.66rem;
  font-weight: 600;
  letter-spacing: 0.2em;
  text-transform: uppercase;
}

.hero__note p {
  color: rgba(247, 241, 230, 0.84);
  font-family: var(--font-serif);
  font-size: 1.04rem;
  line-height: 1.6;
}

.catalog-card {
  display: flex;
  flex-direction: column;
  gap: 22px;
  padding: clamp(24px, 3.4vw, 34px);
  border: 1px solid rgba(255, 255, 255, 0.8);
  border-radius: var(--radius-lg);
  background: rgba(255, 253, 248, 0.78);
  box-shadow: var(--shadow-sm);
  backdrop-filter: blur(12px);
}

.catalog-card__head {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 24px;
}

.catalog-card__head h2 {
  margin-top: 4px;
  font-family: var(--font-serif);
  font-size: clamp(1.4rem, 2.4vw, 1.8rem);
  font-weight: 600;
  letter-spacing: -0.01em;
}

.catalog-card__head p:last-child {
  margin-top: 6px;
  color: var(--ink-muted);
  font-size: 0.85rem;
}

.search-mode {
  display: inline-flex;
  flex: none;
  padding: 4px;
  border: 1px solid var(--line);
  border-radius: 999px;
  background: rgba(20, 61, 50, 0.05);
}

.search-mode button {
  height: 34px;
  padding: 0 16px;
  border: 0;
  border-radius: 999px;
  background: transparent;
  color: var(--ink-muted);
  font-size: 0.8rem;
  font-weight: 600;
  transition:
    background-color 0.2s ease,
    color 0.2s ease,
    box-shadow 0.2s ease;
}

.search-mode button.is-active {
  background: var(--surface);
  box-shadow: 0 4px 12px -8px rgba(13, 44, 36, 0.45);
  color: var(--forest);
}

.search-mode button:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.catalog-search {
  display: grid;
  grid-template-columns: minmax(0, 1fr) auto;
  gap: 12px;
}

.catalog-search__field {
  position: relative;
}

.catalog-search__field svg {
  position: absolute;
  top: 50%;
  left: 17px;
  width: 18px;
  height: 18px;
  transform: translateY(-50%);
  color: var(--ink-muted);
  pointer-events: none;
}

.catalog-search__field input {
  width: 100%;
  height: 52px;
  padding: 0 18px 0 47px;
  border: 1px solid var(--line-strong);
  border-radius: 999px;
  background: rgba(255, 255, 255, 0.7);
  color: var(--ink);
  font-size: 0.92rem;
  transition:
    border-color 0.2s ease,
    box-shadow 0.2s ease,
    background-color 0.2s ease;
}

.catalog-search__field input::placeholder {
  color: #8f9a93;
}

.catalog-search__field input:hover {
  border-color: rgba(20, 61, 50, 0.3);
}

.catalog-search__field input:focus {
  border-color: var(--forest-soft);
  outline: none;
  background: var(--surface);
  box-shadow: 0 0 0 4px rgba(53, 107, 88, 0.12);
}

.catalog-search__button {
  height: 52px;
  padding: 0 28px;
}

.catalog-results {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.catalog-results__meta {
  color: var(--ink-muted);
  font-size: 0.75rem;
  font-weight: 600;
  letter-spacing: 0.05em;
}

.book-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(210px, 1fr));
  gap: 14px;
}

.book-card {
  display: flex;
  flex-direction: column;
  gap: 9px;
  padding: 18px;
  border: 1px solid var(--line);
  border-radius: 18px;
  background: linear-gradient(160deg, rgba(255, 255, 255, 0.84), rgba(242, 236, 224, 0.55));
  transition:
    transform 0.2s ease,
    border-color 0.2s ease,
    box-shadow 0.2s ease;
}

.book-card:hover {
  transform: translateY(-2px);
  border-color: rgba(20, 61, 50, 0.24);
  box-shadow: var(--shadow-sm);
}

.book-card__top {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 10px;
}

.book-card__bid {
  padding: 3px 8px;
  border-radius: 6px;
  background: rgba(20, 61, 50, 0.07);
  color: var(--forest-soft);
  font-size: 0.68rem;
  font-weight: 700;
  letter-spacing: 0.08em;
}

.book-card__stock {
  color: var(--success);
  font-size: 0.7rem;
  font-weight: 600;
}

.book-card__stock.is-empty {
  color: var(--danger);
}

.book-card h3 {
  font-family: var(--font-serif);
  font-size: 1.16rem;
  font-weight: 600;
  line-height: 1.3;
}

.book-card__author {
  color: var(--ink-muted);
  font-size: 0.78rem;
}

.book-card__foot {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  justify-content: space-between;
  gap: 8px 12px;
  margin-top: auto;
  padding-top: 9px;
  border-top: 1px solid var(--line);
}

.book-card__price {
  color: var(--ink-soft);
  font-size: 0.82rem;
  font-weight: 600;
}

.book-card__actions {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  margin-left: auto;
}

.book-card__action {
  padding: 7px 12px;
  border: 1px solid rgba(53, 107, 88, 0.22);
  border-radius: 999px;
  background: rgba(53, 107, 88, 0.08);
  color: var(--forest);
  font-size: 0.74rem;
  font-weight: 600;
  transition:
    background-color 0.2s ease,
    border-color 0.2s ease,
    color 0.2s ease;
}

.book-card__action--edit {
  border-color: rgba(185, 119, 45, 0.26);
  background: rgba(185, 119, 45, 0.1);
  color: var(--amber);
}

.book-card__action:hover:not(:disabled) {
  border-color: var(--forest);
  background: var(--forest);
  color: #f8f2e6;
}

.book-card__action--edit:hover:not(:disabled) {
  border-color: var(--amber);
  background: var(--amber);
  color: #fff8ec;
}

.book-card__action:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.catalog-empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 6px;
  min-height: 140px;
  padding: 20px;
  border: 1px dashed var(--line-strong);
  border-radius: 18px;
  background: rgba(20, 61, 50, 0.02);
  text-align: center;
}

.catalog-empty svg {
  width: 30px;
  height: 30px;
  color: var(--ink-muted);
  opacity: 0.55;
}

.catalog-empty p {
  color: var(--ink-soft);
  font-size: 0.86rem;
  font-weight: 600;
}

.catalog-empty small {
  color: var(--ink-muted);
  font-size: 0.74rem;
}

.workspace {
  display: grid;
  grid-template-columns: minmax(0, 1.55fr) minmax(300px, 0.85fr);
  gap: 24px;
  align-items: start;
}

.action-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 24px;
}

.action-card {
  position: relative;
  display: flex;
  flex-direction: column;
  gap: 18px;
  overflow: hidden;
  padding: 26px;
  border: 1px solid rgba(255, 255, 255, 0.8);
  border-radius: var(--radius-lg);
  background: rgba(255, 253, 248, 0.8);
  box-shadow: var(--shadow-sm);
  backdrop-filter: blur(12px);
}

.action-card::before {
  position: absolute;
  top: 0;
  right: 26px;
  left: 26px;
  height: 2px;
  border-radius: 0 0 2px 2px;
  background: linear-gradient(90deg, var(--amber), transparent);
  content: '';
  opacity: 0.8;
}

.action-card--return::before {
  background: linear-gradient(90deg, var(--forest-soft), transparent);
}

.action-card__head {
  display: flex;
  align-items: flex-start;
  gap: 14px;
  margin-bottom: 2px;
}

.action-card__icon {
  display: grid;
  flex: none;
  width: 46px;
  height: 46px;
  place-items: center;
  border-radius: 15px;
  background: linear-gradient(135deg, var(--amber), #9a5a1f);
  box-shadow: 0 12px 22px -14px rgba(154, 90, 31, 0.8);
  color: #fdf6e8;
}

.action-card--return .action-card__icon {
  background: linear-gradient(135deg, var(--forest), var(--forest-deep));
  box-shadow: 0 12px 22px -14px rgba(13, 44, 36, 0.8);
  color: #f2ddae;
}

.action-card__icon svg {
  width: 24px;
  height: 24px;
}

.action-card__head h2 {
  font-family: var(--font-serif);
  font-size: 1.35rem;
  font-weight: 600;
  line-height: 1.2;
}

.action-card__head p {
  margin-top: 5px;
  color: var(--ink-muted);
  font-size: 0.8rem;
}

.account-note {
  display: flex;
  align-items: center;
  gap: 8px;
  min-height: 44px;
  padding: 0 14px;
  border: 1px dashed var(--line-strong);
  border-radius: 12px;
  background: rgba(20, 61, 50, 0.03);
  color: var(--ink-soft);
  font-size: 0.82rem;
}

.account-note strong {
  color: var(--forest);
}

.account-note__dot {
  width: 7px;
  height: 7px;
  border-radius: 50%;
  background: var(--success);
  box-shadow: 0 0 0 3px rgba(46, 107, 79, 0.12);
}

.action-card .primary-button {
  width: 100%;
  margin-top: 2px;
}

.side-column {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.account-card {
  position: relative;
  overflow: hidden;
  padding: 24px;
  border: 1px solid rgba(255, 255, 255, 0.8);
  border-radius: var(--radius-lg);
  background: linear-gradient(160deg, rgba(255, 253, 248, 0.92), rgba(236, 226, 209, 0.72));
  box-shadow: var(--shadow-sm);
}

.account-card::after {
  position: absolute;
  top: -50px;
  right: -50px;
  width: 140px;
  height: 140px;
  border: 1px solid var(--line);
  border-radius: 50%;
  content: '';
}

.account-card__top {
  position: relative;
  z-index: 1;
  display: flex;
  align-items: center;
  gap: 14px;
}

.account-card__avatar {
  display: grid;
  flex: none;
  width: 50px;
  height: 50px;
  place-items: center;
  border-radius: 16px;
  background: var(--forest);
  color: var(--amber-soft);
  font-family: var(--font-serif);
  font-size: 1.05rem;
  letter-spacing: 0.04em;
}

.account-card__info {
  display: flex;
  flex-direction: column;
  gap: 2px;
  min-width: 0;
}

.account-card__info span {
  color: var(--ink-muted);
  font-size: 0.66rem;
  letter-spacing: 0.16em;
  text-transform: uppercase;
}

.account-card__info strong {
  overflow: hidden;
  color: var(--ink);
  font-size: 1rem;
  font-weight: 600;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.account-card__name {
  color: var(--ink-muted);
  font-size: 0.72rem;
  font-weight: 500;
}

.account-card__status {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  margin-left: auto;
  color: var(--success);
  font-size: 0.72rem;
  font-weight: 600;
}

.account-card__status::before {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: var(--success);
  box-shadow: 0 0 0 3px rgba(46, 107, 79, 0.14);
  content: '';
}

.account-card__meta {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 10px;
  margin-top: 20px;
  padding-top: 18px;
  border-top: 1px solid var(--line);
}

.account-card__meta div {
  display: flex;
  flex-direction: column;
  gap: 3px;
}

.account-card__meta dt {
  color: var(--ink-muted);
  font-size: 0.68rem;
  letter-spacing: 0.1em;
}

.account-card__meta dd {
  color: var(--ink-soft);
  font-size: 0.85rem;
  font-weight: 600;
}

.account-card__edit {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  width: 100%;
  height: 42px;
  margin-top: 18px;
  border: 1px solid var(--line-strong);
  border-radius: 999px;
  background: rgba(255, 253, 248, 0.65);
  color: var(--ink-soft);
  font-size: 0.8rem;
  font-weight: 600;
  transition:
    color 0.2s ease,
    border-color 0.2s ease,
    background-color 0.2s ease;
}

.account-card__edit svg {
  width: 16px;
  height: 16px;
}

.account-card__edit:hover {
  border-color: var(--forest-soft);
  background: var(--surface);
  color: var(--forest);
}

.activity-card {
  display: flex;
  flex-direction: column;
  gap: 16px;
  min-height: 300px;
  padding: 24px;
  border: 1px solid rgba(255, 255, 255, 0.8);
  border-radius: var(--radius-lg);
  background: rgba(255, 253, 248, 0.8);
  box-shadow: var(--shadow-sm);
  backdrop-filter: blur(12px);
}

.activity-card__head {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 12px;
}

.activity-card__head .eyebrow {
  margin-bottom: 4px;
}

.activity-card__head h2 {
  font-family: var(--font-serif);
  font-size: 1.3rem;
  font-weight: 600;
}

.text-button {
  padding: 2px 0;
  border: 0;
  border-bottom: 1px solid transparent;
  background: none;
  color: var(--ink-muted);
  font-size: 0.8rem;
  font-weight: 600;
  transition:
    color 0.2s ease,
    border-color 0.2s ease;
}

.text-button:hover {
  border-color: currentColor;
  color: var(--danger);
}

.activity-list {
  display: flex;
  flex-direction: column;
  gap: 0;
  max-height: 360px;
  margin-right: -6px;
  padding-right: 6px;
  overflow: auto;
  list-style: none;
}

.activity-list li {
  position: relative;
  display: flex;
  gap: 12px;
  padding-bottom: 18px;
}

.activity-list li:not(:last-child)::before {
  position: absolute;
  top: 12px;
  bottom: 0;
  left: 4px;
  width: 1px;
  background: var(--line);
  content: '';
}

.activity-list__dot {
  flex: none;
  width: 9px;
  height: 9px;
  margin-top: 5px;
  border-radius: 50%;
  background: var(--success);
  box-shadow: 0 0 0 3px rgba(46, 107, 79, 0.12);
}

.activity-list li.is-error .activity-list__dot {
  background: var(--danger);
  box-shadow: 0 0 0 3px rgba(168, 64, 47, 0.12);
}

.activity-list__body {
  flex: 1;
  min-width: 0;
}

.activity-list__title {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  gap: 10px;
}

.activity-list__title strong {
  color: var(--ink);
  font-size: 0.85rem;
  font-weight: 600;
}

.activity-list__title time {
  color: var(--ink-muted);
  font-size: 0.7rem;
  white-space: nowrap;
}

.activity-list__body p {
  margin-top: 2px;
  color: var(--ink-muted);
  font-size: 0.76rem;
}

.activity-empty {
  display: flex;
  flex: 1;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 8px;
  min-height: 190px;
  padding: 24px;
  border: 1px dashed var(--line-strong);
  border-radius: 18px;
  color: var(--ink-muted);
  text-align: center;
}

.activity-empty svg {
  width: 34px;
  height: 34px;
  opacity: 0.4;
}

.activity-empty p {
  color: var(--ink-soft);
  font-size: 0.86rem;
  font-weight: 600;
}

.activity-empty small {
  max-width: 200px;
  font-size: 0.72rem;
}

.editor-overlay {
  position: fixed;
  z-index: 30;
  inset: 0;
  display: grid;
  padding: 24px;
  place-items: center;
  background: rgba(13, 44, 36, 0.46);
  backdrop-filter: blur(5px);
}

.editor-card {
  width: min(540px, 100%);
  max-height: calc(100vh - 48px);
  padding: clamp(24px, 4vw, 34px);
  overflow: auto;
  border: 1px solid rgba(255, 255, 255, 0.86);
  border-radius: var(--radius-lg);
  background: var(--surface);
  box-shadow: 0 40px 90px -40px rgba(13, 44, 36, 0.7);
}

.editor-card__head {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 18px;
}

.editor-card__head h2 {
  margin-top: 6px;
  font-family: var(--font-serif);
  font-size: clamp(1.5rem, 3vw, 1.9rem);
  font-weight: 600;
  line-height: 1.2;
}

.editor-card__head p:last-child {
  margin-top: 7px;
  color: var(--ink-muted);
  font-size: 0.82rem;
}

.editor-card__close {
  display: grid;
  flex: none;
  width: 38px;
  height: 38px;
  place-items: center;
  border: 1px solid var(--line);
  border-radius: 50%;
  background: rgba(20, 61, 50, 0.04);
  color: var(--ink-muted);
  transition:
    color 0.2s ease,
    border-color 0.2s ease,
    background-color 0.2s ease;
}

.editor-card__close svg {
  width: 18px;
  height: 18px;
}

.editor-card__close:hover:not(:disabled) {
  border-color: rgba(168, 64, 47, 0.3);
  background: var(--danger-soft);
  color: var(--danger);
}

.editor-card__close:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.editor-card__meta {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  margin-top: 18px;
}

.editor-card__meta span {
  padding: 4px 10px;
  border-radius: 999px;
  background: rgba(20, 61, 50, 0.06);
  color: var(--ink-soft);
  font-size: 0.72rem;
  font-weight: 600;
}

.editor-card__meta span:first-child {
  background: rgba(185, 119, 45, 0.12);
  color: var(--amber);
  letter-spacing: 0.08em;
}

.editor-form {
  display: flex;
  flex-direction: column;
  gap: 18px;
  margin-top: 22px;
}

.editor-form__grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 14px;
}

.profile-form__divider {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  gap: 12px;
  padding-top: 4px;
  border-top: 1px solid var(--line);
}

.profile-form__divider span {
  color: var(--ink-soft);
  font-size: 0.82rem;
  font-weight: 600;
}

.profile-form__divider small {
  color: var(--ink-muted);
  font-size: 0.72rem;
}

.editor-form__actions {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
  margin-top: 2px;
}

.editor-form__actions .primary-button {
  height: 46px;
  padding: 0 22px;
}

.secondary-button {
  height: 46px;
  padding: 0 22px;
  border: 1px solid var(--line-strong);
  border-radius: 999px;
  background: transparent;
  color: var(--ink-soft);
  font-size: 0.9rem;
  font-weight: 600;
  transition:
    color 0.2s ease,
    border-color 0.2s ease,
    background-color 0.2s ease;
}

.secondary-button:hover:not(:disabled) {
  border-color: var(--forest-soft);
  background: rgba(20, 61, 50, 0.05);
  color: var(--forest);
}

.secondary-button:disabled {
  opacity: 0.55;
  cursor: not-allowed;
}

.editor-enter-active,
.editor-leave-active {
  transition: opacity 0.2s ease;
}

.editor-enter-active .editor-card,
.editor-leave-active .editor-card {
  transition:
    opacity 0.2s ease,
    transform 0.2s ease;
}

.editor-enter-from,
.editor-leave-to {
  opacity: 0;
}

.editor-enter-from .editor-card,
.editor-leave-to .editor-card {
  opacity: 0;
  transform: translateY(14px) scale(0.98);
}

@media (max-width: 1020px) {
  .workspace {
    grid-template-columns: 1fr;
  }

  .side-column {
    display: grid;
    grid-template-columns: 1fr 1.2fr;
  }
}

@media (max-width: 760px) {
  .topbar {
    width: calc(100% - 32px);
    padding-top: 20px;
  }

  .topbar__account {
    display: none;
  }

  .dashboard {
    width: calc(100% - 32px);
    padding-bottom: 40px;
  }

  .hero {
    grid-template-columns: 1fr;
    padding: 28px;
  }

  .hero__note {
    display: none;
  }

  .catalog-card__head {
    flex-direction: column;
    gap: 16px;
  }

  .search-mode {
    width: 100%;
  }

  .search-mode button {
    flex: 1;
  }

  .catalog-search {
    grid-template-columns: 1fr;
  }

  .catalog-search__button {
    width: 100%;
  }

  .action-grid {
    grid-template-columns: 1fr;
  }

  .side-column {
    grid-template-columns: 1fr;
  }
}

@media (max-width: 560px) {
  .editor-overlay {
    padding: 16px;
  }

  .editor-card {
    padding: 22px;
  }

  .editor-form__grid {
    grid-template-columns: 1fr;
  }

  .editor-form__actions {
    flex-direction: column-reverse;
  }

  .editor-form__actions .primary-button,
  .editor-form__actions .secondary-button {
    width: 100%;
  }
}

@media (max-width: 460px) {
  .ghost-button {
    width: 40px;
    padding: 0;
    justify-content: center;
    font-size: 0;
    gap: 0;
  }

  .ghost-button svg {
    width: 18px;
    height: 18px;
  }

  .action-card {
    padding: 22px;
  }

  .action-card::before {
    right: 22px;
    left: 22px;
  }
}
</style>
