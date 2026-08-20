import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import { 地表材質 } from './地表材質.ts'
import { 合計255へ正規化する } from './地表材質正規化.ts'

describe('地表材質編集モデル', () => {
    it('正規化関数が任意の入力に対し合計厳密に255を返すこと', () => {
        const テスト入力一覧: Array<[number, number, number, number]> = [
            [0, 0, 0, 0],
            [255, 0, 0, 0],
            [100, 100, 100, 100],
            [1, 1, 1, 1],
            [10, 20, 30, 40],
            [33, 33, 33, 33],
            [1000, 50, 0, 12],
        ]
        for (const [草, 泥, 岩, 砂] of テスト入力一覧) {
            const [r, g, b, a] = 合計255へ正規化する(草, 泥, 岩, 砂)
            const 合計 = r + g + b + a
            assert.strictEqual(合計, 255, `合計が255であるべき: (${草},${泥},${岩},${砂}) => (${r},${g},${b},${a})`)
        }
    })

    it('材質筆致適用後も全画素で合計255の不変条件が保たれること', () => {
        const 対象材質 = new 地表材質(16, 32)
        対象材質.材質筆致を適用する({
            層: '泥',
            通過点列: [{ x: 0, y: 0, z: 0 }],
            半径メートル: 10,
            流量: 0.8,
        })
        for (let i = 0; i < 16 * 16; i++) {
            const r = 対象材質.材質データ[i * 4 + 0] ?? 0
            const g = 対象材質.材質データ[i * 4 + 1] ?? 0
            const b = 対象材質.材質データ[i * 4 + 2] ?? 0
            const a = 対象材質.材質データ[i * 4 + 3] ?? 0
            const 合計 = r + g + b + a
            assert.strictEqual(合計, 255, `画素 ${i} の合計が255であるべき: 実際=${合計}`)
        }
    })
})
