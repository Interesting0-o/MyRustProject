import { ref } from 'vue'
import { defineStore } from 'pinia'

let sequence = 0

export const useActivityStore = defineStore('activity', () => {
  const entries = ref([])

  function record(entry) {
    sequence += 1
    entries.value = [
      {
        id: `${Date.now()}-${sequence}`,
        at: new Date().toISOString(),
        ...entry,
      },
      ...entries.value,
    ].slice(0, 20)
  }

  function clear() {
    entries.value = []
  }

  return { entries, record, clear }
})
