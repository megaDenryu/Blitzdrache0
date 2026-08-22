import { describe, it } from 'node:test'
import assert from 'node:assert/strict'
import type { 大域世界構造, チャンク座標, チャンク構造 } from '../../../../生成/編集資源契約.ts'
import type { プロジェクト保管庫接続, 読込結果, 保存結果 } from '../../../../境界/通信/index.ts'
import { 読込成功, 読込無し, 保存成功 } from '../../../../境界/通信/index.ts'
import { ワールド編集状態 } from '../../編集モデル/index.ts'
import { チャンクを読み込んで登録する } from './チャンク登録.ts'

const 大域構造: 大域世界構造 = {
    区画割り: { 一辺のメートル: 512, 軸あたりチャンク数: 2, チャンクあたり格子解像度: 4 },
    広域道路一覧: [{ 制御点列: [], 全幅メートル: 8, 細分割数: 80 }],
}
const チャンク一辺頂点数 = 大域構造.区画割り.チャンクあたり格子解像度 + 1

// チャンクの構造をまだ持たず、高さ格子だけを配る保管庫。サーバーが大域のマザーハイトマップから
// 切り出した初期値を配る状況の代役である。
class 高さ格子だけ配る保管庫 implements プロジェクト保管庫接続 {
    public constructor(private readonly _配る高さ: ArrayBufferLike | null) {}

    public async 大域世界の構造を読む(): Promise<読込結果<大域世界構造>> { return 読込成功(大域構造) }
    public async 大域世界の構造を保存する(): Promise<保存結果> { return 保存成功() }
    public async 大域世界の高さ格子を読む(): Promise<読込結果<ArrayBufferLike>> { return 読込無し() }
    public async 大域世界の高さ格子を保存する(): Promise<保存結果> { return 保存成功() }
    public async チャンクの構造を読む(): Promise<読込結果<チャンク構造>> { return 読込無し() }
    public async チャンクの構造を保存する(): Promise<保存結果> { return 保存成功() }
    public async チャンクの高さ格子を読む(): Promise<読込結果<ArrayBufferLike>> {
        return this._配る高さ !== null ? 読込成功(this._配る高さ) : 読込無し()
    }
    public async チャンクの高さ格子を保存する(): Promise<保存結果> { return 保存成功() }
    public async チャンクの材質重みを読む(): Promise<読込結果<ArrayBufferLike>> { return 読込無し() }
    public async チャンクの材質重みを保存する(): Promise<保存結果> { return 保存成功() }
}

describe('チャンク登録の初期高さテスト', () => {
    it('構造が未保存でも保管庫が高さ格子を配るとき、その高さが初期の高さになること', async () => {
        const 配る高さ = new Float32Array(チャンク一辺頂点数 * チャンク一辺頂点数)
        for (let i = 0; i < 配る高さ.length; i++) {
            配る高さ[i] = i * 0.5
        }
        const 状態 = new ワールド編集状態(大域構造)
        const 座標: チャンク座標 = { x: 1, z: 0 }

        await チャンクを読み込んで登録する(new 高さ格子だけ配る保管庫(配る高さ.buffer), 状態, 座標)

        assert.deepStrictEqual(
            Array.from(状態.チャンクを取得する(座標).高さ場.格子データ),
            Array.from(配る高さ),
            '座標を種にした合成でなく、保管庫が配った高さがそのまま初期の高さになること',
        )
    })

    it('保管庫が高さ格子を配らないときは座標を種にした合成が初期の高さになること', async () => {
        const 状態 = new ワールド編集状態(大域構造)
        const 座標: チャンク座標 = { x: 1, z: 0 }

        await チャンクを読み込んで登録する(new 高さ格子だけ配る保管庫(null), 状態, 座標)

        const 格子データ = 状態.チャンクを取得する(座標).高さ場.格子データ
        assert.strictEqual(格子データ.length, チャンク一辺頂点数 * チャンク一辺頂点数)
        assert.ok(格子データ.some((高さ) => 高さ !== 0), '合成した起伏が入っていること')
    })
})
