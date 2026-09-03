import { 道路の泥の追従方針の既定 } from '../../../編集モデル/index.ts'
import { 道路パネル } from '../道路/道路パネル.ts'
import { 置いた建物のパネル } from '../建物/置いた建物のパネル.ts'
import { 散布パネル } from '../散布/散布パネル.ts'
import { 地表の焼き直しパネル } from '../地表ペイント/地表の焼き直しパネル.ts'
import { 永続化パネル } from '../永続化/永続化パネル.ts'
import { 等高線パネル } from '../見下ろし図/等高線パネル.ts'
import { 大升パネル } from '../見下ろし図/大升パネル.ts'
import { 見下ろし図の編集状態 } from '../../見下ろし図/見下ろし図の編集状態.ts'

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
        public readonly 等高線: 等高線パネル,
        public readonly 大升: 大升パネル,
    ) {}

    // 見下ろし図の2パネルの初期値は画面の編集状態と同じ1箇所から取る。表示と実際に使う値がずれないためである。
    public static 作る(): インスペクター部品 {
        const 初期 = new 見下ろし図の編集状態()
        return new インスペクター部品(
            new 道路パネル(8.0, 14.0, 80),
            new 置いた建物のパネル(),
            new 散布パネル(5.5),
            new 地表の焼き直しパネル(道路の泥の追従方針の既定),
            new 永続化パネル(),
            new 等高線パネル(初期.新しい等高線の高さメートル, 初期.等高線を導く間隔メートル),
            new 大升パネル(初期.大升の一辺の升目数, {
                高さメートル: 初期.大升に置く高さメートル,
                高さを置くか: 初期.大升に高さを置くか,
                層: 初期.大升に置く層,
                層を置くか: 初期.大升に層を置くか,
                塗りを消すか: 初期.大升の塗りを消すか,
            }),
        )
    }

    public delete(): void {
        this.道路.delete()
        this.建物.delete()
        this.散布.delete()
        this.地表の焼き直し.delete()
        this.永続化.delete()
        this.等高線.delete()
        this.大升.delete()
    }
}
