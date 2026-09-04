import type { 大域世界構造, チャンク座標, チャンク構造, マテリアル台帳 } from '../生成/編集資源契約.ts'
import type { プロジェクト保管庫接続, 読込結果, 保存結果 } from '../境界/通信/index.ts'
import { 読込成功, 読込無し, 保存成功 } from '../境界/通信/index.ts'

// ヘッドレス適用の試験が使う、保存したものをメモリに持つだけの保管庫接続。試験の下ごしらえだけに使い、製品の経路からは呼ばない。
export class 偽保管庫接続 implements プロジェクト保管庫接続 {
    public 保存された大域構造: 大域世界構造 | null = null
    public 保存された大域高さ格子: ArrayBufferLike | null = null
    public readonly 保存されたチャンク構造マップ = new Map<string, チャンク構造>()
    public readonly 保存されたチャンク高さマップ = new Map<string, ArrayBufferLike>()
    public readonly 保存されたチャンク材質マップ = new Map<string, ArrayBufferLike>()
    public 保存されたマテリアル台帳: マテリアル台帳 | null = null

    public async 大域世界の構造を読む(): Promise<読込結果<大域世界構造>> { return this.保存された大域構造 !== null ? 読込成功(this.保存された大域構造) : 読込無し() }
    public async 大域世界の構造を保存する(構造: 大域世界構造): Promise<保存結果> { this.保存された大域構造 = 構造; return 保存成功() }
    public async 大域世界の高さ格子を読む(): Promise<読込結果<ArrayBufferLike>> { return this.保存された大域高さ格子 !== null ? 読込成功(this.保存された大域高さ格子) : 読込無し() }
    public async 大域世界の高さ格子を保存する(バイト列: ArrayBufferLike): Promise<保存結果> { this.保存された大域高さ格子 = バイト列; return 保存成功() }
    public async チャンクの構造を読む(座標: チャンク座標): Promise<読込結果<チャンク構造>> { const 構造 = this.保存されたチャンク構造マップ.get(キー(座標)); return 構造 !== undefined ? 読込成功(構造) : 読込無し() }
    public async チャンクの構造を保存する(座標: チャンク座標, 構造: チャンク構造): Promise<保存結果> { this.保存されたチャンク構造マップ.set(キー(座標), 構造); return 保存成功() }
    public async チャンクの高さ格子を読む(座標: チャンク座標): Promise<読込結果<ArrayBufferLike>> { const 格子 = this.保存されたチャンク高さマップ.get(キー(座標)); return 格子 !== undefined ? 読込成功(格子) : 読込無し() }
    public async チャンクの高さ格子を保存する(座標: チャンク座標, バイト列: ArrayBufferLike): Promise<保存結果> { this.保存されたチャンク高さマップ.set(キー(座標), バイト列); return 保存成功() }
    public async チャンクの材質重みを読む(座標: チャンク座標): Promise<読込結果<ArrayBufferLike>> { const 材質 = this.保存されたチャンク材質マップ.get(キー(座標)); return 材質 !== undefined ? 読込成功(材質) : 読込無し() }
    public async チャンクの材質重みを保存する(座標: チャンク座標, バイト列: ArrayBufferLike): Promise<保存結果> { this.保存されたチャンク材質マップ.set(キー(座標), バイト列); return 保存成功() }
    public async マテリアル台帳を読む(): Promise<読込結果<マテリアル台帳>> { return this.保存されたマテリアル台帳 !== null ? 読込成功(this.保存されたマテリアル台帳) : 読込無し() }
    public async マテリアル台帳を保存する(台帳: マテリアル台帳): Promise<保存結果> { this.保存されたマテリアル台帳 = 台帳; return 保存成功() }
}

function キー(座標: チャンク座標): string {
    return `${座標.x},${座標.z}`
}
