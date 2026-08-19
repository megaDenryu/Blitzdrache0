import type { チャンク座標 } from '../../生成/編集資源契約.ts'
import { チャンク座標キーを生成する } from './チャンク座標キー.ts'

export interface 保存状態情報 {
    readonly 文言: string
    readonly エラーか: boolean
}

// 大域世界およびチャンクの保存・読込の最新状態を単一の出どころとして保持・通知するサービス。
export class 保存状態サービス {
    private _大域状態: 保存状態情報 = { 文言: '未保存(初期生成)', エラーか: false }
    private readonly _チャンク状態マップ = new Map<string, 保存状態情報>()
    private readonly _大域リスナー一覧 = new Set<(状態: 保存状態情報) => void>()
    private readonly _チャンクリスナーマップ = new Map<string, Set<(状態: 保存状態情報) => void>>()

    public 大域状態を取得する(): 保存状態情報 {
        return this._大域状態
    }

    public チャンク状態を取得する(座標: チャンク座標): 保存状態情報 {
        const キー = チャンク座標キーを生成する(座標)
        return this._チャンク状態マップ.get(キー) ?? { 文言: '未保存(初期生成)', エラーか: false }
    }

    public 大域状態を更新する(文言: string, エラーか: boolean): void {
        this._大域状態 = { 文言, エラーか }
        for (const リスナー of this._大域リスナー一覧) {
            リスナー(this._大域状態)
        }
    }

    public チャンク状態を更新する(座標: チャンク座標, 文言: string, エラーか: boolean): void {
        const キー = チャンク座標キーを生成する(座標)
        const 状態: 保存状態情報 = { 文言, エラーか }
        this._チャンク状態マップ.set(キー, 状態)
        const リスナー群 = this._チャンクリスナーマップ.get(キー)
        if (リスナー群 !== undefined) {
            for (const リスナー of リスナー群) {
                リスナー(状態)
            }
        }
    }

    public 大域状態を購読する(リスナー: (状態: 保存状態情報) => void): () => void {
        this._大域リスナー一覧.add(リスナー)
        リスナー(this._大域状態)
        return () => {
            this._大域リスナー一覧.delete(リスナー)
        }
    }

    public チャンク状態を購読する(座標: チャンク座標, リスナー: (状態: 保存状態情報) => void): () => void {
        const キー = チャンク座標キーを生成する(座標)
        let リスナー群 = this._チャンクリスナーマップ.get(キー)
        if (リスナー群 === undefined) {
            リスナー群 = new Set()
            this._チャンクリスナーマップ.set(キー, リスナー群)
        }
        リスナー群.add(リスナー)
        リスナー(this.チャンク状態を取得する(座標))
        return () => {
            リスナー群?.delete(リスナー)
        }
    }
}
