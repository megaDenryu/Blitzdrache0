import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import { 楽曲IDを生成する } from '../境界/index.ts'
import { 楽曲IDを発番する, 新しい楽曲の既定の表示名 } from './楽曲IDを発番する.ts'

describe('楽曲ID発番の単体テスト', () => {
    it('一覧が空のときはsong-1を発番すること', () => {
        assert.strictEqual(楽曲IDを発番する([]), 楽曲IDを生成する('song-1'))
    })

    it('既存の番号の次の番号を発番すること', () => {
        const 既にある = [楽曲IDを生成する('song-1'), 楽曲IDを生成する('song-2')]
        assert.strictEqual(楽曲IDを発番する(既にある), 楽曲IDを生成する('song-3'))
    })

    it('連番に歯抜けがある場合は空いている最小番号を埋めること', () => {
        const 既にある = [楽曲IDを生成する('song-1'), 楽曲IDを生成する('song-3')]
        assert.strictEqual(楽曲IDを発番する(既にある), 楽曲IDを生成する('song-2'))
    })

    it('既定の表示名が返ること', () => {
        assert.strictEqual(新しい楽曲の既定の表示名(), '新しい楽曲')
    })
})
