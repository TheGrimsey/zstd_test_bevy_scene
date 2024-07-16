use std::{io::Write, path::PathBuf};

fn main() {
    let level = 18;
    let files = std::fs::read_dir("./scenes").unwrap();

    let dict = zstd::dict::from_files(files.map(|entry| entry.unwrap().path()), 1024 * 64).unwrap();

    let mut file = std::fs::File::create(PathBuf::from("scene_dict.dict")).unwrap();

    let _ = file.write_all(&dict);

    let _ = file.flush();

    let mut dict_compressor = zstd::bulk::Compressor::with_dictionary(level, &dict).unwrap();
    let mut compressor = zstd::bulk::Compressor::new(level).unwrap();

    let mut sizes = Vec::default();

    for scene in std::fs::read_dir("./scenes").unwrap().map(|entry| entry.unwrap().path()) {
        let input_scene = std::fs::read(scene.clone()).unwrap();
        
        let data_compressed = compressor.compress(&input_scene).unwrap();
        let data_compressed_dict = dict_compressor.compress(&input_scene).unwrap();

        let compressed_rate = data_compressed.len() as f32 / input_scene.len() as f32;
        let dict_compressed_rate = data_compressed_dict.len() as f32 / input_scene.len() as f32;

        println!(
            "{}: Raw: {}, Compressed: {} ({}%), Dict Compression: {} ({}%)",
            scene.to_str().unwrap(),
            input_scene.len(),
            data_compressed.len(),
            compressed_rate * 100.0,
            data_compressed_dict.len(),
            dict_compressed_rate * 100.0
        );

        sizes.push((
            compressed_rate,
            dict_compressed_rate
        ));
    }

    let (sum, sum_dict) = sizes.iter().fold((0.0, 0.0), |(acc_compressed, acc_dict), (compressed, dict)| {
        (acc_compressed + *compressed, acc_dict + *dict)
    });

    let average_ratio = (sum / sizes.len() as f32) * 100.0;
    let average_ratio_dict = (sum_dict / sizes.len() as f32) * 100.0;

    println!("Average Ratio: {average_ratio}%, Average Ratio (with Dict): {average_ratio_dict}%");
}