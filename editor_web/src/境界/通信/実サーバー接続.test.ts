import { describe, it, beforeEach, afterEach } from 'node:test'
import assert from 'node:assert/strict'
import { 実サーバー接続 } from './実サーバー接続.ts'
import type { 大域世界構造, チャンク構造 } from '../../生成/編集資源契約.ts'

describe('実サーバー接続の通信とbigint往復テスト', () => {
    const 元のFetch = globalThis.fetch
    let 最後の要求URL = ''
    let 最後の要求オプション: RequestInit | undefined
    let モックレスポンス: Response = new Response('null', { status: 200 })

    beforeEach(() => {
        globalThis.fetch = async (input: RequestInfo | URL, init?: RequestInit): Promise<Response> => {
            最後の要求URL = String(input)
            最後の要求オプション = init
            return モックレスポンス
        }
    })

    afterEach(() => {
        globalThis.fetch = 元のFetch
    })

    it('大域世界の構造が200+nullのときは「無し」と判定されること', async () => {
        モックレスポンス = new Response('null', { status: 200 })
        const 接続 = new 実サーバー接続()
        const 結果 = await 接続.大域世界の構造を読む()

        assert.strictEqual(結果.種別, '無し')
        assert.strictEqual(最後の要求URL, '/api/大域世界/構造')
    })

    it('大域世界高さ格子が204のときは「無し」と判定されること', async () => {
        モックレスポンス = new Response(null, { status: 204 })
        const 接続 = new 実サーバー接続('http://127.0.0.1:7901')
        const 結果 = await 接続.大域世界の高さ格子を読む()

        assert.strictEqual(結果.種別, '無し')
        assert.strictEqual(最後の要求URL, 'http://127.0.0.1:7901/api/大域世界/高さ格子')
    })

    it('大域世界構造の保存で422のときエラー応答が抽出されること', async () => {
        モックレスポンス = new Response(
            JSON.stringify({ 種別: '検証失敗', 説明: '解像度が不正' }),
            { status: 422, headers: { 'Content-Type': 'application/json' } },
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

    it('チャンク構造のPUTで2^53超えbigintが生数値文字列として送信され、GETで完全復元されること', async () => {
        const 超巨大種 = 18446744073709551615n
        const チャンク: チャンク構造 = {
            道路: { 制御点列: [], 全幅メートル: 8, 散布除外バッファメートル: 14, 細分割数: 80 },
            建物一覧: [],
            散布: { 最小間隔メートル: 5.5, 乱数の種: 超巨大種 },
        }

        // PUT送信テスト
        モックレスポンス = new Response(null, { status: 204 })
        const 接続 = new 実サーバー接続()
        const 保存結果 = await 接続.チャンクの構造を保存する({ x: 0, z: 0 }, チャンク)
        assert.strictEqual(保存結果.種別, '成功')
        assert.strictEqual(最後の要求URL, '/api/チャンク/0/0/構造')
        const 送信本文 = 最後の要求オプション?.body
        assert.ok(typeof 送信本文 === 'string')
        assert.ok(
            送信本文.includes(`"乱数の種":18446744073709551615`) ||
            送信本文.includes(`"乱数の種": 18446744073709551615`),
            '送信JSONに生数値リテラルが含まれること',
        )

        // GET受信テスト
        const 生サーバーJSON = `{"道路":{"制御点列":[],"全幅メートル":8,"散布除外バッファメートル":14,"細分割数":80},"建物一覧":[],"散布":{"最小間隔メートル":5.5,"乱数の種":18446744073709551615}}`
        モックレスポンス = new Response(生サーバーJSON, { status: 200 })
        const 読込結果 = await 接続.チャンクの構造を読む({ x: 0, z: 0 })
        assert.strictEqual(読込結果.種別, '成功')
        if (読込結果.種別 === '成功') {
            assert.strictEqual(読込結果.値.散布.乱数の種, 超巨大種, '乱数の種が1ビットも落ちずに復元されること')
        }
    })

    it('ソースアセットの書き出しが成功したら書いたファイル数と出力先を返すこと', async () => {
        モックレスポンス = new Response(
            JSON.stringify({ 書いたファイル数: 12, 出力先: 'assets/my_world' }),
            { status: 200, headers: { 'Content-Type': 'application/json' } },
        )
        const 接続 = new 実サーバー接続()
        const 結果 = await 接続.ソースアセットへ書き出す('my_world')

        assert.strictEqual(結果.種別, '成功')
        if (結果.種別 === '成功') {
            assert.strictEqual(結果.書いたファイル数, 12)
            assert.strictEqual(結果.出力先, 'assets/my_world')
        }
        assert.strictEqual(最後の要求URL, '/api/書き出し/ソースアセット')
        assert.strictEqual(最後の要求オプション?.method, 'POST')
        assert.strictEqual(最後の要求オプション?.body, JSON.stringify({ 出力先の世界名: 'my_world' }))
    })

    it('世界名を省略したら要求本体が空オブジェクトになること', async () => {
        モックレスポンス = new Response(
            JSON.stringify({ 書いたファイル数: 3, 出力先: 'assets/editor_world' }),
            { status: 200, headers: { 'Content-Type': 'application/json' } },
        )
        const 接続 = new 実サーバー接続()
        await 接続.ソースアセットへ書き出す()

        assert.strictEqual(最後の要求オプション?.body, JSON.stringify({}))
    })

    it('ソースアセットの書き出しが422で失敗したらエラー種別と説明を返すこと', async () => {
        モックレスポンス = new Response(
            JSON.stringify({ 種別: '前提条件エラー', 説明: '大域世界が未保存' }),
            { status: 422, headers: { 'Content-Type': 'application/json' } },
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
