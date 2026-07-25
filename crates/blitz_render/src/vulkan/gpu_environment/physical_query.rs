//! 物理デバイスへの問い合わせ。インスタンスと物理デバイスの組を借用として1つにまとめ、
//! 問い合わせのunsafeをここへ閉じる。呼び出し側は2つのハンドルを個別に持ち回らずに能力を訊ける。
//!
//! 前提: この借用は`GPU環境`が生きている間だけ存在する(インスタンスの生存が問い合わせの前提)。

use ash::vk;

#[derive(Clone, Copy)]
pub(crate) struct 物理デバイス問い合わせ<'a> {
    instance: &'a ash::Instance,
    physical_device: vk::PhysicalDevice,
}

impl<'a> 物理デバイス問い合わせ<'a> {
    pub(super) fn 生成する(instance: &'a ash::Instance, physical_device: vk::PhysicalDevice) -> Self {
        Self { instance, physical_device }
    }

    pub(crate) fn メモリプロパティを取得する(&self) -> vk::PhysicalDeviceMemoryProperties {
        // 安全性: physical_deviceは選定済みで、instanceは借用により問い合わせの間有効である。
        unsafe { self.instance.get_physical_device_memory_properties(self.physical_device) }
    }

    /// 画像形式ごとの機能フラグ。OPTIMALタイリングでのblitと線形フィルタの対応判定に使う。
    pub(crate) fn 形式の性質を取得する(&self, 形式: vk::Format) -> vk::FormatProperties {
        // 安全性: 同上。
        unsafe { self.instance.get_physical_device_format_properties(self.physical_device, 形式) }
    }

    /// GPUタイムスタンプ計測の前提確認(判断30)。戻り値は、そのキューファミリでタイムスタンプが
    /// 有効かと、1tickあたりのナノ秒である。
    pub(crate) fn タイムスタンプ計測条件を調べる(&self, キューファミリ添字: u32) -> (bool, f32) {
        // 安全性: 同上。
        let キューファミリ一覧 = unsafe { self.instance.get_physical_device_queue_family_properties(self.physical_device) };
        let 添字usize = usize::try_from(キューファミリ添字).unwrap_or_else(|_| panic!("キューファミリ添字がusizeに収まらない: {キューファミリ添字}"));
        let タイムスタンプ対応か = キューファミリ一覧.get(添字usize).is_some_and(|性質| 性質.timestamp_valid_bits > 0);

        // 安全性: 同上。
        let 性質 = unsafe { self.instance.get_physical_device_properties(self.physical_device) };
        (タイムスタンプ対応か, 性質.limits.timestamp_period)
    }
}
