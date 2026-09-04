//! SHA-256の指紋。読み戻しのバイト列を1つの綴りへ縮めて、別のプロセスの実行どうしでビット一致を突き合わせるために持つ。
//! 外部クレートを足さずにここで持つのは、依存の追加が採用審査と白リストの更新を要し、この計測の作業でそれを行わないためである。
//! 仕様はFIPS 180-4のSHA-256であり、正しさは既知の入力(空・"abc")の指紋との一致で検査する。定数の表は`constants`が持つ。

mod constants;

use constants::{丸め定数, 初期ハッシュ};

/// バイト列のSHA-256を小文字の十六進64桁で返す。
pub(super) fn 十六進の指紋を計算する(バイト列: &[u8]) -> String {
    sha256の語の列を計算する(バイト列).iter().map(|語| format!("{語:08x}")).collect()
}

fn sha256の語の列を計算する(バイト列: &[u8]) -> [u32; 8] {
    let mut 詰めた = バイト列.to_vec();
    詰めた.push(0x80);
    詰め物を足す(&mut 詰めた);
    let ビット長 = u64::try_from(バイト列.len()).unwrap_or_else(|_| panic!("入力長がu64に収まらない")) * 8;
    詰めた.extend_from_slice(&ビット長.to_be_bytes());
    let mut 状態 = 初期ハッシュ;
    for 塊 in 詰めた.chunks_exact(64) {
        塊を圧縮する(&mut 状態, 塊);
    }
    状態
}

/// 長さの8バイトを足したときに64バイトの倍数になるまで0を詰める。
fn 詰め物を足す(詰めた: &mut Vec<u8>) {
    while 詰めた.len() % 64 != 56 {
        詰めた.push(0);
    }
}

fn 塊を圧縮する(状態: &mut [u32; 8], 塊: &[u8]) {
    let mut 予定 = [0u32; 64];
    for (添字, 語) in 塊.chunks_exact(4).enumerate() {
        予定[添字] = u32::from_be_bytes([語[0], 語[1], 語[2], 語[3]]);
    }
    for 添字 in 16..64 {
        let 小0 = 予定[添字 - 15].rotate_right(7) ^ 予定[添字 - 15].rotate_right(18) ^ (予定[添字 - 15] >> 3);
        let 小1 = 予定[添字 - 2].rotate_right(17) ^ 予定[添字 - 2].rotate_right(19) ^ (予定[添字 - 2] >> 10);
        予定[添字] = 予定[添字 - 16].wrapping_add(小0).wrapping_add(予定[添字 - 7]).wrapping_add(小1);
    }
    let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = *状態;
    for 添字 in 0..64 {
        let 大1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
        let 選択 = (e & f) ^ (!e & g);
        let 一時1 = h
            .wrapping_add(大1)
            .wrapping_add(選択)
            .wrapping_add(丸め定数[添字])
            .wrapping_add(予定[添字]);
        let 大0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
        let 多数決 = (a & b) ^ (a & c) ^ (b & c);
        let 一時2 = 大0.wrapping_add(多数決);
        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(一時1);
        d = c;
        c = b;
        b = a;
        a = 一時1.wrapping_add(一時2);
    }
    for (格納先, 値) in 状態.iter_mut().zip([a, b, c, d, e, f, g, h]) {
        *格納先 = 格納先.wrapping_add(値);
    }
}
