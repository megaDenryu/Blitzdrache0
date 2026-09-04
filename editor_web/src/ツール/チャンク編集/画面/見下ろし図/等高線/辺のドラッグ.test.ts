import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import { 辺の差分を加えた頂点列 } from './辺のドラッグ.ts'

describe('辺の差分を加えた頂点列', () => {
    it('辺の両端の頂点だけへ差分を加えること', () => {
        const 頂点列 = [{ x: 0, z: 0 }, { x: 10, z: 0 }, { x: 10, z: 10 }]
        const 結果 = 辺の差分を加えた頂点列(頂点列, { 線の添字: 0, 頂点の添字甲: 0, 頂点の添字乙: 1 }, { x: 2, z: 3 })
        assert.deepStrictEqual(結果, [{ x: 2, z: 3 }, { x: 12, z: 3 }, { x: 10, z: 10 }])
    })

    it('元の頂点列を書き換えないこと', () => {
        const 頂点列 = [{ x: 0, z: 0 }, { x: 10, z: 0 }]
        辺の差分を加えた頂点列(頂点列, { 線の添字: 0, 頂点の添字甲: 0, 頂点の添字乙: 1 }, { x: 5, z: 5 })
        assert.deepStrictEqual(頂点列, [{ x: 0, z: 0 }, { x: 10, z: 0 }])
    })

    it('閉じた線の末尾と先頭を結ぶ辺なら末尾と先頭の頂点へ差分を加えること', () => {
        const 頂点列 = [{ x: 0, z: 0 }, { x: 10, z: 0 }, { x: 10, z: 10 }]
        const 結果 = 辺の差分を加えた頂点列(頂点列, { 線の添字: 0, 頂点の添字甲: 2, 頂点の添字乙: 0 }, { x: 1, z: 1 })
        assert.deepStrictEqual(結果, [{ x: 1, z: 1 }, { x: 10, z: 0 }, { x: 11, z: 11 }])
    })
})
