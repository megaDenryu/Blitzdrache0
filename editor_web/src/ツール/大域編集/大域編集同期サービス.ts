import type { ワールド編集状態 } from '../チャンク編集/編集モデル/index.ts'
import type { 大域編集画面部品 } from './画面/index.ts'
import type { 大域編集状態 } from './大域編集状態.ts'

// 大域編集モデルの状態変化を三次元描画部品およびUIパネルへ一括同期する。
export class 大域編集同期サービス {
    public constructor(
        private readonly _モデル: ワールド編集状態,
        private readonly _UI状態: 大域編集状態,
        private readonly _部品: 大域編集画面部品,
    ) {}

    public 全体を同期する(): void {
        this.地形を同期する()
        this.道路を同期する()
        this.UIを同期する()
    }

    public 地形を同期する(): void {
        this._部品.三次元ビュー.地形.高さ場を更新する(this._モデル.大域高さ場)
    }

    public 道路を同期する(): void {
        this._部品.三次元ビュー.道路帯.更新する(this._モデル.広域道路, this._モデル.大域高さ場)
        this._部品.三次元ビュー.道路ノード.更新する(this._モデル.広域道路, this._UI状態.選択中ノード添字)
    }

    public UIを同期する(): void {
        this._部品.インスペクター.表示モードを切り替える(this._UI状態.モード)
        this._部品.インスペクター.部品.ヒント.モードを更新する(this._UI状態.モード)
        this._部品.インスペクター.部品.道路.選択ノード有効状態を設定する(this._UI状態.選択中ノード添字 !== null)
    }
}
