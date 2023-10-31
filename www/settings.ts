export const settings = {
  waitLittle: async (): Promise<void> => { await new Promise(resolve => setTimeout(resolve, 10)) }
}
