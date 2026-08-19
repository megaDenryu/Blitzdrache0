import { describe, it, beforeEach, afterEach } from 'node:test'
import assert from 'node:assert/strict'
import { 実サーバー接続 } from './実サーバー接続.ts'
import type { 大域世界構造 } from '../../生成/編集資源契約.ts'
import { モックFetchを差し込む, type モックFetch制御 } from './テスト補助/モックfetch.ts'

describe('実サーバー接続の読込と保存の基本応答', () => {
    let 制御: モックFetch制御

    beforeEach(() => {
        制御 = モックFetchを差し込む()
    })

    afterEach(() => {
        制御.元のFetchへ戻す()
    })

    it('大域世界の構造が200+nullのときは「無し」と判定されること', async () => {
        制御.レスポンスを差し替える(new Response('null', { status: 200 }))
        const 接続 = new 実サーバー接続()
        const 結果 = await 接続.大域世界の構造を読む()

        assert.strictEqual(結果.種別, '無し')
        assert.strictEqual(制御.最後の要求URLを得る(), '/api/大域世界/構造')
    })

    it('大域世界高さ格子が204のときは「無し」と判定されること', async () => {
        制御.レスポンスを差し替える(new Response(null, { status: 204 }))
        const 接続 = new 実サーバー接続('http://127.0.0.1:7901')
        const 結果 = await 接続.大域世界の高さ格子を読む()

        assert.strictEqual(結果.種別, '無し')
        assert.strictEqual(制御.最後の要求URLを得る(), 'http://127.0.0.1:7901/api/大域世界/高さ格子')
    })

    it('大域世界構造の保存で422のときエラー応答が抽出されること', async () => {
        制御.レスポンスを差し替える(
            new Response(
                JSON.stringify({ 種別: '検証失敗', 説明: '解像度が不正' }),
                { status: 422, headers: { 'Content-Type': 'application/json' } },
            ),
        )
        const 接続 = new 実サーバー接続()
        const ダミー構造: 大域世界構造 = {
            区画割り: { 一辺のメートル: 1024, 軸あたりチャンク数: 4, チャンクあたり格子解像度: 128 },
            広域道路: { 制御点列: [], 全幅メートル: 12, 細分割数: 100 },
        }
        const 結果 = await 接続.大域世界の構造を保存する(ダミー構造)

        assert.strictEqual(結果.種別, '失敗')
        if (結果.種別 === '失敗') {
            assert.strictEqual(結果.エラー.種別, '検証失敗')
            assert.strictEqual(結果.エラー.説明, '解像度が不正')
        }
    })
})
