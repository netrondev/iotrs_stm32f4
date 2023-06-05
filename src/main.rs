#![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]

// use panic_halt as _;
use core::panic::PanicInfo;
// use embedded_hal::digital::v2::OutputPin;
use hal::gpio::Speed;
use stm32f4xx_hal as hal;
use w5500::{bus::FourWire, MacAddress};

use crate::hal::{pac, prelude::*};
// use cortex_m_rt::entry;
// use embedded_nal::{IpAddr, Ipv4Addr, SocketAddr};
use rtt_target::{rprintln, rtt_init_print};

// use embedded_nal::TcpClientStack;
use embedded_nal::TcpClientStack;
use smoltcp_nal;

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();
    let p = pac::Peripherals::take().unwrap();

    // let gpioc = p.GPIOC.split();
    // let mut led = gpioc.pc13.into_push_pull_output();

    let rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.freeze();

    let gpioa = p.GPIOA.split();
    let gpiod = p.GPIOD.split();

    let mut led_green = gpiod.pd12.into_push_pull_output();
    // let mut led_orange = gpiod.pd13.into_push_pull_output();
    // let mut led_red = gpiod.pd14.into_push_pull_output();
    // let mut led_blue = gpiod.pd15.into_push_pull_output();

    // let mut cs: OutputPin = p.GPIOA.split().pa3.into_push_pull_output(); // chip select

    // let mut spi1 = p.SPI1.into();

    // let mut spi = hal::spi::Spi::new(p.SPI1); // p.SPI1.spi(pins, mode, freq, clocks);

    // https://github.com/stm32-rs/stm32f4xx-hal/blob/master/examples/spi-dma.rs

    // Note. We set GPIO speed as VeryHigh to it corresponds to SPI frequency 3MHz.
    // Otherwise it may lead to the 'wrong last bit in every received byte' problem.
    // let pb15 = gpiob
    //     .pb15
    //     .into_alternate()
    //     .speed(Speed::VeryHigh)
    //     .internal_pull_up(true);
    // let pb13 = gpiob.pb13.into_alternate().speed(Speed::VeryHigh);

    let mode = hal::spi::Mode {
        polarity: hal::spi::Polarity::IdleLow,
        phase: hal::spi::Phase::CaptureOnFirstTransition,
        // arp_responses: w5500::ArpResponses::DropAfterUse,
        // connection_type: w5500::ConnectionType::Ethernet,
        // on_wake_on_lan: w5500::OnWakeOnLan::Ignore,
        // on_ping_request: w5500::OnPingRequest::Ignore,
    };

    //  {
    //     on_wake_on_lan: ,
    //     on_ping_request,
    //     connection_type,
    //     arp_responses,
    // };

    let sck = gpioa.pa5.into_alternate().speed(Speed::VeryHigh);
    // let miso: Miso = gpioa.pa6.into();
    let miso = gpioa.pa6.into_input();

    // .into_alternate()
    // .speed(Speed::VeryHigh)
    // .internal_pull_up(true);

    let mosi = gpioa.pa7.into_push_pull_output().speed(Speed::VeryHigh);

    let spi2 = hal::spi::Spi::new(p.SPI1, (sck, miso, mosi), mode, 3.MHz(), &clocks);

    let cs = gpioa.pa3.into_push_pull_output();

    // let mut device = w5500::UninitializedDevice::new(FourWire::new(spi2, cs))
    //     .initialize_manual(
    //         MacAddress::new(0, 1, 2, 3, 4, 5),
    //         embedded_nal::Ipv4Addr::new(192, 168, 50, 87),
    //         w5500::Mode::default(),
    //     )
    //     .unwrap();

    // .initialize_macraw(MacAddress::new(0, 1, 2, 3, 4, 5)).unwrap();
    //     // .initialize(MacAddress::new(0, 1, 2, 3, 4, 5), w5500::Mode::default())
    //     // .unwrap();

    //     device.write_frame(frame)

    let version = device.version().unwrap();
    rprintln!("Version {:?}", version);

    let mut counter: usize = 0;
    // let mut socket = device.socket().unwrap();

    // device
    //     .connect(
    //         &mut socket,
    //         embedded_nal::SocketAddr::new(
    //             embedded_nal::IpAddr::V4(embedded_nal::Ipv4Addr::new(192, 168, 50, 86)),
    //             1337,
    //         ),
    //     )
    //     .unwrap();

    let config = smoltcp_nal::smoltcp::iface::Config::new();

    let mut iface = smoltcp_nal::smoltcp::iface::Interface::new(config, &mut device);

    let mut dhcp_socket = smoltcp_nal::smoltcp::socket::dhcpv4::Socket::new();
    dhcp_socket.set_max_lease_duration(Some(smoltcp_nal::smoltcp::time::Duration::from_secs(10)));

    let mut sockets = smoltcp_nal::smoltcp::iface::SocketSet::new([]);
    let dhcp_handle = sockets.add(dhcp_socket);

    loop {
        let timestamp = smoltcp_nal::smoltcp::time::Instant::now();
        iface.poll(timestamp, &mut device, &mut sockets);

        counter += 1;
        rprintln!("Sending {}", counter);

        // device
        //     .send(&mut socket, &[104, 101, 108, 108, 111, 10])
        //     .unwrap();

        let event = sockets
            .get_mut::<smoltcp_nal::smoltcp::socket::dhcpv4::Socket>(dhcp_handle)
            .poll();

        // device.send(&mut socket, &[counter as u8]).unwrap();

        // let mut recvbuf = [0u8; 1024];
        // // let mut asdf: &str = "aaaaaaaaaaaaaa";
        // loop {
        //     let result = device.receive(&mut socket, &mut recvbuf).unwrap();
        //     // asdf = core::str::from_utf8(&recvbuf).unwrap();

        //     if result > 0 {
        //         rprintln!("Data recv {:?} {} {}", result, recvbuf[0], recvbuf[1]);
        //         // rprintln!("Data recv {:?} text:{:?}", result, recvbuf);
        //         // rprintln!("Data recv {} {:?}", result, recvbuf);
        //         break;
        //     }
        // }

        // }
        // device.close(socket).unwrap();

        // for _ in 0..10_000 {
        //     led_green.set_high();
        // }
        // for _ in 0..10_000 {
        //     led_green.set_low();
        // }

        // for _ in 0..10_000 {
        //     led_orange.set_high();
        // }
        // for _ in 0..10_000 {
        //     led_orange.set_low();
        // }

        // for _ in 0..10_000 {
        //     led_red.set_high();
        // }
        // for _ in 0..10_000 {
        //     led_red.set_low();
        // }

        // for _ in 0..10_000 {
        //     led_blue.set_high();
        // }
        // for _ in 0..10_000 {
        //     led_blue.set_low();
        // }

        // counter += 1;

        // if counter > 10 {
        //     panic!("This is an intentional panic.");
        // }
    }
}

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rprintln!("{}", info);
    loop {} // You might need a compiler fence in here.
}
