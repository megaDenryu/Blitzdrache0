import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import {
    世界の編集領域,
    建物の編集領域,
    楽曲の編集領域,
    編集領域の名乗り,
    編集領域の名乗り一覧,
    起動時に選ばれる編集領域,
} from './編集領域の名乗り.ts'

describe('編集領域の登録', () => {
    it('世界・建物・楽曲の3つが登録されていること', () => {
        assert.deepStrictEqual(
            編集領域の名乗り一覧.map((名乗り) => 名乗り.表示名),
            ['世界', '建物', '楽曲'],
        )
    })

    it('3つの識別子が互いに異なること', () => {
        const 識別子一覧 = 編集領域の名乗り一覧.map((名乗り) => String(名乗り.識別子))
        assert.strictEqual(new Set(識別子一覧).size, 3)
    })

    it('どの領域もアイコンの文字を1つ持つこと', () => {
        for (const 名乗り of 編集領域の名乗り一覧) {
            assert.strictEqual(名乗り.アイコンの文字.length, 1)
        }
    })

    it('起動時に選ばれる領域が世界であり、並びの先頭と一致すること', () => {
        assert.strictEqual(起動時に選ばれる編集領域, 世界の編集領域)
        assert.strictEqual(編集領域の名乗り一覧[0], 世界の編集領域)
    })

    it('建物と楽曲が世界とは別の領域であること', () => {
        assert.notStrictEqual(建物の編集領域.識別子, 世界の編集領域.識別子)
        assert.notStrictEqual(楽曲の編集領域.識別子, 世界の編集領域.識別子)
    })

    it('空の識別子・表示名・アイコンの文字を拒むこと', () => {
        assert.throws(() => 編集領域の名乗り.生成する('', '世界', '世'))
        assert.throws(() => 編集領域の名乗り.生成する('world', '', '世'))
        assert.throws(() => 編集領域の名乗り.生成する('world', '世界', ''))
    })
})
