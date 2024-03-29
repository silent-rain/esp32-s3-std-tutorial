use wifi::wifi;

use anyhow::bail;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::{delay::FreeRtos, peripherals::Peripherals},
    log::EspLogger,
    sys::link_patches,
};

/// This configuration is picked up at compile time by `build.rs` from the
/// file `cfg.toml`.
#[toml_cfg::toml_config]
pub struct Config {
    #[default("wifi-GUEST")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

fn main() -> anyhow::Result<()> {
    link_patches();

    EspLogger::initialize_default();
    log::set_max_level(log::LevelFilter::Info);

    // Get the peripherals
    let peripherals = Peripherals::take()?;
    let sysloop = EspSystemEventLoop::take()?;

    // The constant `CONFIG` is auto-generated by `toml_config`.
    let app_config = CONFIG;

    log::info!(
        "======= wifi_ssid: {}, wifi_psk: {}",
        app_config.wifi_ssid,
        app_config.wifi_psk
    );

    // Connect to the Wi-Fi network
    let _wifi = match wifi(
        app_config.wifi_ssid,
        app_config.wifi_psk,
        peripherals.modem,
        sysloop,
    ) {
        Ok(inner) => inner,
        Err(err) => {
            bail!("Could not connect to Wi-Fi network: {:?}", err)
        }
    };

    log::info!("loop");
    loop {
        FreeRtos::delay_ms(1000);
    }
}
