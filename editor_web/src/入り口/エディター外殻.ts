import { div, span, LV2HtmlComponentBase, type DivC } from 'sengen-ui'
import { 外殻レイアウト, アクティビティID } from 'VscodeShellLayout'
import { ワールドパイプラインエディター } from '../ツール/ワールド/index.ts'

// VscodeShellLayoutを構築し、ワールドパイプラインエディターをエディタエリアにホストする外殻。
export class エディター外殻 extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    public readonly シェル: 外殻レイアウト
    public readonly ワールドエディター: ワールドパイプラインエディター

    public constructor() {
        super()
        this.ワールドエディター = new ワールドパイプラインエディター()

        const アイコン描画 = (size: number, color: string) =>
            span({ text: 'W' }).setStyleCSS({
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
            アクティビティ項目一覧: [
                {
                    id: アクティビティID('world-pipeline'),
                    ラベル: 'ワールド',
                    アイコン: アイコン描画,
                },
            ],
            メニューバー表示: true,
            ステータスバー表示: true,
            ステータスバー右テキスト: 'Blitzdrache0 v0.1.0',
        })

        this.シェル.タブを追加する('world-pipeline-chunk', 'ワールド (チャンク 0,0)', this.ワールドエディター)
        this.シェル.タブを選択する('world-pipeline-chunk')

        this._componentRoot = div().setStyleCSS({ width: '100%', height: '100%', position: 'relative' }).child(this.シェル)
    }

    public 寸法を合わせる(幅: number, 高さ: number): void {
        this.ワールドエディター.寸法を合わせる(幅, 高さ)
    }

    public override delete(): void {
        this.ワールドエディター.delete()
        this.シェル.delete()
        super.delete()
    }
}
