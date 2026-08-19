import { describe, it, beforeEach, afterEach } from 'node:test'
import assert from 'node:assert/strict'
import { 実サーバー接続 } from './実サーバー接続.ts'
import { モックFetchを差し込む, type モックFetch制御 } from './テスト補助/モックfetch.ts'

describe('実サーバー接続の書き出し要求', () => {
    let 制御: モックFetch制御

    beforeEach(() => {
        制御 = モックFetchを差し込む()
    })

    afterEach(() => {
        制御.元のFetchへ戻す()
    })

    it('ソースアセットの書き出しが成功したら書いたファイル数と出力先を返すこと', async () => {
        制御.レスポンスを差し替える(
            new Response(
                JSON.stringify({ 書いたファイル数: 12, 出力先: 'assets/my_world' }),
                { status: 200, headers: { 'Content-Type': 'application/json' } },
            ),
        )
        const 接続 = new 実サーバー接続()
        const 結果 = await 接続.ソースアセットへ書き出す('my_world')

        assert.strictEqual(結果.種別, '成功')
        if (結果.種別 === '成功') {
            assert.strictEqual(結果.書いたファイル数, 12)
            assert.strictEqual(結果.出力先, 'assets/my_world')
        }
        assert.strictEqual(制御.最後の要求URLを得る(), '/api/書き出し/ソースアセット')
        assert.strictEqual(制御.最後の要求オプションを得る()?.method, 'POST')
        assert.strictEqual(制御.最後の要求オプションを得る()?.body, JSON.stringify({ 出力先の世界名: 'my_world' }))
    })

    it('世界名を省略したら要求本体が空オブジェクトになること', async () => {
        制御.レスポンスを差し替える(
            new Response(
                JSON.stringify({ 書いたファイル数: 3, 出力先: 'assets/editor_world' }),
                { status: 200, headers: { 'Content-Type': 'application/json' } },
            ),
        )
        const 接続 = new 実サーバー接続()
        await 接続.ソースアセットへ書き出す()

        assert.strictEqual(制御.最後の要求オプションを得る()?.body, JSON.stringify({}))
    })

    it('ソースアセットの書き出しが422で失敗したらエラー種別と説明を返すこと', async () => {
        制御.レスポンスを差し替える(
            new Response(
                JSON.stringify({ 種別: '前提条件エラー', 説明: '大域世界が未保存' }),
                { status: 422, headers: { 'Content-Type': 'application/json' } },
            ),
        )
        const 接続 = new 実サーバー接続()
        const 結果 = await 接続.ソースアセットへ書き出す()

        assert.strictEqual(結果.種別, '失敗')
        if (結果.種別 === '失敗') {
            assert.strictEqual(結果.エラー.種別, '前提条件エラー')
            assert.strictEqual(結果.エラー.説明, '大域世界が未保存')
        }
    })
})
