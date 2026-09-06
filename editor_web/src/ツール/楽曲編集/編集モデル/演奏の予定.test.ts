import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import { 初期楽曲を生成する } from './初期楽曲生成.ts'
import { パターンのステップ数を求める } from './パターンの長さ.ts'
import { パターンのステップで鳴り始める音一覧を求める, 升目の音を組み立てる } from './演奏の予定.ts'

function 空の見本の楽曲() {
    return 初期楽曲を生成する('song-1', '見本')
}

describe('演奏の予定', () => {
    it('打点の無いステップでは1つも鳴り始めないこと', () => {
        const 楽曲 = 空の見本の楽曲()
        const パターン = 楽曲.パターン一覧[0]
        for (let ステップ = 0; ステップ < パターンのステップ数を求める(パターン); ステップ++) {
            assert.strictEqual(パターンのステップで鳴り始める音一覧を求める(楽曲, パターン, ステップ).length, 0)
        }
    })

    it('打点に続く継続の数が、鳴らす長さのステップ数になること', () => {
        const 楽曲 = 空の見本の楽曲()
        const パターン = 楽曲.パターン一覧[0]
        パターン.格子[0].行一覧[0][4] = 1
        パターン.格子[0].行一覧[0][5] = 2
        パターン.格子[0].行一覧[0][6] = 2
        const 音一覧 = パターンのステップで鳴り始める音一覧を求める(楽曲, パターン, 4)
        assert.strictEqual(音一覧.length, 1)
        assert.strictEqual(音一覧[0].種類, '音高の音')
        assert.strictEqual(音一覧[0].長さのステップ数, 3)
        assert.strictEqual(パターンのステップで鳴り始める音一覧を求める(楽曲, パターン, 5).length, 0)
    })

    it('打楽器のトラックからは打楽器の音が取り出されること', () => {
        const 楽曲 = 空の見本の楽曲()
        const パターン = 楽曲.パターン一覧[0]
        パターン.格子[3].行一覧[2][0] = 1
        const 音一覧 = パターンのステップで鳴り始める音一覧を求める(楽曲, パターン, 0)
        assert.strictEqual(音一覧.length, 1)
        const 音 = 音一覧[0]
        assert.strictEqual(音.種類, '打楽器の音')
        if (音.種類 !== '打楽器の音') return
        assert.strictEqual(音.打楽器, 'バスドラム')
        assert.strictEqual(音.楽器, '生ドラム')
    })

    it('升目の音は、その行の音を1ステップ分の長さで鳴らすものになること', () => {
        const 楽曲 = 空の見本の楽曲()
        const 音 = 升目の音を組み立てる(楽曲, 0, 0)
        assert.strictEqual(音.種類, '音高の音')
        if (音.種類 !== '音高の音') return
        assert.strictEqual(音.音高番号, 76)
        assert.strictEqual(音.長さのステップ数, 1)
        assert.strictEqual(音.楽器, 'グランドピアノ')
    })

    it('存在しないトラックや行を指したら明示の失敗になること', () => {
        const 楽曲 = 空の見本の楽曲()
        assert.throws(() => 升目の音を組み立てる(楽曲, 99, 0))
        assert.throws(() => 升目の音を組み立てる(楽曲, 0, 99))
    })
})
