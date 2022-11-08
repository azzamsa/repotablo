import { defineStore } from 'pinia'

export const useLinksStore = defineStore({
  id: 'links',
  state: () => ({
    links: '',
  }),
  getters: {
    getLinks(state) {
      return state.links
    },
  },
  actions: {
    setLinks(newContent: string) {
      this.links = newContent
    },
  },
})
