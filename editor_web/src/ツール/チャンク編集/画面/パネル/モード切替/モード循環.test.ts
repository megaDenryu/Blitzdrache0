import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import { モードを循環させる } from './モード循環.ts'

describe('モードを循環させる', () => {
    const 一覧 = ['選択', '造成', '建物'] as const

    it('方向1で次のモードへ進むこと', () => {
        assert.strictEqual(モードを循環させる(一覧, '選択', 1), '造成')
        assert.strictEqual(モードを循環させる(一覧, '造成', 1), '建物')
    })

    it('末尾から方向1で先頭へ折り返すこと', () => {
        assert.strictEqual(モードを循環させる(一覧, '建物', 1), '選択')
    })

    it('方向-1で前のモードへ戻ること', () => {
        assert.strictEqual(モードを循環させる(一覧, '建物', -1), '造成')
        assert.strictEqual(モードを循環させる(一覧, '造成', -1), '選択')
    })

    it('先頭から方向-1で末尾へ折り返すこと', () => {
        assert.strictEqual(モードを循環させる(一覧, '選択', -1), '建物')
    })

    it('空の一覧を渡すと例外になること', () => {
        assert.throws(() => モードを循環させる([], '選択', 1), /モード一覧が空です/)
    })
})
