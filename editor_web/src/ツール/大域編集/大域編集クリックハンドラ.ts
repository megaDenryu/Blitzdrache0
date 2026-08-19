import type { 部品交差情報 } from 'SengenThree'
import type { 道路点クリックハンドラ } from '../チャンク編集/道路点クリックハンドラ.ts'
import type { 大域編集画面部品 } from './画面/index.ts'
import type { 大域編集状態 } from './大域編集状態.ts'

// 広域道路の点の追加・選択・挿入のクリック入力を、モードによって振り分ける。
// 道路点まわりの手順そのものはチャンク編集と共有の道路点クリックハンドラが持つ。
export class 大域編集クリックハンドラ {
    public constructor(
        private readonly _状態: 大域編集状態,
        private readonly _部品: 大域編集画面部品,
        private readonly _道路点: 道路点クリックハンドラ,
    ) {}

    public クリック処理(最前面当たり: 部品交差情報 | null, 当たり一覧: readonly 部品交差情報[]): void {
        const ビュー = this._部品.三次元ビュー

        if (this._状態.モード === '広域道路作成' && 最前面当たり && 最前面当たり.部品 === ビュー.地形) {
            const 交差点 = 最前面当たり.交差点
            this._道路点.末尾へ追加する({ x: 交差点.x, y: 交差点.y, z: 交差点.z })
            return
        }

        if (this._状態.モード === '広域道路編集') {
            // 点の上のクリックは選択を優先し、点に当たらず帯の上だったときだけ点を割り込ませる。
            if (!this._道路点.道路点を選択する(当たり一覧)) {
                this._道路点.帯の上へ挿入する(当たり一覧)
            }
        }
    }
}
