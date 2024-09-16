const PALETTE_BYTES: [u8; 256 * 3] = [
    0, 0, 0, 1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7, 8, 8, 8, 9, 9, 9, 23, 35, 35, 35, 51, 51,
    47, 67, 67, 63, 83, 83, 75, 99, 99, 91, 115, 115, 111, 131, 131, 131, 151, 151, 159, 175, 175, 183, 195, 195, 211,
    219, 219, 239, 243, 243, 51, 47, 0, 63, 59, 0, 79, 75, 11, 91, 91, 19, 107, 107, 31, 119, 123, 47, 135, 139, 59,
    151, 155, 79, 167, 175, 95, 187, 191, 115, 203, 207, 139, 223, 227, 163, 67, 43, 7, 87, 59, 11, 111, 75, 23, 127,
    87, 31, 143, 99, 39, 159, 115, 51, 179, 131, 67, 191, 151, 87, 203, 175, 111, 219, 199, 135, 231, 219, 163, 247,
    239, 195, 71, 27, 0, 95, 43, 0, 119, 63, 0, 143, 83, 7, 167, 111, 7, 191, 139, 15, 215, 167, 19, 243, 203, 27, 255,
    231, 47, 255, 243, 95, 255, 251, 143, 255, 255, 195, 35, 0, 0, 79, 0, 0, 95, 7, 7, 111, 15, 15, 127, 27, 27, 143,
    39, 39, 163, 59, 59, 179, 79, 79, 199, 103, 103, 215, 127, 127, 235, 159, 159, 255, 191, 191, 27, 51, 19, 35, 63,
    23, 47, 79, 31, 59, 95, 39, 71, 111, 43, 87, 127, 51, 99, 143, 59, 115, 155, 67, 131, 171, 75, 147, 187, 83, 163,
    203, 95, 183, 219, 103, 31, 55, 27, 47, 71, 35, 59, 83, 43, 75, 99, 55, 91, 111, 67, 111, 135, 79, 135, 159, 95,
    159, 183, 111, 183, 207, 127, 195, 219, 147, 207, 231, 167, 223, 247, 191, 15, 63, 0, 19, 83, 0, 23, 103, 0, 31,
    123, 0, 39, 143, 7, 55, 159, 23, 71, 175, 39, 91, 191, 63, 111, 207, 87, 139, 223, 115, 163, 239, 143, 195, 255,
    179, 79, 43, 19, 99, 55, 27, 119, 71, 43, 139, 87, 59, 167, 99, 67, 187, 115, 83, 207, 131, 99, 215, 151, 115, 227,
    171, 131, 239, 191, 151, 247, 207, 171, 255, 227, 195, 15, 19, 55, 39, 43, 87, 51, 55, 103, 63, 67, 119, 83, 83,
    139, 99, 99, 155, 119, 119, 175, 139, 139, 191, 159, 159, 207, 183, 183, 223, 211, 211, 239, 239, 239, 255, 0, 27,
    111, 0, 39, 151, 7, 51, 167, 15, 67, 187, 27, 83, 203, 43, 103, 223, 67, 135, 227, 91, 163, 231, 119, 187, 239,
    143, 211, 243, 175, 231, 251, 215, 247, 255, 11, 43, 15, 15, 55, 23, 23, 71, 31, 35, 83, 43, 47, 99, 59, 59, 115,
    75, 79, 135, 95, 99, 155, 119, 123, 175, 139, 147, 199, 167, 175, 219, 195, 207, 243, 223, 63, 0, 95, 75, 7, 115,
    83, 15, 127, 95, 31, 143, 107, 43, 155, 123, 63, 171, 135, 83, 187, 155, 103, 199, 171, 127, 215, 191, 155, 231,
    215, 195, 243, 243, 235, 255, 63, 0, 0, 87, 0, 0, 115, 0, 0, 143, 0, 0, 171, 0, 0, 199, 0, 0, 227, 7, 0, 255, 7, 0,
    255, 79, 67, 255, 123, 115, 255, 171, 163, 255, 219, 215, 79, 39, 0, 111, 51, 0, 147, 63, 0, 183, 71, 0, 219, 79,
    0, 255, 83, 0, 255, 111, 23, 255, 139, 51, 255, 163, 79, 255, 183, 107, 255, 203, 135, 255, 219, 163, 0, 51, 47, 0,
    63, 55, 0, 75, 67, 0, 87, 79, 7, 107, 99, 23, 127, 119, 43, 147, 143, 71, 167, 163, 99, 187, 187, 131, 207, 207,
    171, 231, 231, 207, 255, 255, 63, 0, 27, 103, 0, 51, 123, 11, 63, 143, 23, 79, 163, 31, 95, 183, 39, 111, 219, 59,
    143, 239, 91, 171, 243, 119, 187, 247, 151, 203, 251, 183, 223, 255, 215, 239, 39, 19, 0, 55, 31, 7, 71, 47, 15,
    91, 63, 31, 107, 83, 51, 123, 103, 75, 143, 127, 107, 163, 147, 127, 187, 171, 147, 207, 195, 171, 231, 219, 195,
    255, 243, 223, 55, 75, 75, 255, 183, 0, 255, 219, 0, 255, 255, 0, 7, 107, 99, 7, 107, 100, 39, 143, 135, 27, 131,
    123, 7, 107, 101, 55, 155, 151, 55, 155, 152, 155, 227, 227, 115, 203, 203, 55, 155, 153, 67, 91, 91, 83, 107, 107,
    99, 123, 123, 111, 51, 47, 131, 55, 47, 151, 63, 51, 171, 67, 51, 191, 75, 47, 211, 79, 43, 231, 87, 35, 255, 95,
    31, 255, 127, 39, 255, 155, 51, 255, 183, 63, 255, 207, 75, 255, 255, 255,
];

const PALETTE_COLORS: [(u8, u8, u8); 256] = [
    (0, 0, 0),
    (1, 1, 1),
    (2, 2, 2),
    (3, 3, 3),
    (4, 4, 4),
    (5, 5, 5),
    (6, 6, 6),
    (7, 7, 7),
    (8, 8, 8),
    (9, 9, 9),
    (23, 35, 35),
    (35, 51, 51),
    (47, 67, 67),
    (63, 83, 83),
    (75, 99, 99),
    (91, 115, 115),
    (111, 131, 131),
    (131, 151, 151),
    (159, 175, 175),
    (183, 195, 195),
    (211, 219, 219),
    (239, 243, 243),
    (51, 47, 0),
    (63, 59, 0),
    (79, 75, 11),
    (91, 91, 19),
    (107, 107, 31),
    (119, 123, 47),
    (135, 139, 59),
    (151, 155, 79),
    (167, 175, 95),
    (187, 191, 115),
    (203, 207, 139),
    (223, 227, 163),
    (67, 43, 7),
    (87, 59, 11),
    (111, 75, 23),
    (127, 87, 31),
    (143, 99, 39),
    (159, 115, 51),
    (179, 131, 67),
    (191, 151, 87),
    (203, 175, 111),
    (219, 199, 135),
    (231, 219, 163),
    (247, 239, 195),
    (71, 27, 0),
    (95, 43, 0),
    (119, 63, 0),
    (143, 83, 7),
    (167, 111, 7),
    (191, 139, 15),
    (215, 167, 19),
    (243, 203, 27),
    (255, 231, 47),
    (255, 243, 95),
    (255, 251, 143),
    (255, 255, 195),
    (35, 0, 0),
    (79, 0, 0),
    (95, 7, 7),
    (111, 15, 15),
    (127, 27, 27),
    (143, 39, 39),
    (163, 59, 59),
    (179, 79, 79),
    (199, 103, 103),
    (215, 127, 127),
    (235, 159, 159),
    (255, 191, 191),
    (27, 51, 19),
    (35, 63, 23),
    (47, 79, 31),
    (59, 95, 39),
    (71, 111, 43),
    (87, 127, 51),
    (99, 143, 59),
    (115, 155, 67),
    (131, 171, 75),
    (147, 187, 83),
    (163, 203, 95),
    (183, 219, 103),
    (31, 55, 27),
    (47, 71, 35),
    (59, 83, 43),
    (75, 99, 55),
    (91, 111, 67),
    (111, 135, 79),
    (135, 159, 95),
    (159, 183, 111),
    (183, 207, 127),
    (195, 219, 147),
    (207, 231, 167),
    (223, 247, 191),
    (15, 63, 0),
    (19, 83, 0),
    (23, 103, 0),
    (31, 123, 0),
    (39, 143, 7),
    (55, 159, 23),
    (71, 175, 39),
    (91, 191, 63),
    (111, 207, 87),
    (139, 223, 115),
    (163, 239, 143),
    (195, 255, 179),
    (79, 43, 19),
    (99, 55, 27),
    (119, 71, 43),
    (139, 87, 59),
    (167, 99, 67),
    (187, 115, 83),
    (207, 131, 99),
    (215, 151, 115),
    (227, 171, 131),
    (239, 191, 151),
    (247, 207, 171),
    (255, 227, 195),
    (15, 19, 55),
    (39, 43, 87),
    (51, 55, 103),
    (63, 67, 119),
    (83, 83, 139),
    (99, 99, 155),
    (119, 119, 175),
    (139, 139, 191),
    (159, 159, 207),
    (183, 183, 223),
    (211, 211, 239),
    (239, 239, 255),
    (0, 27, 111),
    (0, 39, 151),
    (7, 51, 167),
    (15, 67, 187),
    (27, 83, 203),
    (43, 103, 223),
    (67, 135, 227),
    (91, 163, 231),
    (119, 187, 239),
    (143, 211, 243),
    (175, 231, 251),
    (215, 247, 255),
    (11, 43, 15),
    (15, 55, 23),
    (23, 71, 31),
    (35, 83, 43),
    (47, 99, 59),
    (59, 115, 75),
    (79, 135, 95),
    (99, 155, 119),
    (123, 175, 139),
    (147, 199, 167),
    (175, 219, 195),
    (207, 243, 223),
    (63, 0, 95),
    (75, 7, 115),
    (83, 15, 127),
    (95, 31, 143),
    (107, 43, 155),
    (123, 63, 171),
    (135, 83, 187),
    (155, 103, 199),
    (171, 127, 215),
    (191, 155, 231),
    (215, 195, 243),
    (243, 235, 255),
    (63, 0, 0),
    (87, 0, 0),
    (115, 0, 0),
    (143, 0, 0),
    (171, 0, 0),
    (199, 0, 0),
    (227, 7, 0),
    (255, 7, 0),
    (255, 79, 67),
    (255, 123, 115),
    (255, 171, 163),
    (255, 219, 215),
    (79, 39, 0),
    (111, 51, 0),
    (147, 63, 0),
    (183, 71, 0),
    (219, 79, 0),
    (255, 83, 0),
    (255, 111, 23),
    (255, 139, 51),
    (255, 163, 79),
    (255, 183, 107),
    (255, 203, 135),
    (255, 219, 163),
    (0, 51, 47),
    (0, 63, 55),
    (0, 75, 67),
    (0, 87, 79),
    (7, 107, 99),
    (23, 127, 119),
    (43, 147, 143),
    (71, 167, 163),
    (99, 187, 187),
    (131, 207, 207),
    (171, 231, 231),
    (207, 255, 255),
    (63, 0, 27),
    (103, 0, 51),
    (123, 11, 63),
    (143, 23, 79),
    (163, 31, 95),
    (183, 39, 111),
    (219, 59, 143),
    (239, 91, 171),
    (243, 119, 187),
    (247, 151, 203),
    (251, 183, 223),
    (255, 215, 239),
    (39, 19, 0),
    (55, 31, 7),
    (71, 47, 15),
    (91, 63, 31),
    (107, 83, 51),
    (123, 103, 75),
    (143, 127, 107),
    (163, 147, 127),
    (187, 171, 147),
    (207, 195, 171),
    (231, 219, 195),
    (255, 243, 223),
    (55, 75, 75),
    (255, 183, 0),
    (255, 219, 0),
    (255, 255, 0),
    (7, 107, 99),
    (7, 107, 100),
    (39, 143, 135),
    (27, 131, 123),
    (7, 107, 101),
    (55, 155, 151),
    (55, 155, 152),
    (155, 227, 227),
    (115, 203, 203),
    (55, 155, 153),
    (67, 91, 91),
    (83, 107, 107),
    (99, 123, 123),
    (111, 51, 47),
    (131, 55, 47),
    (151, 63, 51),
    (171, 67, 51),
    (191, 75, 47),
    (211, 79, 43),
    (231, 87, 35),
    (255, 95, 31),
    (255, 127, 39),
    (255, 155, 51),
    (255, 183, 63),
    (255, 207, 75),
    (255, 255, 255),
];

const PALETTE_TRANSPARENT: [u8; 256] = [
    0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255,
];

fn output_png(file_path: &std::path::Path, width: u32, height: u32, bytes: &[u8]) {
    let output_file = std::fs::File::create(file_path).unwrap();
    let output_file_buffer_writer = &mut std::io::BufWriter::new(output_file);

    let mut png_encoder = png::Encoder::new(output_file_buffer_writer, width, height);
    png_encoder.set_color(png::ColorType::Indexed);
    png_encoder.set_depth(png::BitDepth::Eight);
    png_encoder.set_palette(&PALETTE_BYTES);
    png_encoder.set_trns(&PALETTE_TRANSPARENT);

    let mut writer = png_encoder.write_header().unwrap();

    writer.write_image_data(bytes).unwrap();
}

fn convert_sprite(input_path: &std::path::Path, output_path: &std::path::Path, front: bool) {
    std::fs::copy(input_path, output_path).unwrap();

    let input_png_decoder = png::Decoder::new(std::fs::File::open(input_path).unwrap());

    let mut input_png_reader = input_png_decoder.read_info().unwrap();
    let mut input_png_buffer = vec![0; input_png_reader.output_buffer_size()];
    let input_png_frame = input_png_reader.next_frame(&mut input_png_buffer).unwrap();
    let input_png_bytes = &input_png_buffer[..input_png_frame.buffer_size()];
    let input_png_info = input_png_reader.info();
    let input_png_palette = input_png_info.palette.as_ref().unwrap();
    let input_png_palette_transparent = input_png_info.trns.as_ref().unwrap();

    let mut output_png_bytes = Vec::with_capacity(input_png_bytes.len());
    for pixel in input_png_bytes {
        let a = if input_png_palette_transparent.len() > usize::from(*pixel) {
            *input_png_palette_transparent.get(usize::from(*pixel)).unwrap()
        } else {
            255
        };

        if a == 0 {
            output_png_bytes.push(0);
        } else {
            let color = (
                *input_png_palette.get(usize::from(*pixel) * 3).unwrap(),
                *input_png_palette.get((usize::from(*pixel) * 3) + 1).unwrap(),
                *input_png_palette.get((usize::from(*pixel) * 3) + 2).unwrap(),
            );

            let output_pixel = u8::try_from(PALETTE_COLORS.iter().position(|x| *x == color).unwrap()).unwrap();
            output_png_bytes.push(output_pixel);
        }
    }

    let mut rail_bytes = Vec::with_capacity(input_png_bytes.len());
    for pixel in &output_png_bytes {
        if *pixel >= 243 && *pixel <= 254 {
            rail_bytes.push(*pixel);
        } else {
            rail_bytes.push(0);
        }
    }

    let mut foreground_bytes = Vec::with_capacity(output_png_bytes.len());
    for pixel in &output_png_bytes {
        if *pixel >= 46 && *pixel <= 57 {
            foreground_bytes.push(*pixel + 197);
        } else {
            foreground_bytes.push(0);
        }
    }

    for pixel in &mut output_png_bytes {
        if *pixel >= 46 && *pixel <= 57 {
            *pixel += 197;
        }
    }

    output_png(output_path, input_png_info.width, input_png_info.height, &output_png_bytes);

    let rail_file_name = output_path.file_stem().unwrap().to_str().unwrap().to_owned() + "_rails.png";
    let mut rail_file_path = output_path.to_owned();
    rail_file_path.set_file_name(rail_file_name.clone());
    output_png(&rail_file_path, input_png_info.width, input_png_info.height, &rail_bytes);

    if front {
        let rail_file_name = output_path.file_stem().unwrap().to_str().unwrap().to_owned() + "_front.png";
        let mut rail_file_path = output_path.to_owned();
        rail_file_path.set_file_name(rail_file_name.clone());
        output_png(&rail_file_path, input_png_info.width, input_png_info.height, &foreground_bytes);

        let rail_file_name = output_path.file_stem().unwrap().to_str().unwrap().to_owned() + "_front_rails.png";
        let mut rail_file_path = output_path.to_owned();
        rail_file_path.set_file_name(rail_file_name.clone());
        output_png(&rail_file_path, input_png_info.width, input_png_info.height, &rail_bytes);
    }
}

fn update_json(
    input: &[serde_json::Value],
    output: &mut [serde_json::Value],
    input_path: &str,
    output_path: &str,
    y_offset: i64,
) {
    let input_path = "sprites/".to_owned() + input_path;
    let output_path = "track/wooden/".to_owned() + output_path;
    for sprite in input {
        if let Some(path) = sprite.get("path") {
            if *path == input_path {
                for output_sprite in output.iter_mut() {
                    if let Some(path) = output_sprite.get("path") {
                        if *path == output_path {
                            *output_sprite.get_mut("x").unwrap() = sprite.get("x").unwrap().clone();
                            *output_sprite.get_mut("y").unwrap() =
                                serde_json::json!(sprite.get("y").unwrap().as_i64().unwrap() + y_offset);
                        }
                    }
                }
            }
        }
    }
}

fn crop_sprite(sprite_json: &mut [serde_json::Value], sprite_path: &str, sprite_dir: &std::path::Path) {
    let png_path = sprite_dir.join(sprite_path);
    let input_png_decoder = png::Decoder::new(std::fs::File::open(&png_path).unwrap());

    let mut input_png_reader = input_png_decoder.read_info().unwrap();
    let mut input_png_buffer = vec![0; input_png_reader.output_buffer_size()];
    let input_png_frame = input_png_reader.next_frame(&mut input_png_buffer).unwrap();
    let input_png_bytes = &input_png_buffer[..input_png_frame.buffer_size()];
    let input_png_info = input_png_reader.info();

    let bounds_left = {
        let mut bounds_left = 0;
        'outer: for x in 0..input_png_info.width {
            for y in 0..input_png_info.height {
                if input_png_bytes[((y * input_png_info.width) + x) as usize] != 0 {
                    bounds_left = x;
                    break 'outer;
                }
            }
        }
        bounds_left
    };
    let bounds_top = {
        let mut bounds_top = 0;
        'outer: for y in 0..input_png_info.height {
            for x in 0..input_png_info.width {
                if input_png_bytes[((y * input_png_info.width) + x) as usize] != 0 {
                    bounds_top = y;
                    break 'outer;
                }
            }
        }
        bounds_top
    };
    let bounds_right = {
        let mut bounds_right = 0;
        'outer: for x in (0..input_png_info.width).rev() {
            for y in 0..input_png_info.height {
                if input_png_bytes[((y * input_png_info.width) + x) as usize] != 0 {
                    bounds_right = x;
                    break 'outer;
                }
            }
        }
        bounds_right + 1
    };
    let bounds_bottom = {
        let mut bounds_bottom = 0;
        'outer: for y in (0..input_png_info.height).rev() {
            for x in 0..input_png_info.width {
                if input_png_bytes[((y * input_png_info.width) + x) as usize] != 0 {
                    bounds_bottom = y;
                    break 'outer;
                }
            }
        }
        bounds_bottom + 1
    };

    let mut output_png_bytes = Vec::with_capacity(input_png_bytes.len());
    for y in bounds_top..bounds_bottom {
        for x in bounds_left..bounds_right {
            output_png_bytes.push(input_png_bytes[((y * input_png_info.width) + x) as usize]);
        }
    }

    output_png(&png_path, bounds_right - bounds_left, bounds_bottom - bounds_top, &output_png_bytes);

    for sprite in sprite_json {
        if let Some(path) = sprite.get("path") {
            if path == sprite_path {
                *sprite.get_mut("x").unwrap() =
                    serde_json::json!(sprite.get("x").unwrap().as_i64().unwrap() + i64::from(bounds_left));
                *sprite.get_mut("y").unwrap() =
                    serde_json::json!(sprite.get("y").unwrap().as_i64().unwrap() + i64::from(bounds_top));
            }
        }
    }
}

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: CliCommands,
}

#[allow(clippy::enum_variant_names)]
#[derive(clap::Subcommand)]
enum CliCommands {
    Convert {
        source_path: std::path::PathBuf,
        openrct2_path: std::path::PathBuf,
    },
    Crop {
        openrct2_path: std::path::PathBuf,
    },
}

fn main() {
    use clap::Parser;
    let cli = Cli::parse();

    match &cli.command {
        CliCommands::Convert {
            source_path,
            openrct2_path,
        } => {
            let f_to_s_path = source_path.join("models").join("flat to steep").join("sprites");
            let output_path = openrct2_path.join("resources").join("g2").join("track").join("wooden");
            convert_sprite(&f_to_s_path.join("sprite_12.png"), &output_path.join("flat_to_steep_up_1_1.png"), false);
            convert_sprite(&f_to_s_path.join("sprite_8.png"), &output_path.join("flat_to_steep_up_1_2.png"), false);
            convert_sprite(&f_to_s_path.join("sprite_4.png"), &output_path.join("flat_to_steep_up_1_3.png"), false);
            convert_sprite(&f_to_s_path.join("sprite_0.png"), &output_path.join("flat_to_steep_up_1_4.png"), false);
            convert_sprite(&f_to_s_path.join("sprite_13.png"), &output_path.join("flat_to_steep_up_2_1.png"), true);
            convert_sprite(&f_to_s_path.join("sprite_9.png"), &output_path.join("flat_to_steep_up_2_2.png"), true);
            convert_sprite(&f_to_s_path.join("sprite_5.png"), &output_path.join("flat_to_steep_up_2_3.png"), true);
            convert_sprite(&f_to_s_path.join("sprite_1.png"), &output_path.join("flat_to_steep_up_2_4.png"), true);
            convert_sprite(&f_to_s_path.join("sprite_14.png"), &output_path.join("flat_to_steep_up_3_1.png"), true);
            convert_sprite(&f_to_s_path.join("sprite_10.png"), &output_path.join("flat_to_steep_up_3_2.png"), true);
            convert_sprite(&f_to_s_path.join("sprite_6.png"), &output_path.join("flat_to_steep_up_3_3.png"), true);
            convert_sprite(&f_to_s_path.join("sprite_2.png"), &output_path.join("flat_to_steep_up_3_4.png"), true);
            convert_sprite(&f_to_s_path.join("sprite_15.png"), &output_path.join("flat_to_steep_up_4_1.png"), false);
            convert_sprite(&f_to_s_path.join("sprite_11.png"), &output_path.join("flat_to_steep_up_4_2.png"), false);
            convert_sprite(&f_to_s_path.join("sprite_7.png"), &output_path.join("flat_to_steep_up_4_3.png"), false);
            convert_sprite(&f_to_s_path.join("sprite_3.png"), &output_path.join("flat_to_steep_up_4_4.png"), false);

            let s_to_f_path = source_path.join("models").join("steep to flat").join("sprites");
            convert_sprite(&s_to_f_path.join("sprite_12.png"), &output_path.join("steep_to_flat_up_1_1.png"), false);
            convert_sprite(&s_to_f_path.join("sprite_8.png"), &output_path.join("steep_to_flat_up_1_2.png"), false);
            convert_sprite(&s_to_f_path.join("sprite_4.png"), &output_path.join("steep_to_flat_up_1_3.png"), false);
            convert_sprite(&s_to_f_path.join("sprite_0.png"), &output_path.join("steep_to_flat_up_1_4.png"), false);
            convert_sprite(&s_to_f_path.join("sprite_13.png"), &output_path.join("steep_to_flat_up_2_1.png"), true);
            convert_sprite(&s_to_f_path.join("sprite_9.png"), &output_path.join("steep_to_flat_up_2_2.png"), true);
            convert_sprite(&s_to_f_path.join("sprite_5.png"), &output_path.join("steep_to_flat_up_2_3.png"), true);
            convert_sprite(&s_to_f_path.join("sprite_1.png"), &output_path.join("steep_to_flat_up_2_4.png"), true);
            convert_sprite(&s_to_f_path.join("sprite_14.png"), &output_path.join("steep_to_flat_up_3_1.png"), true);
            convert_sprite(&s_to_f_path.join("sprite_10.png"), &output_path.join("steep_to_flat_up_3_2.png"), true);
            convert_sprite(&s_to_f_path.join("sprite_6.png"), &output_path.join("steep_to_flat_up_3_3.png"), true);
            convert_sprite(&s_to_f_path.join("sprite_2.png"), &output_path.join("steep_to_flat_up_3_4.png"), true);
            convert_sprite(&s_to_f_path.join("sprite_15.png"), &output_path.join("steep_to_flat_up_4_1.png"), false);
            convert_sprite(&s_to_f_path.join("sprite_11.png"), &output_path.join("steep_to_flat_up_4_2.png"), false);
            convert_sprite(&s_to_f_path.join("sprite_7.png"), &output_path.join("steep_to_flat_up_4_3.png"), false);
            convert_sprite(&s_to_f_path.join("sprite_3.png"), &output_path.join("steep_to_flat_up_4_4.png"), false);

            let openrct2_sprites_path = openrct2_path.join("resources").join("g2").join("sprites.json");
            let openrct2_sprites_string = std::fs::read_to_string(&openrct2_sprites_path).unwrap();
            let mut openrct2_sprites: serde_json::Value = serde_json::from_str(&openrct2_sprites_string).unwrap();
            let orct2_sprites = openrct2_sprites.as_array_mut().unwrap();

            let f_to_s_sprites_string = std::fs::read_to_string(f_to_s_path.with_extension("json")).unwrap();
            let f_to_s_sprites: serde_json::Value = serde_json::from_str(&f_to_s_sprites_string).unwrap();

            let s_to_f_sprites_string = std::fs::read_to_string(s_to_f_path.with_extension("json")).unwrap();
            let s_to_f_sprites: serde_json::Value = serde_json::from_str(&s_to_f_sprites_string).unwrap();

            let f_to_s_sprites = f_to_s_sprites.as_array().unwrap();
            let s_to_f_sprites = s_to_f_sprites.as_array().unwrap();

            update_json(f_to_s_sprites, orct2_sprites, "sprite_12.png", "flat_to_steep_up_1_1.png", 0);
            update_json(f_to_s_sprites, orct2_sprites, "sprite_12.png", "flat_to_steep_up_1_1_rails.png", 0);
            update_json(f_to_s_sprites, orct2_sprites, "sprite_8.png", "flat_to_steep_up_1_2.png", 0);
            update_json(f_to_s_sprites, orct2_sprites, "sprite_8.png", "flat_to_steep_up_1_2_rails.png", 0);
            update_json(f_to_s_sprites, orct2_sprites, "sprite_4.png", "flat_to_steep_up_1_3.png", 16);
            update_json(f_to_s_sprites, orct2_sprites, "sprite_4.png", "flat_to_steep_up_1_3_rails.png", 16);
            update_json(f_to_s_sprites, orct2_sprites, "sprite_0.png", "flat_to_steep_up_1_4.png", 40);
            update_json(f_to_s_sprites, orct2_sprites, "sprite_0.png", "flat_to_steep_up_1_4_rails.png", 40);

            update_json(f_to_s_sprites, orct2_sprites, "sprite_13.png", "flat_to_steep_up_2_1.png", 0);
            update_json(f_to_s_sprites, orct2_sprites, "sprite_13.png", "flat_to_steep_up_2_1_rails.png", 0);
            update_json(f_to_s_sprites, orct2_sprites, "sprite_13.png", "flat_to_steep_up_2_1_front.png", 0);
            update_json(f_to_s_sprites, orct2_sprites, "sprite_13.png", "flat_to_steep_up_2_1_front_rails.png", 0);
            update_json(f_to_s_sprites, orct2_sprites, "sprite_9.png", "flat_to_steep_up_2_2.png", 0);
            update_json(f_to_s_sprites, orct2_sprites, "sprite_9.png", "flat_to_steep_up_2_2_rails.png", 0);
            update_json(f_to_s_sprites, orct2_sprites, "sprite_9.png", "flat_to_steep_up_2_2_front.png", 0);
            update_json(f_to_s_sprites, orct2_sprites, "sprite_9.png", "flat_to_steep_up_2_2_front_rails.png", 0);
            update_json(f_to_s_sprites, orct2_sprites, "sprite_5.png", "flat_to_steep_up_2_3.png", 16);
            update_json(f_to_s_sprites, orct2_sprites, "sprite_5.png", "flat_to_steep_up_2_3_rails.png", 16);
            update_json(f_to_s_sprites, orct2_sprites, "sprite_5.png", "flat_to_steep_up_2_3_front.png", 16);
            update_json(f_to_s_sprites, orct2_sprites, "sprite_5.png", "flat_to_steep_up_2_3_front_rails.png", 16);
            update_json(f_to_s_sprites, orct2_sprites, "sprite_1.png", "flat_to_steep_up_2_4.png", 40);
            update_json(f_to_s_sprites, orct2_sprites, "sprite_1.png", "flat_to_steep_up_2_4_rails.png", 40);
            update_json(f_to_s_sprites, orct2_sprites, "sprite_1.png", "flat_to_steep_up_2_4_front.png", 40);
            update_json(f_to_s_sprites, orct2_sprites, "sprite_1.png", "flat_to_steep_up_2_4_front_rails.png", 40);

            update_json(f_to_s_sprites, orct2_sprites, "sprite_14.png", "flat_to_steep_up_3_1.png", 0);
            update_json(f_to_s_sprites, orct2_sprites, "sprite_14.png", "flat_to_steep_up_3_1_rails.png", 0);
            update_json(f_to_s_sprites, orct2_sprites, "sprite_14.png", "flat_to_steep_up_3_1_front.png", 0);
            update_json(f_to_s_sprites, orct2_sprites, "sprite_14.png", "flat_to_steep_up_3_1_front_rails.png", 0);
            update_json(f_to_s_sprites, orct2_sprites, "sprite_10.png", "flat_to_steep_up_3_2.png", 0);
            update_json(f_to_s_sprites, orct2_sprites, "sprite_10.png", "flat_to_steep_up_3_2_rails.png", 0);
            update_json(f_to_s_sprites, orct2_sprites, "sprite_10.png", "flat_to_steep_up_3_2_front.png", 0);
            update_json(f_to_s_sprites, orct2_sprites, "sprite_10.png", "flat_to_steep_up_3_2_front_rails.png", 0);
            update_json(f_to_s_sprites, orct2_sprites, "sprite_6.png", "flat_to_steep_up_3_3.png", 15);
            update_json(f_to_s_sprites, orct2_sprites, "sprite_6.png", "flat_to_steep_up_3_3_rails.png", 15);
            update_json(f_to_s_sprites, orct2_sprites, "sprite_6.png", "flat_to_steep_up_3_3_front.png", 15);
            update_json(f_to_s_sprites, orct2_sprites, "sprite_6.png", "flat_to_steep_up_3_3_front_rails.png", 15);
            update_json(f_to_s_sprites, orct2_sprites, "sprite_2.png", "flat_to_steep_up_3_4.png", 40);
            update_json(f_to_s_sprites, orct2_sprites, "sprite_2.png", "flat_to_steep_up_3_4_rails.png", 40);
            update_json(f_to_s_sprites, orct2_sprites, "sprite_2.png", "flat_to_steep_up_3_4_front.png", 40);
            update_json(f_to_s_sprites, orct2_sprites, "sprite_2.png", "flat_to_steep_up_3_4_front_rails.png", 40);

            update_json(f_to_s_sprites, orct2_sprites, "sprite_15.png", "flat_to_steep_up_4_1.png", 0);
            update_json(f_to_s_sprites, orct2_sprites, "sprite_15.png", "flat_to_steep_up_4_1_rails.png", 0);
            update_json(f_to_s_sprites, orct2_sprites, "sprite_11.png", "flat_to_steep_up_4_2.png", 0);
            update_json(f_to_s_sprites, orct2_sprites, "sprite_11.png", "flat_to_steep_up_4_2_rails.png", 0);
            update_json(f_to_s_sprites, orct2_sprites, "sprite_7.png", "flat_to_steep_up_4_3.png", 16);
            update_json(f_to_s_sprites, orct2_sprites, "sprite_7.png", "flat_to_steep_up_4_3_rails.png", 16);
            update_json(f_to_s_sprites, orct2_sprites, "sprite_3.png", "flat_to_steep_up_4_4.png", 40);
            update_json(f_to_s_sprites, orct2_sprites, "sprite_3.png", "flat_to_steep_up_4_4_rails.png", 40);

            update_json(s_to_f_sprites, orct2_sprites, "sprite_12.png", "steep_to_flat_up_1_1.png", 8);
            update_json(s_to_f_sprites, orct2_sprites, "sprite_12.png", "steep_to_flat_up_1_1_rails.png", 8);
            update_json(s_to_f_sprites, orct2_sprites, "sprite_8.png", "steep_to_flat_up_1_2.png", 48);
            update_json(s_to_f_sprites, orct2_sprites, "sprite_8.png", "steep_to_flat_up_1_2_rails.png", 48);
            update_json(s_to_f_sprites, orct2_sprites, "sprite_4.png", "steep_to_flat_up_1_3.png", 72);
            update_json(s_to_f_sprites, orct2_sprites, "sprite_4.png", "steep_to_flat_up_1_3_rails.png", 72);
            update_json(s_to_f_sprites, orct2_sprites, "sprite_0.png", "steep_to_flat_up_1_4.png", 88);
            update_json(s_to_f_sprites, orct2_sprites, "sprite_0.png", "steep_to_flat_up_1_4_rails.png", 88);

            update_json(s_to_f_sprites, orct2_sprites, "sprite_13.png", "steep_to_flat_up_2_1.png", 8);
            update_json(s_to_f_sprites, orct2_sprites, "sprite_13.png", "steep_to_flat_up_2_1_rails.png", 8);
            update_json(s_to_f_sprites, orct2_sprites, "sprite_13.png", "steep_to_flat_up_2_1_front.png", 8);
            update_json(s_to_f_sprites, orct2_sprites, "sprite_13.png", "steep_to_flat_up_2_1_front_rails.png", 8);
            update_json(s_to_f_sprites, orct2_sprites, "sprite_9.png", "steep_to_flat_up_2_2.png", 48);
            update_json(s_to_f_sprites, orct2_sprites, "sprite_9.png", "steep_to_flat_up_2_2_rails.png", 48);
            update_json(s_to_f_sprites, orct2_sprites, "sprite_9.png", "steep_to_flat_up_2_2_front.png", 48);
            update_json(s_to_f_sprites, orct2_sprites, "sprite_9.png", "steep_to_flat_up_2_2_front_rails.png", 48);
            update_json(s_to_f_sprites, orct2_sprites, "sprite_5.png", "steep_to_flat_up_2_3.png", 72);
            update_json(s_to_f_sprites, orct2_sprites, "sprite_5.png", "steep_to_flat_up_2_3_rails.png", 72);
            update_json(s_to_f_sprites, orct2_sprites, "sprite_5.png", "steep_to_flat_up_2_3_front.png", 72);
            update_json(s_to_f_sprites, orct2_sprites, "sprite_5.png", "steep_to_flat_up_2_3_front_rails.png", 72);
            update_json(s_to_f_sprites, orct2_sprites, "sprite_1.png", "steep_to_flat_up_2_4.png", 88);
            update_json(s_to_f_sprites, orct2_sprites, "sprite_1.png", "steep_to_flat_up_2_4_rails.png", 88);
            update_json(s_to_f_sprites, orct2_sprites, "sprite_1.png", "steep_to_flat_up_2_4_front.png", 88);
            update_json(s_to_f_sprites, orct2_sprites, "sprite_1.png", "steep_to_flat_up_2_4_front_rails.png", 88);

            update_json(s_to_f_sprites, orct2_sprites, "sprite_14.png", "steep_to_flat_up_3_1.png", 8);
            update_json(s_to_f_sprites, orct2_sprites, "sprite_14.png", "steep_to_flat_up_3_1_rails.png", 8);
            update_json(s_to_f_sprites, orct2_sprites, "sprite_14.png", "steep_to_flat_up_3_1_front.png", 8);
            update_json(s_to_f_sprites, orct2_sprites, "sprite_14.png", "steep_to_flat_up_3_1_front_rails.png", 8);
            update_json(s_to_f_sprites, orct2_sprites, "sprite_10.png", "steep_to_flat_up_3_2.png", 48);
            update_json(s_to_f_sprites, orct2_sprites, "sprite_10.png", "steep_to_flat_up_3_2_rails.png", 48);
            update_json(s_to_f_sprites, orct2_sprites, "sprite_10.png", "steep_to_flat_up_3_2_front.png", 48);
            update_json(s_to_f_sprites, orct2_sprites, "sprite_10.png", "steep_to_flat_up_3_2_front_rails.png", 48);
            update_json(s_to_f_sprites, orct2_sprites, "sprite_6.png", "steep_to_flat_up_3_3.png", 72);
            update_json(s_to_f_sprites, orct2_sprites, "sprite_6.png", "steep_to_flat_up_3_3_rails.png", 72);
            update_json(s_to_f_sprites, orct2_sprites, "sprite_6.png", "steep_to_flat_up_3_3_front.png", 72);
            update_json(s_to_f_sprites, orct2_sprites, "sprite_6.png", "steep_to_flat_up_3_3_front_rails.png", 72);
            update_json(s_to_f_sprites, orct2_sprites, "sprite_2.png", "steep_to_flat_up_3_4.png", 88);
            update_json(s_to_f_sprites, orct2_sprites, "sprite_2.png", "steep_to_flat_up_3_4_rails.png", 88);
            update_json(s_to_f_sprites, orct2_sprites, "sprite_2.png", "steep_to_flat_up_3_4_front.png", 88);
            update_json(s_to_f_sprites, orct2_sprites, "sprite_2.png", "steep_to_flat_up_3_4_front_rails.png", 88);

            update_json(s_to_f_sprites, orct2_sprites, "sprite_15.png", "steep_to_flat_up_4_1.png", 8);
            update_json(s_to_f_sprites, orct2_sprites, "sprite_15.png", "steep_to_flat_up_4_1_rails.png", 8);
            update_json(s_to_f_sprites, orct2_sprites, "sprite_11.png", "steep_to_flat_up_4_2.png", 48);
            update_json(s_to_f_sprites, orct2_sprites, "sprite_11.png", "steep_to_flat_up_4_2_rails.png", 48);
            update_json(s_to_f_sprites, orct2_sprites, "sprite_7.png", "steep_to_flat_up_4_3.png", 72);
            update_json(s_to_f_sprites, orct2_sprites, "sprite_7.png", "steep_to_flat_up_4_3_rails.png", 72);
            update_json(s_to_f_sprites, orct2_sprites, "sprite_3.png", "steep_to_flat_up_4_4.png", 88);
            update_json(s_to_f_sprites, orct2_sprites, "sprite_3.png", "steep_to_flat_up_4_4_rails.png", 88);

            let mut buf = Vec::new();
            let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
            let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
            use serde::Serialize;
            openrct2_sprites.serialize(&mut ser).unwrap();
            buf.push(b'\n');
            std::fs::write(openrct2_sprites_path, buf).unwrap();
        }
        CliCommands::Crop { openrct2_path } => {
            let sprite_dir = openrct2_path.join("resources").join("g2");

            let openrct2_sprites_path = sprite_dir.join("sprites.json");
            let openrct2_sprites_string = std::fs::read_to_string(&openrct2_sprites_path).unwrap();
            let mut openrct2_sprites: serde_json::Value = serde_json::from_str(&openrct2_sprites_string).unwrap();
            let orct2_sprites = openrct2_sprites.as_array_mut().unwrap();

            crop_sprite(orct2_sprites, "track/wooden/flat_to_steep_up_1_1_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/flat_to_steep_up_1_2_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/flat_to_steep_up_1_3_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/flat_to_steep_up_1_4_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/flat_to_steep_up_2_1_front.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/flat_to_steep_up_2_1_front_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/flat_to_steep_up_2_1_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/flat_to_steep_up_2_2_front.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/flat_to_steep_up_2_2_front_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/flat_to_steep_up_2_2_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/flat_to_steep_up_2_3_front.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/flat_to_steep_up_2_3_front_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/flat_to_steep_up_2_3_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/flat_to_steep_up_2_4_front.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/flat_to_steep_up_2_4_front_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/flat_to_steep_up_2_4_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/flat_to_steep_up_3_1_front.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/flat_to_steep_up_3_1_front_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/flat_to_steep_up_3_1_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/flat_to_steep_up_3_2_front.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/flat_to_steep_up_3_2_front_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/flat_to_steep_up_3_2_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/flat_to_steep_up_3_3_front.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/flat_to_steep_up_3_3_front_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/flat_to_steep_up_3_3_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/flat_to_steep_up_3_4_front.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/flat_to_steep_up_3_4_front_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/flat_to_steep_up_3_4_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/flat_to_steep_up_4_1_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/flat_to_steep_up_4_2_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/flat_to_steep_up_4_3_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/flat_to_steep_up_4_4_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/steep_to_flat_up_1_1_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/steep_to_flat_up_1_2_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/steep_to_flat_up_1_3_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/steep_to_flat_up_1_4_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/steep_to_flat_up_2_1_front.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/steep_to_flat_up_2_1_front_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/steep_to_flat_up_2_1_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/steep_to_flat_up_2_2_front.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/steep_to_flat_up_2_2_front_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/steep_to_flat_up_2_2_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/steep_to_flat_up_2_3_front.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/steep_to_flat_up_2_3_front_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/steep_to_flat_up_2_3_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/steep_to_flat_up_2_4_front.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/steep_to_flat_up_2_4_front_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/steep_to_flat_up_2_4_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/steep_to_flat_up_3_1_front.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/steep_to_flat_up_3_1_front_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/steep_to_flat_up_3_1_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/steep_to_flat_up_3_2_front.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/steep_to_flat_up_3_2_front_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/steep_to_flat_up_3_2_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/steep_to_flat_up_3_3_front.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/steep_to_flat_up_3_3_front_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/steep_to_flat_up_3_3_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/steep_to_flat_up_3_4_front.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/steep_to_flat_up_3_4_front_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/steep_to_flat_up_3_4_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/steep_to_flat_up_4_1_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/steep_to_flat_up_4_2_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/steep_to_flat_up_4_3_rails.png", &sprite_dir);
            crop_sprite(orct2_sprites, "track/wooden/steep_to_flat_up_4_4_rails.png", &sprite_dir);

            let mut buf = Vec::new();
            let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
            let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
            use serde::Serialize;
            openrct2_sprites.serialize(&mut ser).unwrap();
            buf.push(b'\n');
            std::fs::write(openrct2_sprites_path, buf).unwrap();
        }
    }
}
