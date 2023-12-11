use std::sync::Mutex;

use esp_idf_svc::{
    hal::{
        delay::FreeRtos,
        gpio::{Gpio5, InterruptType, Output, PinDriver, Pull},
        peripherals::Peripherals,
    },
    sys::{gpio_install_isr_service, gpio_isr_handler_add, link_patches, ESP_INTR_FLAG_LEVEL1},
};
use once_cell::sync::OnceCell;

static G_LED: Mutex<OnceCell<PinDriver<'_, Gpio5, Output>>> = Mutex::new(OnceCell::new());

fn main() -> anyhow::Result<()> {
    link_patches();

    let peripherals = Peripherals::take()?;

    let led = PinDriver::output(peripherals.pins.gpio5)?;
    G_LED.lock().unwrap().get_or_init(move || led);

    let mut button = PinDriver::input(peripherals.pins.gpio4)?;
    // 上拉电阻使能，以防止悬空状态
    button.set_pull(Pull::Up)?;
    // 以检测正边缘上的中断
    button.set_interrupt_type(InterruptType::PosEdge)?;

    unsafe {
        gpio_install_isr_service(ESP_INTR_FLAG_LEVEL1 as i32);

        gpio_isr_handler_add(button.pin(), Some(gpio_isr_handler), std::ptr::null_mut());
    }

    log::info!("loop");
    loop {
        // we are using thread::sleep here to make sure the watchdog isn't triggered
        FreeRtos::delay_ms(1000);
    }
}

// 中断处理函数
unsafe extern "C" fn gpio_isr_handler(_arg: *mut std::ffi::c_void) {
    let mut binding = G_LED.lock().unwrap();
    let led = binding.get_mut().unwrap();
    if led.is_set_high() {
        led.set_low().unwrap()
    } else {
        led.set_high().unwrap()
    }
}
