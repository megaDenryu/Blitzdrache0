import type { チャンク座標 } from '../../../生成/編集資源契約.ts'
import type { ワールド編集状態 } from '../編集モデル/index.ts'
import { 三次元ビュー部品 } from './三次元/三次元ビュー部品.ts'
import { インスペクターパネル } from './パネル/インスペクター/インスペクターパネル.ts'

// チャンク編集画面が集約する三次元ビューとインスペクターパネルの部品DTO。
export class チャンク編集画面部品 {
    public constructor(
        public readonly 三次元ビュー: 三次元ビュー部品,
        public readonly インスペクター: インスペクターパネル,
    ) {}

    public static 作る(編集状態: ワールド編集状態, 対象座標: チャンク座標): チャンク編集画面部品 {
        return new チャンク編集画面部品(
            new 三次元ビュー部品(編集状態),
            new インスペクターパネル(対象座標, '選択'),
        )
    }
}
