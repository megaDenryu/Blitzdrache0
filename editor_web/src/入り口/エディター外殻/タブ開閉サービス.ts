import type { DivC, HtmlComponentBase } from 'sengen-ui'
import type { 外殻レイアウト } from 'VscodeShellLayout'
import type { プロジェクト保管庫接続, チャンク座標, 建物の格子接続 } from '../../境界/通信/index.ts'
import { 大域世界表示名, チャンク表示名を生成する } from '../../境界/index.ts'
import { チャンク編集ツール } from '../../ツール/チャンク編集/index.ts'
import { 建物編集ツール } from '../../ツール/建物編集/index.ts'
import { 大域編集ツール } from '../../ツール/大域編集/index.ts'
import { マテリアル台帳ツール } from '../../ツール/マテリアル/index.ts'
import type { エクスプローラーパネル } from '../エクスプローラー/index.ts'
import { 使い方タブ } from '../ガイド/index.ts'
import type { テーマ管理サービス } from '../テーマ/index.ts'
import type { 実行可能ツール } from '../ツール定義.ts'
import { タブ識別子 } from '../タブ識別子.ts'
import { タブ管理サービス } from '../タブ管理サービス.ts'
import { タブの選択表示を合わせる } from './タブの選択表示を合わせる.ts'

// エディター外殻が持つシェル・エクスプローラー・インスペクタースロットのうち、
// タブを開く/選択が変わったときの同期だけに触れる窓口。エディター外殻はこの型を保持して
// 公開APIから委譲するだけにする。
export class タブ開閉サービス {
    public readonly タブ管理: タブ管理サービス = new タブ管理サービス()

    public constructor(
        private readonly _シェル: 外殻レイアウト,
        private readonly _エクスプローラー: エクスプローラーパネル,
        private readonly _インスペクタースロット: DivC,
        private readonly _保管庫: プロジェクト保管庫接続 & 建物の格子接続,
        private readonly _テーマ管理: テーマ管理サービス,
    ) {}

    public 大域世界を開く(): void {
        this.タブを開くか選ぶ(タブ識別子.大域世界().綴り(), 大域世界表示名, () => {
            const ツール = new 大域編集ツール(undefined, this._保管庫)
            ツール.テーマを適用する(this._テーマ管理.現在テーマを取得する())
            return ツール
        })
    }

    public チャンクを開く(座標: チャンク座標): void {
        this.タブを開くか選ぶ(タブ識別子.チャンクから生成する(座標).綴り(), チャンク表示名を生成する(座標), () => {
            const ツール = new チャンク編集ツール(座標, undefined, this._保管庫)
            ツール.テーマを適用する(this._テーマ管理.現在テーマを取得する())
            return ツール
        })
    }

    // 建物1件の格子を編集するタブを開く。保管庫の接続が格子の口を持たないときは、
    // ツールの内側で明示の失敗として画面へ出る(無言で編集できないタブにはしない)。
    public 建物を開く(建物定義ID: string, 表示名: string): void {
        const 綴り = タブ識別子.建物から生成する(建物定義ID).綴り()
        this.タブを開くか選ぶ(綴り, 表示名, () => new 建物編集ツール(建物定義ID, 表示名, this._保管庫))
    }

    public マテリアルを開く(): void {
        this.タブを開くか選ぶ(タブ識別子.マテリアル().綴り(), 'マテリアル', () => new マテリアル台帳ツール(this._保管庫))
    }

    public 使い方を開く(): void {
        const 綴り = タブ識別子.使い方().綴り()
        this.タブを開くか選ぶ(綴り, 綴り, () => new 使い方タブ())
    }

    public タブ選択時処理(タブID: string): void {
        const ツール = this.タブ管理.タブを選択する(タブID)
        this._インスペクタースロット.clearChildren()
        if (ツール?.インスペクター !== undefined) this._インスペクタースロット.child(ツール.インスペクター)
        タブの選択表示を合わせる(this._エクスプローラー, タブID)
    }

    public タブを閉じたときの後処理(タブID: string): void {
        this.タブ管理.タブを破棄する(タブID)
        if (this.タブ管理.前面ツールを取得する() === undefined) this._インスペクタースロット.clearChildren()
    }

    // 既に開いているタブは選び直すだけにし、ツールを作り直さない。ツールを作るかどうかをこの工程が
    // 決めるため、作り方は呼び出し側から受け取る(作ってから捨てると描画資源が無駄に確保される)。
    // タブを追加するは追加直後に必ず選択するため、直後の明示選択は不要である。
    private タブを開くか選ぶ(綴り: string, 表示名: string, ツールを作る: () => 実行可能ツール & HtmlComponentBase): void {
        if (this._シェル.タブが存在するか(綴り)) {
            this._シェル.タブを選択する(綴り)
            return
        }
        const ツール = ツールを作る()
        this.タブ管理.ツールを登録する(綴り, ツール)
        this._シェル.タブを追加する(綴り, 表示名, ツール)
    }
}
