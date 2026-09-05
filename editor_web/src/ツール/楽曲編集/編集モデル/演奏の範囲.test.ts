import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import { 初期楽曲を生成する } from './初期楽曲生成.ts'
import { 曲のステップ位置を解決する } from './曲構成の計算.ts'
import { 演奏に使う曲構成, 演奏が1周する長さ } from './演奏の範囲.ts'

describe('演奏の範囲', () => {
    it('パターンの繰り返しを選ぶと、曲構成があっても使わないこと', () => {
        const 楽曲 = 初期楽曲を生成する('song-1', '見本')
        楽曲.曲構成 = [{ パターンの名乗り: 'pattern-1', 繰り返し回数: 4 }]
        assert.deepEqual(演奏に使う曲構成('パターンの繰り返し', 楽曲), [])
        assert.strictEqual(演奏が1周する長さ('パターンの繰り返し', 楽曲, 'pattern-1').数値(), 32)
    })

    it('曲構成が空のときは、曲構成のとおりを選んでも選択中のパターンだけを繰り返すこと', () => {
        const 楽曲 = 初期楽曲を生成する('song-1', '見本')
        assert.deepEqual(楽曲.曲構成, [])
        assert.strictEqual(演奏が1周する長さ('曲構成のとおり', 楽曲, 'pattern-1').数値(), 32)
        const 曲構成 = 演奏に使う曲構成('曲構成のとおり', 楽曲)
        for (const 通しステップ of [0, 5, 31, 32, 33, 96]) {
            const 位置 = 曲のステップ位置を解決する(曲構成, 楽曲.パターン一覧, 通しステップ, 'pattern-1')
            assert.strictEqual(位置.パターンの名乗り, 'pattern-1')
            assert.strictEqual(位置.パターン内ステップ, 通しステップ % 32)
        }
    })

    it('曲構成が空で開いているパターンが無いときは、演奏が1周する長さが明示の失敗になること', () => {
        const 楽曲 = 初期楽曲を生成する('song-1', '見本')
        assert.throws(() => 演奏が1周する長さ('曲構成のとおり', 楽曲, null))
    })

    it('曲構成のとおりを選ぶと、節の繰り返し回数ぶんの長さになること', () => {
        const 楽曲 = 初期楽曲を生成する('song-1', '見本')
        楽曲.曲構成 = [
            { パターンの名乗り: 'pattern-1', 繰り返し回数: 2 },
            { パターンの名乗り: 'pattern-1', 繰り返し回数: 3 },
        ]
        assert.strictEqual(演奏が1周する長さ('曲構成のとおり', 楽曲, 'pattern-1').数値(), 32 * 5)
    })
})
