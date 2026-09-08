import assert from 'node:assert/strict'
import { describe, it } from 'node:test'
import type { 曲の節 } from '../../../../生成/編集資源契約.ts'
import { 曲構成をカードの列へ展開する } from '../../編集モデル/index.ts'
import { カード列を節ごとに切る } from './カード列を節ごとに切る.ts'

describe('カード列を節ごとに切るテスト', () => {
    it('空のカード列なら空の列を返すこと', () => {
        assert.deepEqual(カード列を節ごとに切る([]), [])
    })

    it('A×2, B×1の曲構成を、節ごとの2つのグループへ切ること', () => {
        const 曲構成: 曲の節[] = [
            { パターンの名乗り: 'A', 繰り返し回数: 2 },
            { パターンの名乗り: 'B', 繰り返し回数: 1 },
        ]
        const グループ列 = カード列を節ごとに切る(曲構成をカードの列へ展開する(曲構成))

        assert.equal(グループ列.length, 2)

        assert.equal(グループ列[0].節の位置, 0)
        assert.equal(グループ列[0].パターンの名乗り, 'A')
        assert.equal(グループ列[0].カード列.length, 2)
        assert.deepEqual(グループ列[0].先頭カードの位置, { 節の位置: 0, 繰り返しの何回目: 0 })

        assert.equal(グループ列[1].節の位置, 1)
        assert.equal(グループ列[1].パターンの名乗り, 'B')
        assert.equal(グループ列[1].カード列.length, 1)
        assert.deepEqual(グループ列[1].先頭カードの位置, { 節の位置: 1, 繰り返しの何回目: 0 })
    })

    it('A×1, B×1, A×1のように同じパターンが離れて並んでも、節の位置ごとに別グループへ切ること', () => {
        const 曲構成: 曲の節[] = [
            { パターンの名乗り: 'A', 繰り返し回数: 1 },
            { パターンの名乗り: 'B', 繰り返し回数: 1 },
            { パターンの名乗り: 'A', 繰り返し回数: 1 },
        ]
        const グループ列 = カード列を節ごとに切る(曲構成をカードの列へ展開する(曲構成))

        assert.equal(グループ列.length, 3)
        assert.deepEqual(グループ列.map((グループ) => グループ.節の位置), [0, 1, 2])
    })
})
