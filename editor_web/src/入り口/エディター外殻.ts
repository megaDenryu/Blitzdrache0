import { div, LV2HtmlComponentBase, type DivC } from 'sengen-ui'
import type { 外殻レイアウト } from 'VscodeShellLayout'
import type { プロジェクト保管庫接続, チャンク座標 } from '../境界/通信/index.ts'
import { 実サーバー接続, 状態通知付き保管庫接続 } from '../境界/通信/index.ts'
import { 使い方を閲覧済みか } from '../境界/index.ts'
import { エクスプローラーパネル } from './エクスプローラー/index.ts'
import { 設定パネル } from './設定/index.ts'
import { テーマ管理サービス } from './テーマ/index.ts'
import { 起動時タブ計画を立てる } from './起動時タブ計画.ts'
import { 建物の一覧サービス } from './エディター外殻/建物の一覧サービス.ts'
import { 楽曲の一覧サービス } from './エディター外殻/楽曲の一覧サービス.ts'
import { エクスプローラーとタブを結ぶ } from './エディター外殻/エクスプローラーとタブを結ぶ.ts'
import { エクスプローラーアクティビティID, 設定アクティビティID, シェルを構築する } from './エディター外殻/シェルを構築する.ts'
import { タブ開閉サービス } from './エディター外殻/タブ開閉サービス.ts'
import { 外殻ルート } from './スタイル.css.ts'

// エクスプローラーと文書タブ形式で大域世界およびチャンク・建物・楽曲編集ツールを統括する外殻。
export class エディター外殻 extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    public readonly シェル: 外殻レイアウト
    public readonly 保管庫: 状態通知付き保管庫接続
    public readonly テーマ管理: テーマ管理サービス
    private readonly _エクスプローラー: エクスプローラーパネル = new エクスプローラーパネル()
    private readonly _設定パネル: 設定パネル
    private readonly _インスペクタースロット: DivC = div().setStyleCSS({ width: '100%', height: '100%', overflowY: 'auto' })
    private readonly _タブ開閉: タブ開閉サービス
    private readonly _建物の一覧: 建物の一覧サービス
    private readonly _楽曲の一覧: 楽曲の一覧サービス
    private readonly _テーマ購読解除: () => void

    public constructor(保管庫?: プロジェクト保管庫接続) {
        super()
        this.保管庫 = 保管庫 instanceof 状態通知付き保管庫接続 ? 保管庫 : new 状態通知付き保管庫接続(保管庫 ?? new 実サーバー接続())
        this.テーマ管理 = new テーマ管理サービス()
        this._設定パネル = new 設定パネル(this.テーマ管理)

        const 初期テーマ = this.テーマ管理.現在テーマを取得する()
        this.シェル = シェルを構築する(初期テーマ.vsl配色, this._インスペクタースロット)
        this._タブ開閉 = new タブ開閉サービス(
            this.シェル, this._エクスプローラー, this._インスペクタースロット, this.保管庫, this.テーマ管理,
        )

        this._建物の一覧 = new 建物の一覧サービス(this.保管庫, this._エクスプローラー, this._タブ開閉)
        this._楽曲の一覧 = new 楽曲の一覧サービス(this.保管庫, this._エクスプローラー, this._タブ開閉)

        this.シェル.左サイドバーへビューを登録する(エクスプローラーアクティビティID, this._エクスプローラー)
        this.シェル.左サイドバーへビューを登録する(設定アクティビティID, this._設定パネル)

        エクスプローラーとタブを結ぶ(this.シェル, this._エクスプローラー, this._タブ開閉, this._建物の一覧, this._楽曲の一覧)

        // ルート要素へのCSS変数の初期一括適用
        this.テーマ管理.DOMへ適用する(初期テーマ)

        // テーマ変更の購読
        this._テーマ購読解除 = this.テーマ管理.onテーマ変更((新テーマ) => {
            this._タブ開閉.タブ管理.全ツールへテーマを適用する(新テーマ)
        })

        this._componentRoot = div({ class: 外殻ルート }).child(this.シェル)
        void this._建物の一覧.読み直す()
        void this._楽曲の一覧.読み直す()
        for (const タブ種別 of 起動時タブ計画を立てる(使い方を閲覧済みか())) {
            if (タブ種別 === '大域世界') {
                this._タブ開閉.大域世界を開く()
            } else {
                this._タブ開閉.使い方を開く()
            }
        }
    }

    public 大域世界を開く(): void {
        this._タブ開閉.大域世界を開く()
    }

    public 使い方を開く(): void {
        this._タブ開閉.使い方を開く()
    }

    public チャンクを開く(座標: チャンク座標): void {
        this._タブ開閉.チャンクを開く(座標)
    }

    public 寸法を合わせる(幅: number, 高さ: number): void {
        this._タブ開閉.タブ管理.前面ツールを取得する()?.寸法を合わせる(幅, 高さ)
    }

    public override delete(): void {
        this._テーマ購読解除()
        this._タブ開閉.タブ管理.全て破棄する()
        this._エクスプローラー.delete()
        this._設定パネル.delete()
        this.シェル.delete()
        super.delete()
    }
}
