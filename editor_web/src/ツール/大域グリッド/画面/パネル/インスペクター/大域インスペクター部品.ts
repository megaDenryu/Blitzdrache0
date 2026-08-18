import { 大域モード切替パネル, type 大域編集モード } from '../モード切替/index.ts'
import { 大域ヒントパネル } from '../ヒント/index.ts'
import { 大域造成パネル } from '../造成/index.ts'
import { 広域道路パネル } from '../道路/index.ts'
import { スライス仕様パネル } from '../スライス/index.ts'

// 大域インスペクターパネルが集約する各サブパネルの部品DTO。
export class 大域インスペクター部品 {
    public constructor(
        public readonly モード切替: 大域モード切替パネル,
        public readonly ヒント: 大域ヒントパネル,
        public readonly 造成: 大域造成パネル,
        public readonly 道路: 広域道路パネル,
        public readonly スライス: スライス仕様パネル,
    ) {}

    public static 作る(初期モード: 大域編集モード = '大域カメラ'): 大域インスペクター部品 {
        return new 大域インスペクター部品(
            new 大域モード切替パネル(初期モード),
            new 大域ヒントパネル(初期モード),
            new 大域造成パネル('盛る', 80.0, 1.2),
            new 広域道路パネル(12.0, 120),
            new スライス仕様パネル(),
        )
    }

    public delete(): void {
        this.モード切替.delete()
        this.ヒント.delete()
        this.造成.delete()
        this.道路.delete()
        this.スライス.delete()
    }
}
