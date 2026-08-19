import type { DivC } from 'sengen-ui'
import type { 外殻レイアウト } from 'VscodeShellLayout'
import type { プロジェクト保管庫接続, チャンク座標 } from '../../境界/通信/index.ts'
import { 大域世界表示名, チャンク表示名を生成する } from '../../境界/index.ts'
import { チャンク編集ツール } from '../../ツール/チャンク編集/index.ts'
import { 大域編集ツール } from '../../ツール/大域編集/index.ts'
import type { エクスプローラーパネル } from '../エクスプローラー/index.ts'
import { 使い方タブ } from '../ガイド/index.ts'
import type { テーマ管理サービス } from '../テーマ/index.ts'
import { 大域世界タブID, 使い方タブID, チャンクタブIDを生成する, タブIDからチャンク座標を復元する } from '../タブ識別子.ts'
import { タブ管理サービス } from '../タブ管理サービス.ts'

// エディター外殻が持つシェル・エクスプローラー・インスペクタースロットのうち、
// タブを開く/選択が変わったときの同期だけに触れる窓口。エディター外殻はこの型を保持して
// 公開APIから委譲するだけにする。
export class タブ開閉サービス {
    public readonly タブ管理: タブ管理サービス = new タブ管理サービス()

    public constructor(
        private readonly _シェル: 外殻レイアウト,
        private readonly _エクスプローラー: エクスプローラーパネル,
        private readonly _インスペクタースロット: DivC,
        private readonly _保管庫: プロジェクト保管庫接続,
        private readonly _テーマ管理: テーマ管理サービス,
    ) {}

    public 大域世界を開く(): void {
        if (this._シェル.タブが存在するか(大域世界タブID)) {
            this._シェル.タブを選択する(大域世界タブID)
            return
        }
        const ツール = new 大域編集ツール(undefined, this._保管庫)
        ツール.テーマを適用する(this._テーマ管理.現在テーマを取得する())
        this.タブ管理.ツールを登録する(大域世界タブID, ツール)
        // タブを追加するは追加直後に必ず選択するため、直後の明示選択は不要(冗長な再選択を避ける)。
        this._シェル.タブを追加する(大域世界タブID, 大域世界表示名, ツール)
    }

    public 使い方を開く(): void {
        if (this._シェル.タブが存在するか(使い方タブID)) {
            this._シェル.タブを選択する(使い方タブID)
            return
        }
        const タブ = new 使い方タブ()
        this.タブ管理.ツールを登録する(使い方タブID, タブ)
        this._シェル.タブを追加する(使い方タブID, 使い方タブID, タブ)
    }

    public チャンクを開く(座標: チャンク座標): void {
        const タブID = チャンクタブIDを生成する(座標)
        if (this._シェル.タブが存在するか(タブID)) {
            this._シェル.タブを選択する(タブID)
            return
        }
        const ツール = new チャンク編集ツール(座標, undefined, this._保管庫)
        ツール.テーマを適用する(this._テーマ管理.現在テーマを取得する())
        this.タブ管理.ツールを登録する(タブID, ツール)
        this._シェル.タブを追加する(タブID, チャンク表示名を生成する(座標), ツール)
    }

    public タブ選択時処理(タブID: string): void {
        const ツール = this.タブ管理.タブを選択する(タブID)
        this._インスペクタースロット.clearChildren()
        if (ツール?.インスペクター !== undefined) this._インスペクタースロット.child(ツール.インスペクター)
        if (タブID === 大域世界タブID) {
            this._エクスプローラー.大域世界を選択表示する()
        } else if (タブID === 使い方タブID) {
            this._エクスプローラー.使い方を選択表示する()
        } else {
            const 座標 = タブIDからチャンク座標を復元する(タブID)
            if (座標 !== null) this._エクスプローラー.チャンクを選択表示する(座標)
        }
    }

    public タブを閉じたときの後処理(タブID: string): void {
        this.タブ管理.タブを破棄する(タブID)
        if (this.タブ管理.前面ツールを取得する() === undefined) this._インスペクタースロット.clearChildren()
    }
}
