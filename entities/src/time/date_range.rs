use serde::{Deserialize, Serialize};

/// 表示一个时间段的元组类型,包含开始和结束时间戳
pub type Period = (u64, u64);

/// 表示一个日期范围的结构体
///
/// # 字段
///
/// * `start` - 开始时间戳,可选
/// * `end` - 结束时间戳,可选
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct DateRange {
    pub start: Option<u64>,
    pub end: Option<u64>,
}

impl DateRange {
    /// 检查日期范围是否为空
    ///
    /// # 返回值
    ///
    /// 如果开始时间或结束时间任一为 None,则返回 true
    pub fn is_empty(&self) -> bool {
        self.start.is_none() || self.end.is_none()
    }

    /// 将日期范围转换为时间段元组
    ///
    /// # 返回值
    ///
    /// 返回一个包含开始和结束时间戳的元组,如果某个时间戳为 None 则使用 0 代替
    pub fn to_period(&self) -> Period {
        (self.start.unwrap_or(0), self.end.unwrap_or(0))
    }
}
