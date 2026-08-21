import { describe, it, beforeEach, afterEach } from 'node:test'
import assert from 'node:assert/strict'
import { 実サーバー接続 } from './実サーバー接続.ts'
import type { チャンク構造 } from '../../生成/編集資源契約.ts'
import { モックFetchを差し込む, type モックFetch制御 } from './テスト補助/モックfetch.ts'

describe('実サーバー接続のbigint往復', () => {
    let 制御: モックFetch制御

    beforeEach(() => {
        制御 = モックFetchを差し込む()
    })

    afterEach(() => {
        制御.元のFetchへ戻す()
    })

    it('チャンク構造のPUTで2^53超えbigintが生数値文字列として送信され、GETで完全復元されること', async () => {
        const 超巨大種 = 18446744073709551615n
        const チャンク: チャンク構造 = {
            道路一覧: [{ 制御点列: [], 全幅メートル: 8, 散布除外バッファメートル: 14, 細分割数: 80 }],
            建物一覧: [],
            散布: { 最小間隔メートル: 5.5, 乱数の種: 超巨大種 },
            散布の個体一覧: [],
        }

        // PUT送信テスト
        制御.レスポンスを差し替える(new Response(null, { status: 204 }))
        const 接続 = new 実サーバー接続()
        const 保存結果 = await 接続.チャンクの構造を保存する({ x: 0, z: 0 }, チャンク)
        assert.strictEqual(保存結果.種別, '成功')
        assert.strictEqual(制御.最後の要求URLを得る(), '/api/チャンク/0/0/構造')
        const 送信本文 = 制御.最後の要求オプションを得る()?.body
        assert.ok(typeof 送信本文 === 'string')
        assert.ok(
            送信本文.includes(`"乱数の種":18446744073709551615`) ||
            送信本文.includes(`"乱数の種": 18446744073709551615`),
            '送信JSONに生数値リテラルが含まれること',
        )

        // GET受信テスト
        const 生サーバーJSON = `{"道路一覧":[{"制御点列":[],"全幅メートル":8,"散布除外バッファメートル":14,"細分割数":80}],"建物一覧":[],"散布":{"最小間隔メートル":5.5,"乱数の種":18446744073709551615},"散布の個体一覧":[]}`
        制御.レスポンスを差し替える(new Response(生サーバーJSON, { status: 200 }))
        const 読込結果 = await 接続.チャンクの構造を読む({ x: 0, z: 0 })
        assert.strictEqual(読込結果.種別, '成功')
        if (読込結果.種別 === '成功') {
            assert.strictEqual(読込結果.値.散布.乱数の種, 超巨大種, '乱数の種が1ビットも落ちずに復元されること')
        }
    })
})
