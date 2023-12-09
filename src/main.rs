mod adapter;
mod delay_adapter;
mod iv;

use adapter::BlockingAsync;
use delay_adapter::WithDelayNs;
use embedded_hal_async::spi::SpiBus;
use embedded_hal_async::spi::{Operation, SpiDevice};
use embedded_hal_bus::spi::ExclusiveDevice;
use iv::GenericSx127xInterfaceVariant;
use lora_phy::{
    mod_params::{Bandwidth, CodingRate, SpreadingFactor},
    sx1276_7_8_9::{self, SX1276_7_8_9},
    LoRa,
};
use rppal::{
    gpio::{Gpio, Trigger},
    hal::Delay,
    spi::{Bus, Mode, SlaveSelect, Spi},
};
use std::{borrow::BorrowMut, error::Error, time::Duration};

pub enum NodeType {
    Drone,
    Gateway,
}
impl NodeType {
    pub fn from_str(s: &str) -> Result<NodeType, ()> {
        match s {
            "Drone" => Ok(NodeType::Drone),
            "Gateway" => Ok(NodeType::Gateway),
            _ => Err(println!("Invalid node type")),
        }
    }
}

const LORA_CS_PIN: u8 = 25;
const LORA_RESET_PIN: u8 = 17;
const LORA_DIO0_PIN: u8 = 4;
const LORA_DIO1_PIN: u8 = 23;
const LORA_DIO2_PIN: u8 = 24;
// const FREQUENCY: i64 = 868;
const LORA_FREQUENCY_IN_HZ: u32 = 868_000_000; // warning: set this appropriately for the region
#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let node_type = NodeType::from_str(&args[1]).unwrap();
    let gpio = Gpio::new().unwrap();
    let mut reset = gpio.get(LORA_RESET_PIN).unwrap().into_output();
    reset.set_high();
    let dio0 = gpio.get(LORA_DIO0_PIN).unwrap().into_input_pullup();
    let mut nss = gpio.get(LORA_CS_PIN).unwrap().into_output();
    let spi_bus =
        BlockingAsync::new(Spi::new(Bus::Spi0, SlaveSelect::Ss0, 20_000, Mode::Mode0).unwrap());
    let mut spi = ExclusiveDevice::new(spi_bus, nss, WithDelayNs::new(Delay));
    println!("spi created");

    let mut rx_buf = [0_u8; 1];
    let mut ops = [
        Operation::Write(&[0x40 & 0x7f]),
        Operation::Read(&mut rx_buf),
    ];
    spi.transaction(&mut ops).await.unwrap();
    println!(">> received: {:02X?}", rx_buf);

    let config = sx1276_7_8_9::Config {
        chip: sx1276_7_8_9::Sx127xVariant::Sx1276,
        tcxo_used: false,
    };
    let iv = GenericSx127xInterfaceVariant::new(reset, dio0, None, None).unwrap();
    let mut lora = LoRa::new(
        SX1276_7_8_9::new(spi, iv, config),
        false,
        WithDelayNs::new(Delay),
    )
    .await
    .unwrap();
    println!("lora created");
    let mdltn_params = {
        match lora.create_modulation_params(
            SpreadingFactor::_7,
            Bandwidth::_125KHz,
            CodingRate::_4_5,
            LORA_FREQUENCY_IN_HZ,
        ) {
            Ok(mp) => mp,
            Err(err) => {
                println!("Radio error = {:?}", err);
                return;
            }
        }
    };

    match node_type {
        NodeType::Drone => {
            let mut tx_pkt_params = {
                match lora.create_tx_packet_params(4, false, true, false, &mdltn_params) {
                    Ok(pp) => pp,
                    Err(err) => {
                        println!("Radio error = {:?}", err);
                        return;
                    }
                }
            };

            match lora.prepare_for_tx(&mdltn_params, 20, false).await {
                Ok(()) => {}
                Err(err) => {
                    println!("Radio error = {:?}", err);
                    return;
                }
            };

            let buffer = [0x01u8, 0x02u8, 0x03u8];
            println!("sending buffer = {:?}", buffer);
            match lora
                .tx(&mdltn_params, &mut tx_pkt_params, &buffer, 100)
                .await
            {
                Ok(()) => {
                    println!("TX DONE");
                }
                Err(err) => {
                    println!("Radio error = {:?}", err);
                    return;
                }
            };
            println!("tx successful");

            match lora.sleep(false).await {
                Ok(()) => println!("Sleep successful"),
                Err(err) => println!("Sleep unsuccessful = {:?}", err),
            }
        }
        NodeType::Gateway => {
            let mut receiving_buffer = [00u8; 100];

            let rx_pkt_params = {
                match lora.create_rx_packet_params(
                    4,
                    false,
                    receiving_buffer.len() as u8,
                    true,
                    false,
                    &mdltn_params,
                ) {
                    Ok(pp) => pp,
                    Err(err) => {
                        println!("Radio error = {:?}", err);
                        return;
                    }
                }
            };

            match lora
                .prepare_for_rx(&mdltn_params, &rx_pkt_params, None, None, false)
                .await
            {
                Ok(()) => {}
                Err(err) => {
                    println!("Radio error = {:?}", err);
                    return;
                }
            };

            loop {
                println!("waiting for rx");
                receiving_buffer = [00u8; 100];
                match lora.rx(&rx_pkt_params, &mut receiving_buffer).await {
                    Ok((received_len, _rx_pkt_status)) => {
                        if (received_len == 3)
                            && (receiving_buffer[0] == 0x01u8)
                            && (receiving_buffer[1] == 0x02u8)
                            && (receiving_buffer[2] == 0x03u8)
                        {
                            println!("rx successful");
                        } else {
                            println!("rx unknown packet");
                        }
                    }
                    Err(err) => println!("rx unsuccessful = {:?}", err),
                }
            }
        }
    }
}
