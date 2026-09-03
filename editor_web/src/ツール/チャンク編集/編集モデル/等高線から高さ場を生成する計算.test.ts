import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import type { 等高線 } from '../../../生成/編集資源契約.ts'
import { 等高線から高さ場を生成する } from './等高線から高さ場を生成する計算.ts'

const 解像度 = 33
const 一辺 = 256
const 格子間隔 = 一辺 / (解像度 - 1)

function 正方形の等高線(半辺: number, 高さ: number): 等高線 {
    return {
        高さメートル: 高さ,
        頂点列: [{ x: -半辺, z: -半辺 }, { x: 半辺, z: -半辺 }, { x: 半辺, z: 半辺 }, { x: -半辺, z: 半辺 }],
        閉じている: true,
    }
}

function 起伏のある格子(): Float32Array {
    const 格子 = new Float32Array(解像度 * 解像度)
    for (let i = 0; i < 格子.length; i++) 格子[i] = Math.sin(i * 0.37) * 5 + 3
    return 格子
}

function 格子点(gx: number, gz: number): number {
    return gz * 解像度 + gx
}

describe('等高線から高さ場を生成する計算', () => {
    it('1本の水平な閉じた等高線から生成した内側が等高線の高さで平坦になること', () => {
        const 結果 = 等高線から高さ場を生成する([正方形の等高線(64, 10)], 解像度, 一辺, new Float32Array(解像度 * 解像度))
        const 内側の範囲 = [8, 24]
        for (let gz = 内側の範囲[0] ?? 0; gz <= (内側の範囲[1] ?? 0); gz++) {
            for (let gx = 内側の範囲[0] ?? 0; gx <= (内側の範囲[1] ?? 0); gx++) {
                assert.ok(Math.abs((結果[格子点(gx, gz)] ?? 0) - 10) < 0.05, `内側(${gx},${gz})が10になるべき: ${結果[格子点(gx, gz)]}`)
            }
        }
    })

    it('2本の同心の等高線の間が単調に変わること', () => {
        const 結果 = 等高線から高さ場を生成する([正方形の等高線(96, 4), 正方形の等高線(32, 12)], 解像度, 一辺, new Float32Array(解像度 * 解像度))
        const 中央 = (解像度 - 1) / 2
        const 内側の格子 = 中央 + 32 / 格子間隔
        const 外側の格子 = 中央 + 96 / 格子間隔
        for (let gx = 内側の格子; gx < 外側の格子; gx++) {
            const 手前 = 結果[格子点(gx, 中央)] ?? 0
            const 奥 = 結果[格子点(gx + 1, 中央)] ?? 0
            assert.ok(手前 >= 奥 - 1e-4, `内側から外側へ向かって下がるべき: gx=${gx} ${手前} -> ${奥}`)
        }
        assert.ok(Math.abs((結果[格子点(内側の格子, 中央)] ?? 0) - 12) < 1e-3)
        assert.ok(Math.abs((結果[格子点(外側の格子, 中央)] ?? 0) - 4) < 1e-3)
    })

    it('外周の格子点が変わらないこと', () => {
        const 元 = 起伏のある格子()
        const 結果 = 等高線から高さ場を生成する([正方形の等高線(64, 10)], 解像度, 一辺, 元)
        for (let i = 0; i < 解像度; i++) {
            for (const 添字 of [格子点(i, 0), 格子点(i, 解像度 - 1), 格子点(0, i), 格子点(解像度 - 1, i)]) {
                assert.strictEqual(結果[添字], 元[添字], `外周(${添字})は変わらないべき`)
            }
        }
    })

    it('等高線が0本なら理由の文とともに拒否されること', () => {
        assert.throws(() => 等高線から高さ場を生成する([], 解像度, 一辺, new Float32Array(解像度 * 解像度)), /等高線が1本も無い/)
    })
})
