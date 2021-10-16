use std::*;
use std::f32;
use std::{fs, io};
use std::fs::File;
use std::path::Path;
use serde::{Deserialize, Serialize};
use serde_json::{Result, to_writer};
use glob::glob;
use itertools::MultiProduct; 
extern crate image;    
use image::{GenericImageView, ImageBuffer, RgbaImage, imageops, Rgba, Pixel};


#[derive(Serialize, Deserialize)]
pub struct Layer {
    pub name: String,
    pub folder: String,
    pub weight: u8,
}

#[derive(Serialize, Deserialize)]
pub struct ExamplePlacement {
    pub name: String,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub avatar: Option<u32>,
}

#[derive(Serialize, Deserialize)]
pub struct ImageUsageExample {
    pub name: String,
    pub file: String,
    pub placements: Vec<ExamplePlacement>,
}

#[derive(Serialize, Deserialize)]
pub struct NFTAttribute {
   pub trait_type: String,
   pub value: String,
}

#[derive(Serialize, Deserialize)]
pub struct NFTMetadata {
   pub description: String,
   pub external_url: String,
   pub image: String,
   pub name: String,
   pub attributes: Vec<NFTAttribute>,
}


pub fn read_collection_schema(filename: &str) -> Result<Vec<Layer>> {
   let contents = fs::read_to_string(filename)
       .expect("Something went wrong reading the file {}");
   let layers: Vec<Layer> = serde_json::from_str(&contents)?;
   Ok(layers)
}


pub fn get_layers_images(layers: &Vec<Layer>) -> io::Result<Vec<Vec<std::path::PathBuf>>> {
   let mut images: Vec<Vec<std::path::PathBuf>> = Vec::new();
   let mut layer_index = 0;
   if !Path::new("layers").exists() {
      panic!("You need to create `layers` directory first.");
   }
   for layer in layers {
      images.push(Vec::new());
      for extension in vec!("png", "jpg", "PNG", "JPG") {
         let glob_pattern = format!("layers/{}/*.{}", layer.folder, extension);
         for entry in glob(&glob_pattern).expect("Failed to read glob pattern") {
            images[layer_index].push(entry.unwrap());
         }
      }
      layer_index += 1;
   }

   Ok(images)
}


pub fn get_trait_name_from_image_path(
   image_path: &std::path::PathBuf) -> Result<String> {
   return Ok(image_path.file_stem().unwrap().to_str().unwrap().to_string());
}


pub fn generate_and_merge_images(
   layers: &Vec<Layer>,
   all_images_in_layers_iterator: MultiProduct<std::slice::Iter<std::path::PathBuf>>) -> Result<()> {
   let mut i_image = 0;
   if !Path::new("images").exists() {
      fs::create_dir_all("images").expect("Failed to create images directory.");
   }

   let mut NFTMetadata = NFTMetadata {
      description: "Collection generated with NFT image generator".to_string(),
      external_url: "https://partyparrotclub.com".to_string(),
      image: "".to_string(),
      name: "".to_string(),
      attributes: Vec::new(),
   };

   for image_to_generate in all_images_in_layers_iterator {
         if image_to_generate.len() < 2 {
            continue;
         }
         let mut composed_image = image::open(image_to_generate[0]).expect("Opening image failed");
         let mut nft_attributes: Vec<NFTAttribute> = Vec::new();
         for i_layer in 1..image_to_generate.len() {
            // println!("{:?}", image_to_generate[i_layer]);
            println!("{:?} {:?}",
            layers[i_layer].name,
            get_trait_name_from_image_path(image_to_generate[i_layer]));

            let im2 = image::open(image_to_generate[i_layer]).expect("Opening image failed");
            image::imageops::overlay(&mut composed_image, &im2, 0, 0);

            let trait_value = NFTAttribute{
               trait_type: layers[i_layer].name.to_string(),
               value: get_trait_name_from_image_path(
                  image_to_generate[i_layer]).unwrap(),
            };
            nft_attributes.push(trait_value);
         }
         let file_name = format!("images/{}.png", i_image);
         composed_image.save(&Path::new(&file_name)).expect("Failed to save image.");

         NFTMetadata.attributes = nft_attributes;
         to_writer(&File::create(format!("images/{}.json", i_image)).unwrap(), &NFTMetadata)?;
         i_image += 1;
   }

   Ok(())
}


pub fn read_usage_generation_schema(filename: &str) -> Result<Vec<ImageUsageExample>> {
   let contents = fs::read_to_string(filename)
       .expect("Something went wrong reading the usage_generation_schema file ");
   let usage_examples: Vec<ImageUsageExample> = serde_json::from_str(&contents)?;
   Ok(usage_examples)
}



pub fn is_in_circle(x: u32, y: u32, cx: u32, cy: u32, r: u32) -> bool {
   let dx: u32 = if x > cx {x - cx} else {cx - x};
   let dy: u32 = if y > cy {y - cy} else {cy - y};
   return (((dx).pow(2) + (dy).pow(2)) as f32).sqrt() < (r as f32);
}


pub fn crop_circles(image: &mut image::ImageBuffer<Rgba<u8>, std::vec::Vec<u8>>) -> Result<()> {
   let (width, height) = image.dimensions();
   let cx = width / 2;
   let cy = height / 2;
   let r = cx;

   for (x, y, pixel) in image.enumerate_pixels_mut() {
      if !is_in_circle(x, y, cx, cy, r) {
         *pixel = image::Rgba([0, 0, 0, 0]);
      }
   }

   Ok(())
}


pub fn generate_usage_examples(usage_examples: Vec<ImageUsageExample>) -> Result<()> {
   let mut i_image = 0;

   if !Path::new("images/examples").exists() {
      fs::create_dir_all("images/examples").expect("Failed to create examples directory.");
   }

   for usage_example in usage_examples {
      let usage_example_filename = usage_example.file;
      let mut template_file = image::open(
         format!("templates/{}.png", usage_example_filename)).expect(
         "Opening template file failed");

      for placement in usage_example.placements {
         if placement.avatar.is_some() {
            i_image = placement.avatar.unwrap();
         }
         let file_name = format!("images/{}.png", i_image);
         let pfp_image = image::open(file_name).expect("Opening sample pfp failed. Make sure the file exists.");
         let mut resized_pfp_image = image::imageops::resize(
            &pfp_image, placement.width, placement.width, image::imageops::Gaussian);
         crop_circles(&mut resized_pfp_image).expect("Error cropping to circle");
         image::imageops::overlay(&mut template_file, &resized_pfp_image, placement.x, placement.y);
      }
      template_file.save(
         format!("images/examples/{}.png", usage_example_filename)).expect("Failed to save image.");
   }

   Ok(())
}
