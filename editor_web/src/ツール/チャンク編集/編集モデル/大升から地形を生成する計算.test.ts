import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import { 大升から地形を生成する } from './大升から地形を生成する計算.ts'
import { 高さ場から大升を導く } from './高さ場から大升を導く計算.ts'

const 解像度 = 33
const 一辺の升目数 = 8

function 起伏のある格子(): Float32Array {
    const 格子 = new Float32Array(解像度 * 解像度)
    for (let i = 0; i < 格子.length; i++) 格子[i] = Math.sin(i * 0.37) * 5 + 3
    return 格子
}

function 草だけの材質(): Uint8Array {
    const 材質 = new Uint8Array(解像度 * 解像度 * 4)
    for (let i = 0; i < 解像度 * 解像度; i++) 材質[i * 4] = 255
    return 材質
}

describe('大升から地形を生成する計算', () => {
    it('大升の塗りが塗っていない大升の格子点を変えないこと', () => {
        const 元の高さ = 起伏のある格子()
        const 元の材質 = 草だけの材質()
        const 結果 = 大升から地形を生成する([{ 列: 1, 行: 1, 高さメートル: 20, 層: '岩' }], 一辺の升目数, 解像度, 元の高さ, 元の材質)
        for (let gz = 24; gz < 解像度; gz++) {
            for (let gx = 24; gx < 解像度; gx++) {
                const 添字 = gz * 解像度 + gx
                assert.strictEqual(結果.高さ[添字], 元の高さ[添字], `塗っていない大升(3,3)の格子点(${gx},${gz})の高さは変わらないべき`)
                for (let 層 = 0; 層 < 4; 層++) {
                    assert.strictEqual(結果.材質[添字 * 4 + 層], 元の材質[添字 * 4 + 層])
                }
            }
        }
        const 塗った中心 = (12 * 解像度 + 12)
        assert.ok(Math.abs((結果.高さ[塗った中心] ?? 0) - 20) < 1e-3, '塗った大升の中心は塗りの高さになるべき')
        assert.strictEqual(結果.材質[塗った中心 * 4 + 2], 255, '塗った大升の中心は岩100%になるべき')
    })

    it('大升の層の境界で重みの合計が255を保つこと', () => {
        const 結果 = 大升から地形を生成する(
            [{ 列: 1, 行: 1, 高さメートル: null, 層: '岩' }, { 列: 2, 行: 1, 高さメートル: null, 層: '砂' }],
            一辺の升目数,
            解像度,
            起伏のある格子(),
            草だけの材質(),
        )
        for (let i = 0; i < 解像度 * 解像度; i++) {
            const 合計 = (結果.材質[i * 4] ?? 0) + (結果.材質[i * 4 + 1] ?? 0) + (結果.材質[i * 4 + 2] ?? 0) + (結果.材質[i * 4 + 3] ?? 0)
            assert.strictEqual(合計, 255, `画素${i}の重みの合計は255であるべき`)
        }
        const 境界の画素 = 12 * 解像度 + 16
        assert.ok((結果.材質[境界の画素 * 4 + 2] ?? 0) > 0 && (結果.材質[境界の画素 * 4 + 3] ?? 0) > 0, '層の境界では岩と砂が混ざるべき')
    })

    it('一辺が升目数を割り切らないときは拒否されること', () => {
        assert.throws(() => 大升から地形を生成する([], 7, 解像度, 起伏のある格子(), 草だけの材質()), /割り切らない/)
    })
})

describe('高さ場から大升を導く計算', () => {
    it('大升ごとに格子点の平均高さと重み合計が最大の層になること', () => {
        const 高さ = new Float32Array(解像度 * 解像度).fill(7)
        const 材質 = 草だけの材質()
        for (let gz = 8; gz <= 16; gz++) {
            for (let gx = 8; gx <= 16; gx++) {
                材質[(gz * 解像度 + gx) * 4] = 0
                材質[(gz * 解像度 + gx) * 4 + 2] = 255
            }
        }
        const 塗り一覧 = 高さ場から大升を導く(一辺の升目数, 解像度, 高さ, 材質)
        assert.strictEqual(塗り一覧.length, 16)
        const 岩の大升 = 塗り一覧.find((塗り) => 塗り.列 === 1 && 塗り.行 === 1)
        assert.deepStrictEqual(岩の大升, { 列: 1, 行: 1, 高さメートル: 7, 層: '岩' })
        assert.strictEqual(塗り一覧.find((塗り) => 塗り.列 === 3 && 塗り.行 === 0)?.層, '草')
    })
})
