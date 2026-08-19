import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import type { 大域世界構造, チャンク座標, チャンク構造 } from '../../生成/編集資源契約.ts'
import { 読込成功, 読込無し, 保存成功 } from './index.ts'
import { 保存状態サービス } from './保存状態サービス.ts'
import { 状態通知付き保管庫接続 } from './状態通知付き保管庫接続.ts'

class 偽保管庫 implements プロジェクト保管庫接続 {
    public 大域構造: 大域世界構造 | null = null
    public チャンク構造マップ = new Map<string, チャンク構造>()

    public async 大域世界の構造を読む(): Promise<読込結果<大域世界構造>> {
        return this.大域構造 !== null ? 読込成功(this.大域構造) : 読込無し()
    }
    public async 大域世界の構造を保存する(構造: 大域世界構造): Promise<保存結果> {
        this.大域構造 = 構造
        return 保存成功()
    }
    public async 大域世界の高さ格子を読む(): Promise<読込結果<ArrayBufferLike>> { return 読込無し() }
    public async 大域世界の高さ格子を保存する(): Promise<保存結果> { return 保存成功() }
    public async チャンクの構造を読む(座標: チャンク座標): Promise<読込結果<チャンク構造>> {
        const キー = `${座標.x},${座標.z}`
        const 構造 = this.チャンク構造マップ.get(キー)
        return 構造 !== undefined ? 読込成功(構造) : 読込無し()
    }
    public async チャンクの構造を保存する(座標: チャンク座標, 構造: チャンク構造): Promise<保存結果> {
        const キー = `${座標.x},${座標.z}`
        this.チャンク構造マップ.set(キー, 構造)
        return 保存成功()
    }
    public async チャンクの高さ格子を読む(): Promise<読込結果<ArrayBufferLike>> { return 読込無し() }
    public async チャンクの高さ格子を保存する(): Promise<保存結果> { return 保存成功() }
    public async チャンクの材質重みを読む(): Promise<読込結果<ArrayBufferLike>> { return 読込無し() }
    public async チャンクの材質重みを保存する(): Promise<保存結果> { return 保存成功() }
}

describe('保存状態サービスと状態通知付き保管庫接続のテスト', () => {
    it('大域状態とチャンク状態の購読・更新・購読解除が正しく動作すること', () => {
        const サービス = new 保存状態サービス()
        const 座標: チャンク座標 = { x: 1, z: 2 }
        const 大域通知履歴: string[] = []
        const チャンク通知履歴: string[] = []

        const 大域解除 = サービス.大域状態を購読する((状態) => { 大域通知履歴.push(状態.文言) })
        const チャンク解除 = サービス.チャンク状態を購読する(座標, (状態) => { チャンク通知履歴.push(状態.文言) })

        assert.strictEqual(大域通知履歴[0], '未保存(初期生成)')
        assert.strictEqual(チャンク通知履歴[0], '未保存(初期生成)')

        サービス.大域状態を更新する('保存完了', false)
        サービス.チャンク状態を更新する(座標, '保存完了', false)

        assert.strictEqual(大域通知履歴[1], '保存完了')
        assert.strictEqual(チャンク通知履歴[1], '保存完了')

        大域解除()
        チャンク解除()
        サービス.大域状態を更新する('読込完了(起動時)', false)
        assert.strictEqual(大域通知履歴.length, 2, '解除後は通知されないこと')
    })

    it('状態通知付き保管庫接続経由の保存と読込で通知が自動発行されること', async () => {
        const 偽 = new 偽保管庫()
        const サービス = new 保存状態サービス()
        const 保管庫 = new 状態通知付き保管庫接続(偽, サービス)
        const 対象座標: チャンク座標 = { x: 0, z: 0 }

        const 大域構造: 大域世界構造 = {
            区画割り: { 一辺のメートル: 1024, 軸あたりチャンク数: 4, チャンクあたり格子解像度: 128 },
            広域道路: { 制御点列: [], 全幅メートル: 10, 細分割数: 100 },
        }
        await 保管庫.大域世界の構造を保存する(大域構造)
        assert.strictEqual(サービス.大域状態を取得する().文言, '保存完了')

        const チャンク: チャンク構造 = {
            座標: 対象座標, 道路: { ノード列: [], エッジ列: [] }, 建物: { 建物一覧: [] }, 散布: { 散布一覧: [] },
        }
        await 保管庫.チャンクの構造を保存する(対象座標, チャンク)
        assert.strictEqual(サービス.チャンク状態を取得する(対象座標).文言, '保存完了')
    })
})
