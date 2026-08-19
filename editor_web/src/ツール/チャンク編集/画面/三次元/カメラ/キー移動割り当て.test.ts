import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import { キー状態から移動方向を決める } from './キー移動割り当て.ts'

describe('キー状態から移動方向を決める', () => {
    it('何も押されていなければ全成分が0になること', () => {
        const 方向 = キー状態から移動方向を決める(new Set())
        assert.deepStrictEqual(方向, { 前後: 0, 左右: 0, 上下: 0 })
    })

    it('Wで前後が+1、Sで前後が-1になること', () => {
        assert.strictEqual(キー状態から移動方向を決める(new Set(['w'])).前後, 1)
        assert.strictEqual(キー状態から移動方向を決める(new Set(['s'])).前後, -1)
    })

    it('Dで左右が+1、Aで左右が-1になること', () => {
        assert.strictEqual(キー状態から移動方向を決める(new Set(['d'])).左右, 1)
        assert.strictEqual(キー状態から移動方向を決める(new Set(['a'])).左右, -1)
    })

    it('Eで上下が+1、Qで上下が-1になること', () => {
        assert.strictEqual(キー状態から移動方向を決める(new Set(['e'])).上下, 1)
        assert.strictEqual(キー状態から移動方向を決める(new Set(['q'])).上下, -1)
    })

    it('対となるキーを同時に押すと相殺されて0になること', () => {
        assert.strictEqual(キー状態から移動方向を決める(new Set(['w', 's'])).前後, 0)
        assert.strictEqual(キー状態から移動方向を決める(new Set(['a', 'd'])).左右, 0)
        assert.strictEqual(キー状態から移動方向を決める(new Set(['q', 'e'])).上下, 0)
    })

    it('複数軸の同時押しがそれぞれ独立して反映されること', () => {
        const 方向 = キー状態から移動方向を決める(new Set(['w', 'd', 'e']))
        assert.deepStrictEqual(方向, { 前後: 1, 左右: 1, 上下: 1 })
    })
})
