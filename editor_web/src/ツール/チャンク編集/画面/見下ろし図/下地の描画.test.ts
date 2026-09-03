import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import { 高さ場, 地表材質 } from '../../編集モデル/index.ts'
import { 下地の画像を作る } from './下地の描画.ts'
import type { 下地の配色 } from './下地の配色.ts'

// 標高の色を白一色にし、識別色を原色にすると、画素の色から格子点の材質を読める。
const 配色: 下地の配色 = {
    材質色: { 草: '#00ff00', 泥: '#ff0000', 岩: '#0000ff', 砂: '#ffff00' },
    標高低色: 0xffffff,
    標高中色: 0xffffff,
    標高高色: 0xffffff,
}

function 材質を1層だけにする(材質: 地表材質, gx: number, gz: number, 層: number): void {
    const 添字 = (gz * 材質.解像度 + gx) * 4
    for (let i = 0; i < 4; i++) 材質.材質データ[添字 + i] = i === 層 ? 255 : 0
}

function 画素を読む(画像: { 幅: number; 画素: Uint8ClampedArray }, px: number, py: number): [number, number, number, number] {
    const 添字 = (py * 画像.幅 + px) * 4
    return [画像.画素[添字] ?? -1, 画像.画素[添字 + 1] ?? -1, 画像.画素[添字 + 2] ?? -1, 画像.画素[添字 + 3] ?? -1]
}

describe('下地の画像を作る', () => {
    it('四隅の画素が期待の格子点に対応し、格子のZが0の行が画像の上端に来ること', () => {
        const 解像度 = 5
        const 高さ = new 高さ場(解像度, 8)
        const 材質 = new 地表材質(解像度, 8)
        材質を1層だけにする(材質, 0, 0, 1)
        材質を1層だけにする(材質, 解像度 - 1, 0, 2)
        材質を1層だけにする(材質, 0, 解像度 - 1, 3)
        材質を1層だけにする(材質, 解像度 - 1, 解像度 - 1, 0)

        const 画像 = 下地の画像を作る(高さ, 材質, 配色)
        assert.strictEqual(画像.幅, 解像度)
        assert.strictEqual(画像.高さ, 解像度)
        assert.strictEqual(画像.画素.length, 解像度 * 解像度 * 4)

        // 標高の色(白)を0.4混ぜるため、原色の成分は255、0の成分は102になる。
        assert.deepStrictEqual(画素を読む(画像, 0, 0), [255, 102, 102, 255], '左上=北西(gx=0,gz=0)は泥')
        assert.deepStrictEqual(画素を読む(画像, 解像度 - 1, 0), [102, 102, 255, 255], '右上=北東(gx=最大,gz=0)は岩')
        assert.deepStrictEqual(画素を読む(画像, 0, 解像度 - 1), [255, 255, 102, 255], '左下=南西(gx=0,gz=最大)は砂')
        assert.deepStrictEqual(画素を読む(画像, 解像度 - 1, 解像度 - 1), [102, 255, 102, 255], '右下=南東(gx=最大,gz=最大)は草')
    })

    it('高い格子点ほど標高の高い側の色に近づくこと', () => {
        const 解像度 = 3
        const 高さ = new 高さ場(解像度, 4)
        高さ.標高を格子添字で設定する(0, 0)
        高さ.標高を格子添字で設定する(解像度 * 解像度 - 1, 10)
        const 材質 = new 地表材質(解像度, 4)
        const 黒と白の配色: 下地の配色 = {
            材質色: { 草: '#000000', 泥: '#000000', 岩: '#000000', 砂: '#000000' },
            標高低色: 0x000000,
            標高中色: 0x808080,
            標高高色: 0xffffff,
        }
        const 画像 = 下地の画像を作る(高さ, 材質, 黒と白の配色)
        const 低い = 画素を読む(画像, 0, 0)
        const 高い = 画素を読む(画像, 解像度 - 1, 解像度 - 1)
        assert.strictEqual(低い[0], 0, '最も低い点は低色のまま')
        assert.strictEqual(高い[0], 102, '最も高い点は高色(白)を0.4混ぜた値')
    })

    it('識別色の書式が違うときは例外にすること', () => {
        const 高さ = new 高さ場(2, 2)
        const 材質 = new 地表材質(2, 2)
        assert.throws(() => 下地の画像を作る(高さ, 材質, { ...配色, 材質色: { ...配色.材質色, 草: 'green' } }))
    })
})
