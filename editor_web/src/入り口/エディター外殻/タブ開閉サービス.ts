import type { DivC, HtmlComponentBase } from 'sengen-ui'
import type { 外殻レイアウト } from 'VscodeShellLayout'
import type { プロジェクト保管庫接続, チャンク座標, 建物の格子接続, 建物定義ID, 楽曲接続, 楽曲ID } from '../../境界/通信/index.ts'
import { 大域世界表示名, チャンク表示名を生成する } from '../../境界/index.ts'
import { チャンク編集ツール } from '../../ツール/チャンク編集/index.ts'
import { 建物編集ツール } from '../../ツール/建物編集/index.ts'
import { 楽曲編集ツール } from '../../ツール/楽曲編集/index.ts'
import { 大域編集ツール } from '../../ツール/大域編集/index.ts'
import { マテリアル台帳ツール } from '../../ツール/マテリアル/index.ts'
import { 使い方タブ } from '../ガイド/index.ts'
import type { テーマ管理サービス } from '../テーマ/index.ts'
import type { 実行可能ツール } from '../ツール定義.ts'
import { タブ識別子 } from '../タブ識別子.ts'
import { タブ管理サービス } from '../タブ管理サービス.ts'
import type { 編集領域登録簿 } from '../編集領域/index.ts'
import type { 下パネルの差し替え係 } from './下パネルの差し替え係.ts'
import { タブ見出しを文書の表示名へ追随させる } from './タブ見出しを文書の表示名へ追随させる.ts'

// エディター外殻が持つシェル・編集領域登録簿・インスペクタースロットのうち、
// タブを開く/選択が変わったときの同期だけに触れる窓口。エディター外殻はこの型を保持して
// 公開APIから委譲するだけにする。
export class タブ開閉サービス {
    public readonly タブ管理: タブ管理サービス = new タブ管理サービス()

    public constructor(
        private readonly _シェル: 外殻レイアウト,
        private readonly _登録簿: 編集領域登録簿,
        private readonly _インスペクタースロット: DivC,
        private readonly _下パネルの差し替え: 下パネルの差し替え係,
        private readonly _保管庫: プロジェクト保管庫接続 & 建物の格子接続 & 楽曲接続,
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

    // 建物1件の格子を編集するタブを開く。
    public 建物を開く(建物定義ID: 建物定義ID, 表示名: string): void {
        const 綴り = タブ識別子.建物から生成する(建物定義ID).綴り()
        this.タブを開くか選ぶ(綴り, 表示名, () => this.建物編集ツールを作る(建物定義ID, 表示名, 綴り))
    }

    // 楽曲1件の編集タブを開く。
    public 楽曲を開く(楽曲ID: 楽曲ID, 表示名: string): void {
        const 綴り = タブ識別子.楽曲から生成する(楽曲ID).綴り()
        this.タブを開くか選ぶ(綴り, 表示名, () => this.楽曲編集ツールを作る(楽曲ID, 表示名, 綴り))
    }

    public マテリアルを開く(): void {
        this.タブを開くか選ぶ(タブ識別子.マテリアル().綴り(), 'マテリアル', () => new マテリアル台帳ツール(this._保管庫))
    }

    public 使い方を開く(): void {
        const 綴り = タブ識別子.使い方().綴り()
        this.タブを開くか選ぶ(綴り, 綴り, () => new 使い方タブ())
    }

    // 右サイドバーと下パネルの入れ替えを、ツールを前面にするより先に済ませる。下パネルの開閉で
    // エディタ領域の高さが変わるため、前面にした直後の寸法合わせがその後でないと、三次元の描画の
    // 解像度が開閉前の高さのまま残る。
    public タブ選択時処理(タブID: string): void {
        const これから前面のツール = this.タブ管理.ツールを取得する(タブID)
        this._インスペクタースロット.clearChildren()
        const インスペクター = これから前面のツール?.インスペクター
        if (インスペクター !== undefined) this._インスペクタースロット.child(インスペクター)
        this._下パネルの差し替え.前面のツールに合わせる(これから前面のツール?.下パネル)
        this.タブ管理.タブを選択する(タブID)
        this._登録簿.前面のタブに合わせて選択表示する(タブ識別子.綴りから復元する(タブID))
    }

    public タブを閉じたときの後処理(タブID: string): void {
        this.タブ管理.タブを破棄する(タブID)
        if (this.タブ管理.前面ツールを取得する() !== undefined) return
        this._インスペクタースロット.clearChildren()
        this._下パネルの差し替え.空にする()
    }

    // 楽曲の表示名は中央の欄で書き換えられるため、作ったツールの表示名の知らせをタブの見出しへ結んでおく。
    private 楽曲編集ツールを作る(楽曲ID: 楽曲ID, 表示名: string, 綴り: string): 楽曲編集ツール {
        const ツール = new 楽曲編集ツール(楽曲ID, 表示名, this._保管庫)
        ツール.表示名の知らせの口.配線する(new タブ見出しを文書の表示名へ追随させる(this._シェル, 綴り))
        return ツール
    }

    // 建物の表示名も楽曲と同じく中央の欄で書き換えられるため、同じ届け先へ結ぶ。
    private 建物編集ツールを作る(建物定義ID: 建物定義ID, 表示名: string, 綴り: string): 建物編集ツール {
        const ツール = new 建物編集ツール(建物定義ID, 表示名, this._保管庫)
        ツール.表示名の知らせの口.配線する(new タブ見出しを文書の表示名へ追随させる(this._シェル, 綴り))
        return ツール
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
