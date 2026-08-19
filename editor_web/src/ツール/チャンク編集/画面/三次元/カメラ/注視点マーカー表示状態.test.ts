import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import { 注視点マーカー表示状態 } from './注視点マーカー表示状態.ts'

describe('注視点マーカー表示状態', () => {
    it('初期状態は非可視であること', () => {
        const 状態 = new 注視点マーカー表示状態()
        assert.strictEqual(状態.可視か(), false)
    })

    it('操作されたら可視になること', () => {
        const 状態 = new 注視点マーカー表示状態()
        状態.操作された()
        assert.strictEqual(状態.可視か(), true)
    })

    it('非表示にすると可視から非可視へ戻ること', () => {
        const 状態 = new 注視点マーカー表示状態()
        状態.操作された()
        状態.非表示にする()
        assert.strictEqual(状態.可視か(), false)
    })

    it('非表示後に再び操作されたら可視へ戻ること', () => {
        const 状態 = new 注視点マーカー表示状態()
        状態.操作された()
        状態.非表示にする()
        状態.操作された()
        assert.strictEqual(状態.可視か(), true)
    })
})
