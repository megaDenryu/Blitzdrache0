import { div, LV2HtmlComponentBase, type DivC } from 'sengen-ui'
import type { 外殻レイアウト } from 'VscodeShellLayout'
import type { プロジェクト保管庫接続, チャンク座標 } from '../境界/通信/index.ts'
import { 実サーバー接続, 状態通知付き保管庫接続 } from '../境界/通信/index.ts'
import { 使い方を閲覧済みか } from '../境界/index.ts'
import { 設定パネル } from './設定/index.ts'
import { テーマ管理サービス } from './テーマ/index.ts'
import { 起動時タブ計画を立てる } from './起動時タブ計画.ts'
import { 編集領域ごとの前面タブ記憶, 編集領域切替サービス, 編集領域登録簿 } from './編集領域/index.ts'
import { 建物の一覧サービス } from './エディター外殻/建物の一覧サービス.ts'
import { 楽曲の一覧サービス } from './エディター外殻/楽曲の一覧サービス.ts'
import { エクスプローラーの操作をタブへ結ぶ } from './エディター外殻/エクスプローラーの操作をタブへ結ぶ.ts'
import { シェルの出来事をタブ開閉と領域切替へ結ぶ } from './エディター外殻/シェルの出来事をタブ開閉と領域切替へ結ぶ.ts'
import { シェルを構築する } from './エディター外殻/シェルを構築する.ts'
import { 下パネルの差し替え係 } from './エディター外殻/下パネルの差し替え係.ts'
import { 左サイドバーへ領域と設定を登録する } from './エディター外殻/左サイドバーへ領域と設定を登録する.ts'
import { タブ開閉サービス } from './エディター外殻/タブ開閉サービス.ts'
import { 外殻ルート } from './スタイル.css.ts'

// アクティビティバーで編集領域(世界・建物・楽曲)を切り替え、開いた対象を全領域で共有する
// 文書タブとして並べる外殻。参照: `_doc/設計/ゲーム開発用エディター基盤.md`「判断12」。
export class エディター外殻 extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    public readonly シェル: 外殻レイアウト
    public readonly 保管庫: 状態通知付き保管庫接続
    public readonly テーマ管理: テーマ管理サービス
    private readonly _登録簿: 編集領域登録簿 = new 編集領域登録簿()
    private readonly _設定パネル: 設定パネル
    private readonly _インスペクタースロット: DivC = div().setStyleCSS({ width: '100%', height: '100%', overflowY: 'auto' })
    private readonly _下パネルスロット: DivC = div().setStyleCSS({ width: '100%', height: '100%' })
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
        this.シェル = シェルを構築する(初期テーマ.vsl配色, this._インスペクタースロット, this._下パネルスロット)
        this._タブ開閉 = new タブ開閉サービス(
            this.シェル, this._登録簿, this._インスペクタースロット,
            new 下パネルの差し替え係(this.シェル, this._下パネルスロット),
            this.保管庫, this.テーマ管理,
        )
        this._建物の一覧 = new 建物の一覧サービス(this.保管庫, this._登録簿.建物のエクスプローラー, this._タブ開閉)
        this._楽曲の一覧 = new 楽曲の一覧サービス(this.保管庫, this._登録簿.楽曲のエクスプローラー, this._タブ開閉)

        左サイドバーへ領域と設定を登録する(this.シェル, this._登録簿, this._設定パネル)
        エクスプローラーの操作をタブへ結ぶ(this._登録簿, this._タブ開閉, this._建物の一覧, this._楽曲の一覧)
        シェルの出来事をタブ開閉と領域切替へ結ぶ(
            this.シェル, this._登録簿, this._タブ開閉,
            new 編集領域切替サービス(this.シェル, new 編集領域ごとの前面タブ記憶()),
            this._建物の一覧, this._楽曲の一覧,
        )

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
        this._登録簿.全て破棄する()
        this._設定パネル.delete()
        this.シェル.delete()
        super.delete()
    }
}
