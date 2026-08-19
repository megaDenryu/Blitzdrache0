import { div, span, LV2HtmlComponentBase, type DivC } from 'sengen-ui'
import { 外殻レイアウト, アクティビティID } from 'VscodeShellLayout'
import type { プロジェクト保管庫接続, チャンク座標 } from '../境界/通信/index.ts'
import { 実サーバー接続, 状態通知付き保管庫接続 } from '../境界/通信/index.ts'
import { ワールドパイプラインエディター } from '../ツール/ワールド/index.ts'
import { 大域グリッドエディター } from '../ツール/大域グリッド/index.ts'
import { エクスプローラーパネル } from './エクスプローラー/index.ts'
import { 大域世界タブID, チャンクタブIDを生成する, タブIDからチャンク座標を復元する } from './タブ識別子.ts'
import { タブ管理サービス } from './タブ管理サービス.ts'
import { 外殻ルート } from './スタイル.css.ts'
import { エディターテーマ配色 } from './エディターテーマ.ts'

// エクスプローラーと文書タブ形式で大域世界およびチャンク編集ツールを統括する外殻。
export class エディター外殻 extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    public readonly シェル: 外殻レイアウト
    public readonly 保管庫: プロジェクト保管庫接続
    private readonly _エクスプローラー: エクスプローラーパネル = new エクスプローラーパネル()
    private readonly _タブ管理: タブ管理サービス = new タブ管理サービス()
    private readonly _インスペクタースロット: DivC = div().setStyleCSS({ width: '100%', height: '100%', overflowY: 'auto' })

    public constructor(保管庫?: プロジェクト保管庫接続) {
        super()
        this.保管庫 = 保管庫 instanceof 状態通知付き保管庫接続 ? 保管庫 : new 状態通知付き保管庫接続(保管庫 ?? new 実サーバー接続())
        this.シェル = new 外殻レイアウト({
            テーマ: エディターテーマ配色,
            タイトル: 'Blitzdrache0 エディター',
            アクティビティ項目一覧: [{
                id: アクティビティID('explorer'), ラベル: 'エクスプローラー',
                アイコン: (size, color) => span({ text: 'E' }).setStyleCSS({
                    fontWeight: 'bold', fontSize: `${size * 0.8}px`, color,
                    display: 'flex', alignItems: 'center', justifyContent: 'center', width: `${size}px`, height: `${size}px`,
                }),
            }],
            メニューバー表示: true, ステータスバー表示: true, ステータスバー右テキスト: 'Blitzdrache0 v0.1.0',
            パネル初期表示: false, 右サイドバー有効: true, 右サイドバー内容: this._インスペクタースロット,
        })
        this.シェル.左サイドバーへビューを登録する(アクティビティID('explorer'), this._エクスプローラー)
        this._エクスプローラー.配線する({
            on大域世界を開く: () => this.大域世界を開く(), onチャンクを開く: (座標) => this.チャンクを開く(座標),
        })
        this.シェル.onタブイベント({
            onタブ選択: (id) => this._タブ選択時処理(id),
            onタブ閉じる: (id) => {
                this._タブ管理.タブを破棄する(id)
                if (this._タブ管理.前面ツールを取得する() === undefined) this._インスペクタースロット.clearChildren()
            },
        })
        this._componentRoot = div({ class: 外殻ルート }).child(this.シェル)
        this.大域世界を開く()
    }

    public 大域世界を開く(): void {
        if (this.シェル.タブが存在するか(大域世界タブID)) {
            this.シェル.タブを選択する(大域世界タブID)
            return
        }
        const ツール = new 大域グリッドエディター(undefined, this.保管庫)
        this._タブ管理.ツールを登録する(大域世界タブID, ツール)
        this.シェル.タブを追加する(大域世界タブID, '大域世界', ツール)
        this.シェル.タブを選択する(大域世界タブID)
    }

    public チャンクを開く(座標: チャンク座標): void {
        const タブID = チャンクタブIDを生成する(座標)
        if (this.シェル.タブが存在するか(タブID)) {
            this.シェル.タブを選択する(タブID)
            return
        }
        const ツール = new ワールドパイプラインエディター(座標, undefined, this.保管庫)
        this._タブ管理.ツールを登録する(タブID, ツール)
        this.シェル.タブを追加する(タブID, `チャンク (${座標.x}, ${座標.z})`, ツール)
        this.シェル.タブを選択する(タブID)
    }

    private _タブ選択時処理(タブID: string): void {
        const ツール = this._タブ管理.タブを選択する(タブID)
        this._インスペクタースロット.clearChildren()
        if (ツール?.インスペクター !== undefined) this._インスペクタースロット.child(ツール.インスペクター)
        if (タブID === 大域世界タブID) {
            this._エクスプローラー.大域世界を選択表示する()
        } else {
            const 座標 = タブIDからチャンク座標を復元する(タブID)
            if (座標 !== null) this._エクスプローラー.チャンクを選択表示する(座標)
        }
    }

    public 寸法を合わせる(幅: number, 高さ: number): void {
        this._タブ管理.前面ツールを取得する()?.寸法を合わせる(幅, 高さ)
    }

    public override delete(): void {
        this._タブ管理.全て破棄する()
        this._エクスプローラー.delete()
        this.シェル.delete()
        super.delete()
    }
}
