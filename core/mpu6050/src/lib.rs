pub mod conf;
pub mod hal;
pub mod reg;

/// 加速度和角速度数据
#[derive(Default)]
pub struct AccelGyroData {
    pub acc_x: i16,
    pub acc_y: i16,
    pub acc_z: i16,
    pub gyro_x: i16,
    pub gyro_y: i16,
    pub gyro_z: i16,
}
