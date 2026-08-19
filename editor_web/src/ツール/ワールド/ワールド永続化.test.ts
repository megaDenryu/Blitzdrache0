import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import type { 大域世界構造, チャンク座標, チャンク構造 } from '../../生成/編集資源契約.ts'
import type { プロジェクト保管庫接続, 読込結果, 保存結果 } from '../../境界/通信/index.ts'
import { 読込成功, 読込無し, 保存成功, 保存状態サービス, 状態通知付き保管庫接続 } from '../../境界/通信/index.ts'
import { ワールド状態を保管庫から読み込む, ワールド状態を保管庫へ保存する } from './ワールド永続化.ts'

class 偽保管庫 implements プロジェクト保管庫接続 {
    public 大域構造: 大域世界構造 | null = null
    public 保存呼出順序: string[] = []

    public async 大域世界の構造を読む(): Promise<読込結果<大域世界構造>> {
        return this.大域構造 !== null ? 読込成功(this.大域構造) : 読込無し()
    }
    public async 大域世界の構造を保存する(構造: 大域世界構造): Promise<保存結果> {
        this.大域構造 = 構造
        this.保存呼出順序.push('大域世界構造')
        return 保存成功()
    }
    public async 大域世界の高さ格子を読む(): Promise<読込結果<ArrayBufferLike>> { return 読込無し() }
    public async 大域世界の高さ格子を保存する(): Promise<保存結果> {
        this.保存呼出順序.push('大域世界高さ格子')
        return 保存成功()
    }
    public async チャンクの構造を読む(): Promise<読込結果<チャンク構造>> { return 読込無し() }
    public async チャンクの構造を保存する(): Promise<保存結果> {
        this.保存呼出順序.push('チャンク構造')
        return 保存成功()
    }
    public async チャンクの高さ格子を読む(): Promise<読込結果<ArrayBufferLike>> { return 読込無し() }
    public async チャンクの高さ格子を保存する(): Promise<保存結果> {
        this.保存呼出順序.push('チャンク高さ格子')
        return 保存成功()
    }
    public async チャンクの材質重みを読む(): Promise<読込結果<ArrayBufferLike>> { return 読込無し() }
    public async チャンクの材質重みを保存する(): Promise<保存結果> {
        this.保存呼出順序.push('チャンク材質重み')
        return 保存成功()
    }
}

describe('ワールド永続化の大域世界構造整合テスト', () => {
    it('保存済みの大域世界構造があるとき、その区画割りに基づく頂点数のチャンクが生成されること', async () => {
        const 保管庫 = new 偽保管庫()
        保管庫.大域構造 = {
            区画割り: { 一辺のメートル: 512, 軸あたりチャンク数: 2, チャンクあたり格子解像度: 64 },
            広域道路: { 制御点列: [], 全幅メートル: 8, 細分割数: 80 },
        }
        const 座標: チャンク座標 = { x: 0, z: 0 }
        const { 状態, 結果種別 } = await ワールド状態を保管庫から読み込む(保管庫, 座標)
        assert.strictEqual(結果種別, '読込成功')
        const チャンク = 状態.チャンクを取得する(座標)
        assert.strictEqual(チャンク.高さ場.解像度, 65, '保存済み区画割りの解像度+1になること')
    })

    it('チャンク保存時に大域構造と高さ格子が先に保存され、通知が大域リスナーへ届くこと', async () => {
        const 偽 = new 偽保管庫()
        const サービス = new 保存状態サービス()
        const 保管庫 = new 状態通知付き保管庫接続(偽, サービス)
        const 座標: チャンク座標 = { x: 0, z: 0 }
        const { 状態 } = await ワールド状態を保管庫から読み込む(保管庫, 座標)

        assert.strictEqual(サービス.大域状態を取得する().文言, '未保存(初期生成)')
        await ワールド状態を保管庫へ保存する(保管庫, 状態)

        assert.strictEqual(サービス.大域状態を取得する().文言, '保存完了', '大域状態が保存完了へ更新されること')
        assert.strictEqual(サービス.チャンク状態を取得する(座標).文言, '保存完了')
        assert.deepStrictEqual(偽.保存呼出順序, ['大域世界構造', '大域世界高さ格子', 'チャンク構造', 'チャンク高さ格子', 'チャンク材質重み'])
    })
})
