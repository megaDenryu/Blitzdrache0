import type { ワールド編集状態 } from '../編集モデル/index.ts'
import { 三次元ビュー部品 } from './三次元/三次元ビュー部品.ts'
import { インスペクターパネル } from './パネル/インスペクター/インスペクターパネル.ts'

// ワールド編集画面が集約する三次元ビューとインスペクターパネルの部品DTO。
export class ワールド画面部品 {
    public constructor(
        public readonly 三次元ビュー: 三次元ビュー部品,
        public readonly インスペクター: インスペクターパネル,
    ) {}

    public static 作る(編集状態: ワールド編集状態): ワールド画面部品 {
        return new ワールド画面部品(
            new 三次元ビュー部品(編集状態),
            new インスペクターパネル('カメラ'),
        )
    }
}
