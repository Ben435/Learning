function todo() {
    this.data = new Map()

    return {
        put: async (k, v) => this.data.set(k, v),
        get: (k) => this.data.get(k)
    }
}

global.TODOS = todo()

const index = require('./index')

describe('index', () => {
    it('getCache', () => {
        TODOS.put('hello', 'bye')

        expect(index.getCache('hello')).toEqual('bye')
    })
})