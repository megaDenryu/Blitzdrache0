import { 広域道路パネル } from '../道路/index.ts'
import { スライス仕様パネル } from '../スライス/index.ts'
import { 大域永続化パネル } from '../永続化/index.ts'

// 大域インスペクターパネルが集約するサブパネルの部品DTO。ここに並ぶのは「選んでいる道の設定」と
// 「世界ぜんたいへ効く操作(切り出しての書き出し・保存)」だけであり、これから使う筆と道は
// 下パネルの棚が持つ(設計正本の判断14)。
export class 大域インスペクター部品 {
    private constructor(
        public readonly 道路: 広域道路パネル,
        public readonly スライス: スライス仕様パネル,
        public readonly 永続化: 大域永続化パネル,
    ) {}

    public static 作る(): 大域インスペクター部品 {
        return new 大域インスペクター部品(
            new 広域道路パネル(12.0, 120),
            new スライス仕様パネル(),
            new 大域永続化パネル(),
        )
    }

    public delete(): void {
        this.道路.delete()
        this.スライス.delete()
        this.永続化.delete()
    }
}
