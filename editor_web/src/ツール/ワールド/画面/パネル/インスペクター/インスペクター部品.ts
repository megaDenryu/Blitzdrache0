import type { 編集モード } from '../モード切替/モード定義.ts'
import { モード切替パネル } from '../モード切替/モード切替パネル.ts'
import { 造成パネル } from '../造成/造成パネル.ts'
import { 地表ペイントパネル } from '../地表ペイント/地表ペイントパネル.ts'
import { 道路パネル } from '../道路/道路パネル.ts'
import { 建物パネル } from '../建物/建物パネル.ts'
import { 散布パネル } from '../散布/散布パネル.ts'

// インスペクターパネルが集約する各設定サブパネルの部品DTO。
export class インスペクター部品 {
    public constructor(
        public readonly モード切替: モード切替パネル,
        public readonly 造成: 造成パネル,
        public readonly 地表ペイント: 地表ペイントパネル,
        public readonly 道路: 道路パネル,
        public readonly 建物: 建物パネル,
        public readonly 散布: 散布パネル,
    ) {}

    public static 作る(初期モード: 編集モード): インスペクター部品 {
        return new インスペクター部品(
            new モード切替パネル(初期モード),
            new 造成パネル('盛る', 20.0, 0.5),
            new 地表ペイントパネル('草', 15.0, 0.4),
            new 道路パネル(8.0, 14.0, 80),
            new 建物パネル(),
            new 散布パネル(5.5),
        )
    }
}
