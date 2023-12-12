use std::{
    ffi::{c_void, CString},
    mem::transmute,
    sync::{
        atomic::{AtomicBool, Ordering},
        Mutex,
    },
};

use esp_idf_svc::{
    hal::{
        delay::FreeRtos,
        gpio::{Gpio4, Gpio5, Input, InterruptType, PinDriver, Pull},
        peripherals::Peripherals,
    },
    log::EspLogger,
    sys::{
        esp_log_level_t_ESP_LOG_INFO, esp_log_write, gpio_install_isr_service,
        gpio_isr_handler_add, link_patches, ESP_INTR_FLAG_LEVEL1,
    },
};
use once_cell::unsync::OnceCell;

static FLAG: AtomicBool = AtomicBool::new(false);

static G_ENCODER_S1: Mutex<OnceCell<PinDriver<'_, Gpio4, Input>>> = Mutex::new(OnceCell::new());
static G_ENCODER_S2: Mutex<OnceCell<PinDriver<'_, Gpio5, Input>>> = Mutex::new(OnceCell::new());

/// 中断传递参数
#[derive(Debug)]
struct Args {
    pub pin: i32,
}

fn main() -> anyhow::Result<()> {
    link_patches();

    EspLogger::initialize_default();
    log::set_max_level(log::LevelFilter::Info);

    // Get the peripherals
    let peripherals = Peripherals::take()?;

    let mut encoder_s1 = PinDriver::input(peripherals.pins.gpio4)?;
    let mut encoder_s2 = PinDriver::input(peripherals.pins.gpio5)?;

    // 上拉电阻使能，以防止悬空状态
    encoder_s1.set_pull(Pull::Up)?;
    // 以检测负边缘上的中断
    encoder_s1.set_interrupt_type(InterruptType::NegEdge)?;

    // 上拉电阻使能，以防止悬空状态
    encoder_s2.set_pull(Pull::Up)?;
    // 以检测负边缘上的中断
    encoder_s2.set_interrupt_type(InterruptType::NegEdge)?;

    unsafe {
        gpio_install_isr_service(ESP_INTR_FLAG_LEVEL1 as i32);

        // let mut args1 = std::ptr::null_mut();
        let mut args1 = Args {
            pin: encoder_s1.pin(),
        };
        gpio_isr_handler_add(
            encoder_s1.pin(),
            Some(gpio_isr_handler),
            &mut args1 as *mut Args as *mut c_void,
        );

        let mut args2 = Args {
            pin: encoder_s2.pin(),
        };
        let mut args3 = Args {
            pin: encoder_s2.pin(),
        };
        let data = {
            let args = &mut args3 as *mut Args as *mut c_void;
            // args as *mut Args
            // 使用 transmute 函数将 *mut c_void 转换为 *mut Args
            transmute::<*mut c_void, *mut Args>(args)
        };
        println!("isr encoder s2: {:#?}", (*data).pin);

        gpio_isr_handler_add(
            encoder_s2.pin(),
            Some(gpio_isr_handler),
            &mut args2 as *mut Args as *mut c_void,
        );
    }

    // 日志函数的使用示例
    {
        let tag = CString::new("Rust").unwrap();
        // 创建一个静态的 CString 变量，用于存储日志内容
        let c_str = CString::new(format!("G_ENCODER_S2 Pin: {}\n", encoder_s2.pin())).unwrap();
        // 调用 esp_log_write 函数，打印一个信息级别的日志
        unsafe {
            esp_log_write(
                esp_log_level_t_ESP_LOG_INFO, // 日志级别
                tag.as_ptr(),                 // 日志标签
                c_str.as_ptr(),               // 日志内容
            )
        };
    }

    G_ENCODER_S1.lock().unwrap().get_or_init(move || encoder_s1);
    G_ENCODER_S2.lock().unwrap().get_or_init(move || encoder_s2);

    // Set up a variable that keeps track of press button count
    let mut count = 0_u32;

    log::info!("loop");
    loop {
        FreeRtos::delay_ms(10);
        // Check if global flag is asserted
        if FLAG.load(Ordering::Relaxed) {
            // Reset global flag
            FLAG.store(false, Ordering::Relaxed);
            // Update Press count and print
            count = count.wrapping_add(1);
            println!("Press Count {}", count);
        }
    }
}

// 中断处理函数
unsafe extern "C" fn gpio_isr_handler(args: *mut std::ffi::c_void) {
    let data = &mut *(args as *mut Args);
    let mut lock_encoder_s1 = G_ENCODER_S1.lock().unwrap();
    let mut lock_encoder_s2 = G_ENCODER_S2.lock().unwrap();
    let encoder_s1 = lock_encoder_s1.get_mut().unwrap();
    let encoder_s2 = lock_encoder_s2.get_mut().unwrap();

    // 如果出现数据乱跳的现象，可再次判断引脚电平，以避免抖动
    if data.pin == encoder_s2.pin() {
        // 正转
        if encoder_s1.is_low() {
            FLAG.store(true, Ordering::Relaxed);
        }
    }
}
