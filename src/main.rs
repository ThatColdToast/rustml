#![allow(non_snake_case)]

mod Neural;
use Neural::Network;

use std::fs;
use std::io::Read;

fn getFileData(filename: &String) -> Vec<u8> {
    let mut f = fs::File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    return buffer
}

// fn as_u32_be(array: &[u8; 4]) -> i32 {
fn as_u32_be(array: &[u8; 4]) -> i32 {
    println!("Read: {} {} {} {}", array[0], array[1], array[2], array[3]);

    ((array[0] as i32) << 24) +
    ((array[1] as i32) << 16) +
    ((array[2] as i32) <<  8) +
    ((array[3] as i32) <<  0)
}

fn as_u32_le(array: &[u8; 4]) -> i32 {
    ((array[0] as i32) <<  0) +
    ((array[1] as i32) <<  8) +
    ((array[2] as i32) << 16) +
    ((array[3] as i32) << 24)
}

fn getTrainingData() -> (Vec<u8>, i32, i32, i32, Vec<u8>) {
    println!("Reading Data");
    let mut imageData = getFileData(&String::from("src/train-images.idx3-ubyte"));
    let magicNumberImage = as_u32_be(&[imageData[0], imageData[1], imageData[2], imageData[3]]);
    let imageCount = as_u32_be(&[imageData[4], imageData[5], imageData[6], imageData[7]]);
    let rowCount = as_u32_be(&[imageData[8], imageData[9], imageData[10], imageData[11]]);
    let columnCount = as_u32_be(&[imageData[12], imageData[13], imageData[14], imageData[15]]);
    let images: Vec<u8> = imageData.drain(16..).collect();

    // println!("mN:{} iC:{} rC:{} cC:{}", magicNumberImage, imageCount, rowCount, columnCount);

    let mut labelData = getFileData(&String::from("src/train-labels.idx1-ubyte"));
    let magicNumberLabel = as_u32_be(&[labelData[0], labelData[1], labelData[2], labelData[3]]);
    let labelCount = as_u32_be(&[labelData[4], labelData[5], labelData[6], labelData[7]]);
    let labels: Vec<u8> = labelData.drain(8..).collect();

    // println!("mN:{} lC:{}", magicNumberLabel, labelCount);

    if imageCount != labelCount {
        panic!("file counts do not match");
    }

    return (images, imageCount, rowCount, columnCount, labels);
}

fn main() {
    println!("Starting...");
    // let (images, imageCount, rowCount, columnCount, labels) = getTrainingData();

    let mut network = Network::new();
    network.Init(10, 5, 4);
    network.AddLayer(10);
    network.AddLayer(3);
    network.AddLayer(4);

    // println!(" - Network Created -");
    
    // println!(" - Network Trained -");

    // network.propagate();
    // println!(" - Network Prediction: -");
    // network.print();

    // network.BackProp();

    // let zero = vec![0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0];
    // println!(" - Network Cost: {} -", network.cost(&zero));

    // network.print();
}
