import type { ポインタ事象情報, 部品交差情報 } from 'SengenThree'
import type { ワールド編集状態 } from './編集モデル/index.ts'
import type { チャンク編集画面部品 } from './画面/index.ts'
import type { チャンク編集状態 } from './チャンク編集状態.ts'
import type { 道路点クリックハンドラ } from './道路点クリックハンドラ.ts'
import type { チャンク編集同期サービス } from './チャンク編集同期サービス.ts'

// クリック時の道路点の追加・分岐・選択・挿入と、建物の選択を処理する。
// 道路点まわりの手順そのものは道路点クリックハンドラが持ち、ここはモードによる振り分けを担う。
export class チャンク編集クリックハンドラ {
    public constructor(
        private readonly _モデル: ワールド編集状態,
        private readonly _状態: チャンク編集状態,
        private readonly _部品: チャンク編集画面部品,
        private readonly _道路点: 道路点クリックハンドラ,
        private readonly _同期: チャンク編集同期サービス,
    ) {}

    public クリックを処理する(事象: ポインタ事象情報): void {
        if (this._状態.モード === '道作成') {
            this._道を描く(事象)
        } else if (this._状態.モード === '道編集') {
            // 点の上のクリックは選択を優先し、点に当たらず帯の上だったときだけ点を割り込ませる。
            if (!this._道路点.道路点を選択する(事象.当たり一覧)) {
                this._道路点.帯の上へ挿入する(事象.当たり一覧)
            }
        } else if (this._状態.モード === '建物' || this._状態.モード === '選択') {
            this._建物を選択する(事象.当たり一覧)
        }
    }

    // 既存の点の上なら分岐を始め、地形の上なら描き足す。地形のダブルクリックは、
    // 2回目のクリックで点を足さずに道を終え、次のクリックから新しい道を始める合図にする。
    private _道を描く(事象: ポインタ事象情報): void {
        if (this._道路点.分岐を始める(事象.当たり一覧)) return
        const 最前面当たり = 事象.最前面当たり
        if (最前面当たり === null || 最前面当たり.部品 !== this._部品.三次元ビュー.地形) return
        if (事象.原初事象.detail >= 2) {
            this._道路点.次のクリックから新しい道にする()
            return
        }
        const 交差点 = 最前面当たり.交差点
        this._道路点.描き足す({ x: 交差点.x, y: 交差点.y, z: 交差点.z })
    }

    private _建物を選択する(当たり一覧: readonly 部品交差情報[]): boolean {
        const ビュー = this._部品.三次元ビュー
        const チャンク = this._モデル.チャンクを取得する(this._状態.対象チャンク座標)
        for (const 建物 of チャンク.建物.一覧を取得する()) {
            const グループ = ビュー.建物.識別子からグループを取得する(建物.識別子)
            if (グループ) {
                const 一致 = 当たり一覧.some((hit) => {
                    let 親 = hit.原初交差情報.object.parent
                    while (親 !== null) {
                        if (親 === グループ) return true
                        親 = 親.parent
                    }
                    return false
                })
                if (一致) {
                    this._状態.選択中建物識別子 = 建物.識別子
                    this._同期.UIを同期する()
                    return true
                }
            }
        }
        return false
    }
}
