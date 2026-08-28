import { 道路の泥の追従方針の既定 } from '../../../編集モデル/index.ts'
import { 道路パネル } from '../道路/道路パネル.ts'
import { 置いた建物のパネル } from '../建物/置いた建物のパネル.ts'
import { 散布パネル } from '../散布/散布パネル.ts'
import { 地表の焼き直しパネル } from '../地表ペイント/地表の焼き直しパネル.ts'
import { 永続化パネル } from '../永続化/永続化パネル.ts'

// インスペクターパネルが集約する設定サブパネルの部品DTO。ここに並ぶのは「選んでいるものの設定」と
// 「いま開いているチャンクへ効く操作」だけであり、これから使う筆と置くものは下パネルの棚が持つ
// (設計正本の判断14)。
export class インスペクター部品 {
    private constructor(
        public readonly 道路: 道路パネル,
        public readonly 建物: 置いた建物のパネル,
        public readonly 散布: 散布パネル,
        public readonly 地表の焼き直し: 地表の焼き直しパネル,
        public readonly 永続化: 永続化パネル,
    ) {}

    public static 作る(): インスペクター部品 {
        return new インスペクター部品(
            new 道路パネル(8.0, 14.0, 80),
            new 置いた建物のパネル(),
            new 散布パネル(5.5),
            new 地表の焼き直しパネル(道路の泥の追従方針の既定),
            new 永続化パネル(),
        )
    }

    public delete(): void {
        this.道路.delete()
        this.建物.delete()
        this.散布.delete()
        this.地表の焼き直し.delete()
        this.永続化.delete()
    }
}
