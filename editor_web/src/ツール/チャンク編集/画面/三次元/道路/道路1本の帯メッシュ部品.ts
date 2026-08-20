import { DoubleSide, Mesh, MeshStandardMaterial, MeshBasicMaterial, type Object3D } from 'three'
import { グループ, ジオメトリ包み } from 'SengenThree'
import type { 道路スプライン, 高さ場 } from '../../../編集モデル/index.ts'
import { 道路帯幾何データを生成する } from './道路帯ジオメトリ生成.ts'

// 道路1本ぶんの路面メッシュと散布除外バッファメッシュ、およびその形状・材質の資源を所有する部品。
// 道路一覧の1本ごとに1つ作り、道路が消えたら破棄する(資源の解放は三次元部品の破棄連鎖が担う)。
export class 道路1本の帯メッシュ部品 extends グループ {
    private readonly _路面ジオメトリ: ジオメトリ包み
    private readonly _バッファジオメトリ: ジオメトリ包み
    private readonly _路面メッシュ: Mesh
    private readonly _バッファメッシュ: Mesh
    private readonly _路面材質: MeshStandardMaterial

    public constructor(道路色: number) {
        super()
        this._路面ジオメトリ = new ジオメトリ包み()
        this._バッファジオメトリ = new ジオメトリ包み()

        this._路面材質 = new MeshStandardMaterial({
            color: 道路色,
            roughness: 0.7,
            polygonOffset: true,
            polygonOffsetFactor: -2,
            polygonOffsetUnits: -2,
            side: DoubleSide,
        })
        const バッファ材質 = new MeshBasicMaterial({
            color: 0xf43f5e,
            transparent: true,
            opacity: 0.12,
            depthWrite: false,
            side: DoubleSide,
        })

        this._路面メッシュ = new Mesh(this._路面ジオメトリ.実体, this._路面材質)
        this._バッファメッシュ = new Mesh(this._バッファジオメトリ.実体, バッファ材質)
        this._路面メッシュ.visible = false
        this._バッファメッシュ.visible = false

        this.実体.add(this._路面メッシュ)
        this.実体.add(this._バッファメッシュ)

        this.資源台帳.登録する(this._路面ジオメトリ)
        this.資源台帳.登録する(this._バッファジオメトリ)
        this.資源台帳.登録する(this._路面材質)
        this.資源台帳.登録する(バッファ材質)
    }

    public 更新する(スプライン: 道路スプライン, 高さ場モデル: 高さ場): void {
        const 路面幾何 = 道路帯幾何データを生成する(スプライン, スプライン.全幅メートル, スプライン.細分割数, 高さ場モデル, 0.06)
        const バッファ幾何 = 道路帯幾何データを生成する(
            スプライン,
            スプライン.散布除外バッファメートル * 2,
            スプライン.細分割数,
            高さ場モデル,
            0.02,
        )

        if (路面幾何 !== null && バッファ幾何 !== null) {
            this._路面ジオメトリ
                .頂点位置を設定する(路面幾何.頂点配列)
                .UV座標を設定する(路面幾何.UV配列)
                .添字を設定する(路面幾何.添字配列)
                .法線を自動計算する()
            this._バッファジオメトリ
                .頂点位置を設定する(バッファ幾何.頂点配列)
                .UV座標を設定する(バッファ幾何.UV配列)
                .添字を設定する(バッファ幾何.添字配列)
                .法線を自動計算する()

            this._路面メッシュ.visible = true
            this._バッファメッシュ.visible = true
        } else {
            this._路面メッシュ.visible = false
            this._バッファメッシュ.visible = false
        }
    }

    // 当たったのが路面そのものかを判別する。散布除外バッファのメッシュは路面より広く張り出すため、
    // 帯の上のクリックを受け付ける処理は路面だけを見る必要がある。
    public 路面メッシュか(物体: Object3D): boolean {
        return 物体 === this._路面メッシュ
    }

    // テーマ切替時に路面の色を差し替える(参照: 工房テーマ/夜間テーマの道路色)。
    public 道路色を更新する(道路色: number): void {
        this._路面材質.color.set(道路色)
    }
}
