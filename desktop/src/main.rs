use anyhow::{anyhow, Result};
use nes_lib::{
    self,
    cart::{header::FileHeader, mapper::INesMapper},
    Bus, NesBus,
};
use parking_lot::Mutex;
use pixels::{Pixels, SurfaceTexture};
use std::sync::Arc;
use std::{env, fs, thread};
use winit::event::WindowEvent;
use winit::{dpi::LogicalSize, event::Event, event_loop::EventLoop, window::WindowBuilder};
use winit_input_helper::WinitInputHelper;

struct DesktopScreen {}

impl DesktopScreen {
    fn new() -> Self {
        Self {}
    }
}

impl nes_lib::Screen for DesktopScreen {
    fn put_pixel(&mut self, row: u16, col: u16, c: nes_lib::Color) {
        println!("r: {} c: {} rgb: {:?}", row, col, c);
    }
}

fn get_on_event<'a>() -> nes_lib::EventCb {
    return move |src| {
        match src {
            //EventSource::BusRead { addr, val } => println!("read {:x} <- ${:x}", val, addr),
            //EventSource::BusWrite { addr, val } => println!("Bus ${:x} = {:x}", addr, val),
            _ => {}
        }
    };
}

fn init_nes(
    rom_bytes: &[u8],
    on_event: nes_lib::EventCb,
) -> Result<nes_lib::Nes<NesBus<INesMapper, DesktopScreen>>> {
    let header =
        FileHeader::try_decode(&rom_bytes[..16]).map_err(|_| anyhow!("failed to decode"))?;
    let cart = match header {
        FileHeader::INes(header) => INesMapper::try_decode(&header, &rom_bytes[16..])
            .map_err(|_| anyhow!("failed to decode"))?,
        FileHeader::Nes2(_) => return Err(anyhow!("Decoded as NES 2.0")),
    };

    let screen = DesktopScreen::new();
    let ppu = nes_lib::Ppu::new(screen);
    let mut bus = NesBus::new(cart, ppu);
    bus.on_event = on_event.clone();

    let mut nes = nes_lib::Nes::new(bus);
    nes.on_event = on_event.clone();

    Ok(nes)
}

fn draw_tile(frame: &mut [u8], tile_idx: usize, tile: &[u8]) {
    debug_assert!(tile_idx < 32 * 16);
    let mut color_idxs = [0_u8; 64];
    for (bit, plane) in tile.chunks_exact(8).enumerate() {
        for row in 0..8 {
            for col in 0..8 {
                let val = ((plane[row] >> (7 - col)) & 1) << bit;
                color_idxs[col + row * 8] |= val;
            }
        }
    }

    let (screen_row_start, mut screen_col_start) =
        (((tile_idx / 16) % 16) * 8, (tile_idx % 16) * 8);
    debug_assert!(screen_col_start + 7 < 128);
    if tile_idx >= 256 {
        screen_col_start += 128;
    }
    debug_assert!(screen_row_start + 7 < 128);
    debug_assert!(screen_col_start + 7 < 256);

    for d_row in 0..8 {
        for d_col in 0..8 {
            let screen_row = screen_row_start + d_row;
            let screen_col = screen_col_start + d_col;
            let screen_index = screen_col + screen_row * 256;
            let color_index = d_col + d_row * 8;
            let (r, g, b): (u8, u8, u8) = match color_idxs[color_index] {
                0b00 => (0, 0, 0),
                0b01 => (255, 0, 0),
                0b10 => (0, 255, 0),
                0b11 => (0, 0, 255),
                _ => unreachable!(),
            };
            frame[screen_index * 4] = r;
            frame[screen_index * 4 + 1] = g;
            frame[screen_index * 4 + 2] = b;
            frame[screen_index * 4 + 3] = 255;
        }
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];
    let rom_bytes = fs::read(filepath)?;

    let event_loop = EventLoop::new()?;
    let window = {
        let size = LogicalSize::new(256, 128);
        let scaled_size = LogicalSize::new(256 as f64 * 3.0, 128 as f64 * 3.0);
        WindowBuilder::new()
            .with_title("nes-emu")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };
    let mut input = WinitInputHelper::new();

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(256 as u32, 128 as u32, surface_texture)?
    };

    let chr = Arc::new(Mutex::new([0_u8; 0x2000]));

    {
        let chr = chr.clone();
        let mut nes = init_nes(&rom_bytes, get_on_event()).unwrap();
        thread::spawn(move || {
            println!("PC starting at {:X}", nes.cpu.pc);

            loop {
                for _ in 0..20_000_000 / 60 {
                    //println!("pc: {:x}", nes.cpu.pc);
                    if nes.cpu.jammed {
                        println!("nes is jammed");
                        break;
                    }
                    nes.tick();
                }
                *chr.lock() = nes.bus.debug_chr().unwrap();
            }
        });
    }

    {
        let chr = chr.clone();
        event_loop
            .run(move |event, elwt| {
                if let Event::WindowEvent { ref event, .. } = event {
                    match event {
                        WindowEvent::RedrawRequested => {
                            // Draw stuff
                            let frame = pixels.frame_mut();
                            let chr = chr.lock();

                            for (tile_idx, tile) in chr.chunks_exact(16).enumerate() {
                                draw_tile(frame, tile_idx, tile);
                            }

                            if let Err(err) = pixels.render() {
                                eprintln!("{}", err);
                                elwt.exit();
                                return;
                            }
                        }
                        WindowEvent::CloseRequested => {
                            elwt.exit();
                            return;
                        }
                        _ => {}
                    }
                }

                if input.update(&event) {
                    window.request_redraw();
                }
            })
            .map_err(|err| anyhow!(err))
    }
}
