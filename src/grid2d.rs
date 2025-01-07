use super::constant::INVALID;
use super::types::Grid;
use super::util::bilinear;
use serde::{Deserialize, Serialize};
// 定义数据结构
#[derive(Serialize, Deserialize, Debug)]
pub struct Grid2D {
    pub epsg: i32,    // 格网地理坐标系
    pub start_x: f64, // 格网起始经纬度，通常是左上角或左下角
    pub start_y: f64,
    end_x: f64,
    end_y: f64,
    pub res_x: f64,
    pub res_y: f64,
    pub width: usize,
    pub height: usize,
    pub datas: Vec<f64>,
}
impl Grid for Grid2D {
    fn get_end_x(&self) -> f64 {
        self.end_x
    }
    fn get_end_y(&self) -> f64 {
        self.end_y
    }
}
impl Grid2D {
    pub fn new(
        start_x: f64,
        start_y: f64,
        epsg: i32,
        res_x: f64,
        res_y: f64,
        width: usize,
        height: usize,
        datas: Vec<f64>,
    ) -> Grid2D {
        // 校验datas长度是否等于宽高
        if datas.len() != width * height {
            panic!("2D格网datas长度与元数据的width*height不匹配！");
        }
        let end_x = start_x + res_x * (width as f64);
        let end_y = start_y + res_y * (height as f64);
        Grid2D {
            epsg,
            start_x,
            start_y,
            end_x,
            end_y,
            res_x,
            res_y,
            width,
            height,
            datas,
        }
    }
    // 任意经纬度，返回格网数值
    // 内置双线性插值取值
    pub fn get_val(&self, x: f64, y: f64) -> f64 {
        let _min_y = self.start_y.min(self.get_end_y());
        let _max_y = self.start_y.max(self.get_end_y());
        if x < self.start_x || x > self.get_end_x() || y < _min_y || y > _max_y {
            INVALID
        } else {
            // 双线性插值
            // 计算点所在的起始行列号，相对起始点的百分比
            let float_row = (y - self.start_y) / self.res_y;
            let float_column = (x - self.start_x) / self.res_x;
            let start_row = float_row.floor() as usize;
            let start_column = float_column.floor() as usize;
            let left_scale_factor = float_column.fract();
            let top_scale_factor = float_row.fract();

            let mut end_row = start_row + 1;
            let mut end_column = start_column + 1;
            if end_row >= self.height {
                end_row = start_row;
            }
            if end_column >= self.width {
                end_column = start_column;
            }
            let lt_index = start_row * self.width + start_column;
            let rt_index = start_row * self.width + end_column;
            let lb_index = end_row * self.width + start_column;
            let rb_index = end_row * self.width + end_column;
            let lt_wal = self.datas[lt_index];
            let rt_val = self.datas[rt_index];
            let lb_val = self.datas[lb_index];
            let rb_val = self.datas[rb_index];
            let val = bilinear(
                lt_wal,
                rt_val,
                lb_val,
                rb_val,
                left_scale_factor,
                top_scale_factor,
            );
            val
        }
    }
}
