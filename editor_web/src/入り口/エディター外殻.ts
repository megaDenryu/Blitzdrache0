import { div, span, LV2HtmlComponentBase, type DivC } from 'sengen-ui'
import { 外殻レイアウト, アクティビティID } from 'VscodeShellLayout'
import type { ツール項目, 実行可能ツール } from './ツール定義.ts'
import { ツール登録一覧 } from './ツール一覧.ts'
import { 外殻ルート } from './スタイル.css.ts'

// VscodeShellLayoutを構築し、複数のツールをアクティビティバーから切り替えてホストする外殻。
export class エディター外殻 extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    public readonly シェル: 外殻レイアウト
    private readonly _ホスト: DivC
    private readonly _ツールマップ: Map<string, ツール項目> = new Map<string, ツール項目>()
    private _現在ツール: 実行可能ツール | null = null
    private _現在ツールID: string | null = null

    public constructor() {
        super()
        for (const t of ツール登録一覧) {
            this._ツールマップ.set(t.識別子, t)
        }

        const アイコン描画 = (文字: string) => (size: number, color: string) =>
            span({ text: 文字 }).setStyleCSS({
                fontWeight: 'bold',
                fontSize: `${size * 0.8}px`,
                color: color,
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                width: `${size}px`,
                height: `${size}px`,
            })

        this.シェル = new 外殻レイアウト({
            タイトル: 'Blitzdrache0 エディター',
            アクティビティ項目一覧: ツール登録一覧.map((t) => ({
                id: アクティビティID(t.識別子),
                ラベル: t.ラベル,
                アイコン: アイコン描画(t.アイコン記号),
            })),
            メニューバー表示: true,
            ステータスバー表示: true,
            ステータスバー右テキスト: 'Blitzdrache0 v0.1.0',
        })

        this._ホスト = div().setStyleCSS({ width: '100%', height: '100%', position: 'relative' })
        this.シェル.タブを追加する('editor-main', 'エディター', this._ホスト)
        this.シェル.タブを選択する('editor-main')

        this.シェル.onアクティビティ選択((id) => {
            this.ツールを切り替える(id)
        })

        this.ツールを切り替える('world-pipeline')
        this._componentRoot = div({ class: 外殻ルート }).child(this.シェル)
    }

    public ツールを切り替える(ツールID: string): void {
        if (this._現在ツールID === ツールID) return
        const 定義 = this._ツールマップ.get(ツールID)
        if (定義 === undefined) {
            throw new Error(`未登録のツール識別子: ${ツールID}`)
        }

        if (this._現在ツール !== null) {
            this._現在ツール.delete()
            this._ホスト.clearChildren()
        }

        this._現在ツール = 定義.ツールを生成する()
        this._現在ツールID = ツールID
        this._ホスト.child(this._現在ツール)
        this._現在ツール.寸法を合わせる(window.innerWidth, window.innerHeight)
    }

    public 寸法を合わせる(幅: number, 高さ: number): void {
        this._現在ツール?.寸法を合わせる(幅, 高さ)
    }

    public override delete(): void {
        this._現在ツール?.delete()
        this.シェル.delete()
        super.delete()
    }
}
