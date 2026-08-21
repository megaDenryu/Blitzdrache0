import { div, LV2HtmlComponentBase, type DivC } from 'sengen-ui'
import type { プロジェクト保管庫接続 } from '../../境界/通信/index.ts'
import { 実サーバー接続 } from '../../境界/通信/index.ts'
import { マテリアル台帳パネル } from './画面/index.ts'
import { マテリアル台帳の管理 } from './編集モデル/index.ts'
import { マテリアル台帳イベントを配線する, 起動時にマテリアル台帳を読み込む } from './マテリアル台帳配線.ts'

// マテリアルの登録(エンジン材質名との対応)と識別用カラーの定義を行う文書タブのツールルート。
// 三次元ビューを持たない(判断9)。実行可能ツールの契約は空実装で満たす(前面/背面での
// 描画ループの開始・停止は無く、寸法合わせも要らない)。
export class マテリアル台帳ツール extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    public readonly 画面: マテリアル台帳パネル = new マテリアル台帳パネル()
    public readonly 管理: マテリアル台帳の管理 = new マテリアル台帳の管理()
    public readonly 保管庫: プロジェクト保管庫接続

    public constructor(保管庫?: プロジェクト保管庫接続) {
        super()
        this.保管庫 = 保管庫 ?? new 実サーバー接続()
        this._componentRoot = div().setStyleCSS({ width: '100%', height: '100%' }).child(this.画面)

        const 表示を再構築する = マテリアル台帳イベントを配線する(this.画面, this.管理, this.保管庫)
        void 起動時にマテリアル台帳を読み込む(this.画面, this.管理, this.保管庫, 表示を再構築する)
    }

    public 寸法を合わせる(): void {}

    public 前面になった(): void {}

    public 背面になった(): void {}

    public override delete(): void {
        this.画面.delete()
        super.delete()
    }
}
