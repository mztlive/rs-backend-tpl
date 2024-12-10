use crate::errors::{Error, Result};
use entity_core::BaseModel;
use entity_macros::Entity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Entity)]
pub struct SKU {
    #[serde(flatten)]
    pub base: BaseModel,
    pub sku_code: String,
    pub name: String,
    pub price: f64,
    pub original_price: Option<f64>,
    pub stock: i32,
    pub specs: Vec<SkuSpec>,
    pub image: Option<String>,
    pub status: SkuStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SkuSpec {
    pub name: String,  // 如：颜色、尺寸
    pub value: String, // 如：红色、XL
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SkuStatus {
    Active,   // 可售
    Inactive, // 停售
}

#[derive(Debug, Clone)]
pub enum PriceChangeType {
    Fixed(f64),             // 固定价格
    IncreaseByAmount(f64),  // 增加固定金额
    IncreaseByPercent(f64), // 按百分比增加
    DecreaseByAmount(f64),  // 减少固定金额
    DecreaseByPercent(f64), // 按百分比减少
}

impl SKU {
    pub fn new(
        id: String,
        sku_code: String,
        name: String,
        price: f64,
        stock: i32,
        specs: Vec<SkuSpec>,
    ) -> Self {
        Self {
            base: BaseModel::new(id),
            sku_code,
            name,
            price,
            original_price: None,
            stock,
            specs,
            image: None,
            status: SkuStatus::Active,
        }
    }

    pub fn activate(&mut self) -> Result<()> {
        match self.status {
            SkuStatus::Inactive => {
                self.status = SkuStatus::Active;
                Ok(())
            }
            SkuStatus::Active => Ok(()),
        }
    }

    pub fn deactivate(&mut self) -> Result<()> {
        match self.status {
            SkuStatus::Active => {
                self.status = SkuStatus::Inactive;
                Ok(())
            }
            SkuStatus::Inactive => Ok(()),
        }
    }

    /// 调整价格
    ///
    /// # Arguments
    /// * `change_type` - 调价方式
    ///
    /// # Returns
    /// * `Ok(())` - 调价成功
    /// * `Err` - 调价失败，返回具体错误信息
    pub fn change_price(&mut self, change_type: PriceChangeType) -> Result<()> {
        // 保存原价
        if self.original_price.is_none() {
            self.original_price = Some(self.price);
        }

        let new_price = match change_type {
            PriceChangeType::Fixed(price) => {
                if price <= 0.0 {
                    return Err(Error::from_str("价格必须大于0"));
                }
                price
            }
            PriceChangeType::IncreaseByAmount(amount) => {
                if amount <= 0.0 {
                    return Err(Error::from_str("增加金额必须大于0"));
                }
                self.price + amount
            }
            PriceChangeType::IncreaseByPercent(percent) => {
                if !(0.0..=100.0).contains(&percent) {
                    return Err(Error::from_str("百分比必须在0-100之间"));
                }
                self.price * (1.0 + percent / 100.0)
            }
            PriceChangeType::DecreaseByAmount(amount) => {
                if amount <= 0.0 {
                    return Err(Error::from_str("减少金额必须大于0"));
                }
                if amount >= self.price {
                    return Err(Error::from_str("减少金额不能大于或等于原价"));
                }
                self.price - amount
            }
            PriceChangeType::DecreaseByPercent(percent) => {
                if !(0.0..=90.0).contains(&percent) {
                    return Err(Error::from_str("折扣比例必须在0-90之间"));
                }
                self.price * (1.0 - percent / 100.0)
            }
        };

        // 价格精度处理（保留2位小数）
        self.price = (new_price * 100.0).round() / 100.0;
        Ok(())
    }
}
