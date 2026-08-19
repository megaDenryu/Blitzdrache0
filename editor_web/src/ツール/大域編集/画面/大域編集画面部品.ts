import type { ワールド編集状態 } from '../../チャンク編集/編集モデル/index.ts'
import { 大域三次元ビュー部品 } from './三次元/大域三次元ビュー部品.ts'
import { 大域インスペクターパネル } from './パネル/インスペクター/大域インスペクターパネル.ts'

// 大域編集画面が集約する三次元ビューとインスペクターパネルの部品DTO。
export class 大域編集画面部品 {
    public constructor(
        public readonly 三次元ビュー: 大域三次元ビュー部品,
        public readonly インスペクター: 大域インスペクターパネル,
    ) {}

    public static 作る(編集状態: ワールド編集状態): 大域編集画面部品 {
        return new 大域編集画面部品(
            new 大域三次元ビュー部品(編集状態),
            new 大域インスペクターパネル('選択'),
        )
    }
}
