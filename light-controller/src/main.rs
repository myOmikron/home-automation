use std::ffi::CStr;
use std::str::from_utf8;
use std::thread::sleep;
use std::time::Duration;

use anyhow::Result;
use embedded_svc::mqtt::client::{Event, QoS};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::log::EspLogger;
use esp_idf_svc::mqtt::client::{EspMqttClient, MqttClientConfiguration};
use esp_idf_svc::tls::X509;
// If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_sys as _;

use crate::config::CONFIG;
use crate::wifi::connect_wifi;

mod config;
mod wifi;

const CERT: &'static str = concat!(include_str!("../../vagrant/brain/fullchain.crt"), "\0");

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;

    let app_config = CONFIG;

    let Ok(_wifi) = connect_wifi(
        app_config.wifi_ssid,
        app_config.wifi_psk,
        peripherals.modem,
        sysloop,
    ) else {
        panic!("alarm");
    };

    let broker_url = format!(
        "mqtts://{}:{}@{}",
        app_config.mqtt_user, app_config.mqtt_pass, app_config.mqtt_host
    );

    let mqtt_config = MqttClientConfiguration {
        skip_cert_common_name_check: true,
        server_certificate: Some(X509::pem(
            CStr::from_bytes_with_nul(CERT.as_bytes()).unwrap(),
        )),
        ..Default::default()
    };

    let mut client = EspMqttClient::new(broker_url, &mqtt_config, |res| {
        if let Ok(Event::Received(msg)) = res {
            println!("res {msg:#?} {}", from_utf8(msg.data()).unwrap());
        } else if let Err(e) = res {
            panic!("Mqtt error: {e}");
        }
    })?;

    client
        .subscribe(app_config.mqtt_topic, QoS::ExactlyOnce)
        .unwrap();

    loop {
        sleep(Duration::from_secs(1));
    }
}
