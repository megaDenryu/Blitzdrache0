import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import type { 等高線 } from '../../../生成/編集資源契約.ts'
import { 等高線を格子点へ固定する } from './等高線の格子点への固定.ts'
import { 等高線の拘束衝突 } from './等高線の拘束衝突.ts'

const 解像度 = 33
const 一辺 = 256

function 開いた線(高さ: number, 始点: { x: number; z: number }, 終点: { x: number; z: number }): 等高線 {
    return { 高さメートル: 高さ, 頂点列: [始点, 終点], 閉じている: false }
}

describe('等高線を格子点へ固定する', () => {
    it('実際に交差する高さの異なる2本の等高線を拒否すること', () => {
        const 横線 = 開いた線(10, { x: -64, z: 0 }, { x: 64, z: 0 })
        const 縦線 = 開いた線(20, { x: 0, z: -64 }, { x: 0, z: 64 })
        assert.throws(
            () => 等高線を格子点へ固定する([横線, 縦線], 解像度, 一辺),
            (例外: unknown) => 例外 instanceof 等高線の拘束衝突,
        )
    })

    it('交差しないが丸めで同じ格子点へ落ちる高さの異なる2本の等高線を拒否すること', () => {
        const 上の線 = 開いた線(10, { x: -64, z: 0.1 }, { x: 64, z: 0.1 })
        const 下の線 = 開いた線(20, { x: -64, z: -0.1 }, { x: 64, z: -0.1 })
        assert.throws(
            () => 等高線を格子点へ固定する([上の線, 下の線], 解像度, 一辺),
            (例外: unknown) => 例外 instanceof 等高線の拘束衝突,
        )
    })

    it('同じ高さの2本の等高線を許可すること', () => {
        const 横線 = 開いた線(10, { x: -64, z: 0 }, { x: 64, z: 0 })
        const 縦線 = 開いた線(10, { x: 0, z: -64 }, { x: 0, z: 64 })
        assert.doesNotThrow(() => 等高線を格子点へ固定する([横線, 縦線], 解像度, 一辺))
    })

    it('高さの一致とみなす許容差メートル以内の2本の等高線を許可すること', () => {
        const 横線 = 開いた線(10, { x: -64, z: 0 }, { x: 64, z: 0 })
        const 縦線 = 開いた線(10.00005, { x: 0, z: -64 }, { x: 0, z: 64 })
        assert.doesNotThrow(() => 等高線を格子点へ固定する([横線, 縦線], 解像度, 一辺))
    })
})
