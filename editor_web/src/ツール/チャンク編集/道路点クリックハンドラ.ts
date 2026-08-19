import type { 部品交差情報 } from 'SengenThree'
import type { 位置3次元, 道路対象 } from '../../生成/編集資源契約.ts'
import type { ワールド編集状態 } from './編集モデル/index.ts'
import { 道路対象の道路スプラインを取り出す, 道路点の挿入先を求める } from './編集モデル/index.ts'
import type {
    道路点の選択状態,
    道路点編集の同期先,
    道路点編集の操作先,
    道路点編集対象ビュー,
} from './道路点編集の相手.ts'

// 道路点のクリック編集を担う操作サービス。末尾への追加・マーカーの選択・帯の上への割り込みの
// 3つを持ち、チャンク編集と大域編集の両方の道路モードから同じ手順で呼ばれる。
export class 道路点クリックハンドラ {
    public constructor(
        private readonly _モデル: ワールド編集状態,
        private readonly _状態: 道路点の選択状態,
        private readonly _ビュー: 道路点編集対象ビュー,
        private readonly _操作: 道路点編集の操作先,
        private readonly _同期: 道路点編集の同期先,
        private readonly _道路対象: 道路対象,
        // 広域道路は大域地形より少し浮かせて置くため、追加する点をこの高さだけ持ち上げる。
        private readonly _追加時に持ち上げるメートル: number = 0,
    ) {}

    public 末尾へ追加する(交差点: 位置3次元): void {
        this._操作.コマンドを実行する({
            種類: '道路点を追加する',
            値: {
                対象: this._道路対象,
                位置: { x: 交差点.x, y: 交差点.y + this._追加時に持ち上げるメートル, z: 交差点.z },
            },
        })
    }

    // マーカーに当たっていればその道路点を選ぶ。選べたらtrueを返す。
    public 道路点を選択する(当たり一覧: readonly 部品交差情報[]): boolean {
        for (const 当たり of 当たり一覧) {
            if (当たり.部品 !== this._ビュー.道路点マーカー) continue
            const 添字 = this._ビュー.道路点マーカー.当たった道路点の添字を求める(当たり.原初交差情報.object)
            if (添字 === null) continue
            this._状態.選択中の道路点の添字 = 添字
            this._同期.道路を同期する()
            this._同期.UIを同期する()
            return true
        }
        return false
    }

    // 路面の上に当たっていれば、いちばん近い区間へ点を割り込ませる。割り込ませたらtrueを返す。
    // 散布除外バッファのメッシュは路面より広いため、路面そのものに当たったときだけ受け付ける。
    public 帯の上へ挿入する(当たり一覧: readonly 部品交差情報[]): boolean {
        const 路面の当たり = 当たり一覧.find(
            (当たり) => 当たり.部品 === this._ビュー.道路帯 && this._ビュー.道路帯.路面メッシュか(当たり.原初交差情報.object),
        )
        if (路面の当たり === undefined) return false

        const 制御点列 = 道路対象の道路スプラインを取り出す(this._モデル, this._道路対象).制御点列
        const 交差点 = 路面の当たり.交差点
        const 挿入先 = 道路点の挿入先を求める(制御点列, { x: 交差点.x, y: 交差点.y, z: 交差点.z })
        if (挿入先 === null) return false

        this._操作.コマンドを実行する({
            種類: '道路点を挿入する',
            値: { 対象: this._道路対象, 添字: 挿入先.添字, 位置: 挿入先.位置 },
        })
        this._状態.選択中の道路点の添字 = 挿入先.添字
        this._同期.道路を同期する()
        this._同期.UIを同期する()
        return true
    }
}
