import axios from 'axios'

const client = axios.create({
  baseURL: import.meta.env.VITE_API_BASE_URL || '',
  timeout: 15000,
})

const loginPaths = {
  reader: '/login/reader',
  librarian: '/login/librarian',
}

const signUpPaths = {
  reader: '/sign_up/reader',
  librarian: '/sign_up/librarian',
}

function errorMessage(error) {
  if (error.response?.data?.messages) {
    return error.response.data.messages
  }

  if (error.code === 'ECONNABORTED' || error.code === 'ETIMEDOUT') {
    return '请求超时，请稍后重试'
  }

  if (error.response) {
    return '服务器暂时无法处理请求，请稍后重试'
  }

  return '无法连接服务器，请确认后端服务已启动'
}

async function post(path, payload) {
  try {
    const { data } = await client.post(path, payload)
    return data ?? { success: false, messages: '服务器返回了空响应', token: null }
  } catch (error) {
    return { success: false, messages: errorMessage(error), token: null }
  }
}

export function login(role, credentials) {
  return post(loginPaths[role], credentials)
}

export function signUp(role, profile) {
  return post(signUpPaths[role], profile)
}

export function borrowBook(account, bid) {
  return post('/borrow_book', { account, bid })
}

export function returnBook(account, bid) {
  return post('/return_book', { account, bid })
}

export function findBookByBid(bid) {
  return post('/find_book_by_bid', { bid })
}

export function findBookByName(name) {
  return post('/find_book_by_name', { name })
}

export function updateBookName(bid, name) {
  return post('/update_book_name', { bid, name })
}

export function updateBookPrice(bid, price) {
  return post('/update_book_price', { bid, price })
}

export function updateBookNum(bid, num) {
  return post('/update_book_num', { bid, num })
}

export function updateReader(account, { name, currentPassword, newPassword } = {}) {
  return post('/update_reader', {
    account,
    name,
    current_password: currentPassword,
    new_password: newPassword,
  })
}
