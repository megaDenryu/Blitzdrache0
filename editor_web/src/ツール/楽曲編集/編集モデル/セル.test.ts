import assert from 'node:assert/strict'
import { describe, it } from 'node:test'
import {
    セルを数値へ変換する,
    数値からセルへ変換する,
    type セル,
} from './セル.ts'

describe('セルの相互変換と判別共用体のテスト', () => {
    it('0〜4の各数値と判別共用体が相互に完全一致すること', () => {
        const 対象一覧: Array<{ readonly 数値: number, readonly セル: セル }> = [
            { 数値: 0, セル: { 種類: '打点なし' } },
            { 数値: 1, セル: { 種類: '音の始まり', 進行に従うか: true } },
            { 数値: 2, セル: { 種類: '音の継続', 進行に従うか: true } },
            { 数値: 3, セル: { 種類: '音の始まり', 進行に従うか: false } },
            { 数値: 4, セル: { 種類: '音の継続', 進行に従うか: false } },
        ]
        for (const { 数値, セル } of 対象一覧) {
            assert.deepEqual(数値からセルへ変換する(数値), セル)
            assert.equal(セルを数値へ変換する(セル), 数値)
        }
    })

    it('範囲外の数値は明示の失敗（例外）になること', () => {
        const 不正値一覧 = [-1, 5, 10, 1.5, NaN]
        for (const 不正値 of 不正値一覧) {
            assert.throws(() => 数値からセルへ変換する(不正値))
        }
    })

    it('判別共用体の種類で音の始まり・継続・打点なしが識別できること', () => {
        const 打点なしセル: セル = { 種類: '打点なし' }
        const 音始まりセル: セル = { 種類: '音の始まり', 進行に従うか: true }
        const 音継続セル: セル = { 種類: '音の継続', 進行に従うか: false }

        assert.equal(打点なしセル.種類 === '音の始まり', false)
        assert.equal(音始まりセル.種類 === '音の始まり', true)
        assert.equal(音継続セル.種類 === '音の継続', true)
        assert.equal(音始まりセル.種類 === '音の継続', false)
    })
})
