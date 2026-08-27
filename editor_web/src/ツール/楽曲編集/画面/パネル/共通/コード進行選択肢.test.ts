import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import {
    トラックの進行の選択肢一覧を組み立てる,
    パターンの進行の選択肢一覧を組み立てる,
    綴りから進行の選びを復元する,
} from './コード進行選択肢.ts'

describe('コード進行の選択肢のテスト', () => {
    it('トラックの選択肢は先頭が楽曲全体に従う項目であること', () => {
        const 選択肢一覧 = トラックの進行の選択肢一覧を組み立てる(null, [])
        const 先頭 = 選択肢一覧[0]
        assert.notEqual(先頭, undefined)
        assert.equal(先頭?.text, '楽曲全体の進行に従う')
        assert.equal(先頭?.selected, true)
    })

    it('パターンの選択肢には楽曲全体に従う項目が出ないこと', () => {
        const 選択肢一覧 = パターンの進行の選択肢一覧を組み立てる(
            { 種類: '既定の進行', 識別子: '戦闘と道' },
            [],
        )
        assert.equal(選択肢一覧.some((項目) => 項目.text === '楽曲全体の進行に従う'), false)
        assert.equal(選択肢一覧.filter((項目) => 項目.selected).length, 1)
    })

    it('組み立てた綴りが元の参照へ戻ること', () => {
        const 独自進行一覧 = [{ 名前: '独自:紛らわしい名前', 和音一覧: [] }]
        const 選択肢一覧 = トラックの進行の選択肢一覧を組み立てる(
            { 種類: '独自の進行', 名前: '独自:紛らわしい名前' },
            独自進行一覧,
        )
        for (const 項目 of 選択肢一覧) {
            const 選び = 綴りから進行の選びを復元する(項目.value)
            assert.equal(項目.selected, 選び.種類 === 'この進行を使う' && 選び.参照.種類 === '独自の進行')
        }
    })

    it('読めない綴りは既定値で埋めずに失敗すること', () => {
        assert.throws(() => 綴りから進行の選びを復元する(''))
        assert.throws(() => 綴りから進行の選びを復元する('でたらめ'))
    })
})
