import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import { 高さ場 } from '../../../編集モデル/高さ場.ts'
import { ブラシ角度列を作る, ブラシ浮上量メートル } from './ブラシ角度列.ts'
import { ブラシ塗り円頂点を更新する } from './ブラシ塗り円頂点計算.ts'

function 一定高さの高さ場を作る(高さ: number): 高さ場 {
    const 解像度 = 4
    const 格子データ = new Float32Array(解像度 * 解像度).fill(高さ)
    return new 高さ場(解像度, 100, 格子データ)
}

describe('ブラシ塗り円頂点計算', () => {
    it('塗り円の中心頂点(添字0)が中心位置の地形高さ+浮上量になること', () => {
        const 分割数 = 6
        const 角度列 = ブラシ角度列を作る(分割数)
        const 頂点配列 = new Float32Array((分割数 + 1) * 3)
        const 高さ場モデル = 一定高さの高さ場を作る(7)

        ブラシ塗り円頂点を更新する(頂点配列, 角度列, 0, 0, 4, 高さ場モデル)

        assert.strictEqual(頂点配列[0], 0)
        assert.ok(Math.abs((頂点配列[1] ?? NaN) - (7 + ブラシ浮上量メートル)) < 1e-4)
        assert.strictEqual(頂点配列[2], 0)
    })
})
