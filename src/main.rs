extern crate image;

use std::{env, fs::File, io::Read};

use image::{DynamicImage, GenericImage, Rgba};

const COLOR: [[u8; 4]; 4] = [
    [0x00, 0x00, 0x00, 0xff],
    [0x55, 0x55, 0x55, 0xff],
    [0xaa, 0xaa, 0xaa, 0xff],
    [0xff, 0xff, 0xff, 0xff],
];

fn main() {
    // コマンドライン引数を受け取る
    let path: String = match env::args().nth(1) {
        None => panic!("ファイルが指定されていません"),
        Some(s) => s,
    };

    let mut buf = Vec::new();
    // ファイルをオープンし、バイト列として読み込む
    match File::open(path) {
        Ok(mut f) => match f.read_to_end(&mut buf) {
            Ok(_) => {}
            Err(_) => panic!("ファイルの読み込みに失敗しました"),
        },
        Err(_) => panic!("ファイルが見つかりませんでした"),
    };

    // NESファイルかチェック
    if &buf[0..3] != [78, 69, 83] {
        panic!("nesファイルではありません")
    }

    let char_rom_start = 0x10 + buf[4] as u16 * 0x4000;
    let char_rom_end = char_rom_start + buf[5] as u16 * 0x2000;

    println!("{:?} {:?}", char_rom_start, char_rom_end);

    create_character_graphic(&buf[char_rom_start as usize..char_rom_end as usize]);
}

fn create_character_graphic(char_rom: &[u8]) {
    let sprite_num: u32 = (char_rom.len() >> 4) as u32;
    println!("Sprite Num => {:?}", sprite_num);
    let h_num: u32 = 8;
    let w_num: u32 = -(-(sprite_num as i32) / h_num as i32) as u32;
    println!("{:?}, {:?}", w_num, h_num);

    const SPRITE_DSIZE: u32 = 0x10;
    const SPRITE_WSIZE: u32 = 0x8;
    const SPRITE_HSIZE: u32 = 0x8;

    let mut char_img = DynamicImage::new_rgb8(w_num * SPRITE_WSIZE, h_num * SPRITE_HSIZE);

    for i in 0..sprite_num {
        let start = (i * SPRITE_DSIZE) as usize;
        let data = [
            &char_rom[start..(start + (SPRITE_DSIZE / 2) as usize)],
            &char_rom[start + (SPRITE_DSIZE / 2) as usize..start + SPRITE_DSIZE as usize],
        ];
        let x = (i % w_num) * SPRITE_WSIZE;
        let y = (i / w_num) * SPRITE_HSIZE;

        for h in 0..SPRITE_HSIZE {
            for w in 0..SPRITE_WSIZE {
                let d =
                    (((data[1][h as usize] >> w) & 0x1) << 1) + ((data[0][h as usize] >> w) & 0x1);
                char_img.put_pixel(x + SPRITE_WSIZE - w - 1, y + h, Rgba(COLOR[d as usize]));
            }
        }
    }

    match char_img.save("charactor.png") {
        Ok(_) => {}
        Err(_) => panic!("画像の出力に失敗しました"),
    };
}
