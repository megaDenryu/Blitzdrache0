import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import { 居なくなった道路点の選択を外す } from './選択中の道路点の後始末.ts'

describe('選択中の道路点の後始末', () => {
    it('制御点の数より大きい添字を選んだままなら選択を外すこと', () => {
        const 状態 = { 選択中の道路点の添字: 3 as number | null, つかんでいる道路点の添字: null }
        居なくなった道路点の選択を外す(状態, 3)
        assert.strictEqual(状態.選択中の道路点の添字, null)
    })

    it('制御点が1つも無くなったら選択を外すこと', () => {
        const 状態 = { 選択中の道路点の添字: 0 as number | null, つかんでいる道路点の添字: null }
        居なくなった道路点の選択を外す(状態, 0)
        assert.strictEqual(状態.選択中の道路点の添字, null)
    })

    it('まだ居る道路点を選んでいるなら選択を保つこと', () => {
        const 状態 = { 選択中の道路点の添字: 2 as number | null, つかんでいる道路点の添字: null }
        居なくなった道路点の選択を外す(状態, 3)
        assert.strictEqual(状態.選択中の道路点の添字, 2)
    })

    it('何も選んでいないときは何もしないこと', () => {
        const 状態 = { 選択中の道路点の添字: null as number | null, つかんでいる道路点の添字: null }
        居なくなった道路点の選択を外す(状態, 0)
        assert.strictEqual(状態.選択中の道路点の添字, null)
    })
})
