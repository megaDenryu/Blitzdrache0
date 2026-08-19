import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import { 大域編集状態 } from './大域編集状態.ts'

describe('大域編集状態の履歴スタック上限', () => {
    it('履歴スタック上限(50件)を超えて積むと最古の断片が破棄されること', () => {
        const 状態 = new 大域編集状態()
        for (let i = 0; i < 55; i++) {
            状態.取り消し断片を積む({
                種類: '造成筆致',
                対象チャンク: null,
                変更前格子データ: new Float32Array([i]),
            })
        }
        assert.equal(状態.取り消し履歴スタック.length, 50)
        const 最新 = 状態.取り消し断片を取り出す()
        assert.ok(最新 !== undefined)
        assert.equal(最新.種類, '造成筆致')
        if (最新.種類 === '造成筆致') {
            assert.equal(最新.変更前格子データ[0], 54)
        }
    })
})
