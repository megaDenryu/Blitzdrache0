import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import { 高さ場 } from '../../../編集モデル/高さ場.ts'
import { ブラシ角度列を作る, ブラシ浮上量メートル } from './ブラシ角度列.ts'
import { ブラシリング頂点を更新する } from './ブラシリング頂点計算.ts'

function 一定高さの高さ場を作る(高さ: number): 高さ場 {
    const 解像度 = 4
    const 格子データ = new Float32Array(解像度 * 解像度).fill(高さ)
    return new 高さ場(解像度, 100, 格子データ)
}

describe('ブラシリング頂点計算', () => {
    it('平坦な地形ではリングの全頂点のYが地形の高さ+浮上量になること', () => {
        const 分割数 = 8
        const 角度列 = ブラシ角度列を作る(分割数)
        const 頂点配列 = new Float32Array(分割数 * 2 * 3)
        const 高さ場モデル = 一定高さの高さ場を作る(10)

        ブラシリング頂点を更新する(頂点配列, 角度列, 0, 0, 5, 高さ場モデル)

        for (let i = 0; i < 分割数 * 2; i++) {
            const y = 頂点配列[i * 3 + 1]
            assert.ok(y !== undefined)
            assert.ok(Math.abs(y - (10 + ブラシ浮上量メートル)) < 1e-4, `頂点${i}のYが地形高さ+浮上量であるべき: ${y}`)
        }
    })

    it('中心をずらすと頂点のローカルXZが中心からの相対座標になること', () => {
        const 分割数 = 4
        const 角度列 = ブラシ角度列を作る(分割数)
        const 頂点配列 = new Float32Array(分割数 * 2 * 3)
        const 高さ場モデル = 一定高さの高さ場を作る(0)

        ブラシリング頂点を更新する(頂点配列, 角度列, 20, -10, 3, 高さ場モデル)

        // 角度0の外周点: ワールド(20+3, -10) → ローカル(3, 0)
        assert.ok(Math.abs((頂点配列[1 * 3 + 0] ?? NaN) - 3) < 1e-4)
        assert.ok(Math.abs((頂点配列[1 * 3 + 2] ?? NaN) - 0) < 1e-4)
    })
})
