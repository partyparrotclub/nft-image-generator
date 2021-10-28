use clap::{App, Arg};
use itertools::Itertools;
use std::fs;
mod images_generator;

fn main() {
    let matches = App::new("NFT Image Generator")
        .version("0.1")
        .author("by Party Parrot Club")
        .about("Helps creators to generate a collection.")
        .arg(
            Arg::new("v")
                .short('v')
                .multiple_occurrences(true)
                .takes_value(true)
                .about("Sets the level of verbosity"),
        )
        .subcommand(
            App::new("build")
                .about("builds the collection")
                .version("0.1")
                .arg(
                    Arg::new("debug")
                        .short('d')
                        .about("print debug information verbosely"),
                ),
        )
        .subcommand(
            App::new("build_examples")
                .about("builds only the usage examples")
                .version("0.1")
                .arg(
                    Arg::new("debug")
                        .short('d')
                        .about("print debug information verbosely"),
                ),
        )
        .get_matches();

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match matches.occurrences_of("v") {
        0 => println!("Verbose mode is off"),
        1 => println!("Verbose mode is kind of on"),
        2 => println!("Verbose mode is on"),
        _ => println!("Don't be crazy"),
    }

    if let Some(ref matches) = matches.subcommand_matches("build_examples") {
        // "$ myapp test" was run
        if matches.is_present("debug") {
            // "$ myapp test -d" was run
            println!("Printing debug info...");
        } else {
            let usage_examples_schema =
                images_generator::read_usage_generation_schema("usage_generation_schema.json")
                    .unwrap();
            images_generator::generate_usage_examples(usage_examples_schema)
                .expect("Failed to generate sample images.");
        }
    }

    if matches.is_present("debug") {
        // "$ myapp test -d" was run
        println!("Printing debug info...");
    } else {
        let filename = "collection_schema.json";
        println!("In file {}", filename);
        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

        println!("With Collection Schema:\n{}", contents);

        let mut collection_schema = images_generator::read_collection_schema(filename).unwrap();
        // println!("Read from file:\n{}", collection_schema[0].name);

        collection_schema.sort_by(|a, b| a.weight.cmp(&b.weight));
        // println!("Sorted collection_schema:\n{}", collection_schema[0].name);
        let layers_content = images_generator::get_layers_images(&collection_schema).unwrap();
        println!("Layers:\n{:?}", layers_content);

        assert!(layers_content.len() > 2);
        let all_images_in_layers_iterator = layers_content
            .iter()
            .map(|x| x.iter())
            .multi_cartesian_product();
        println!("Images in layers are generated. Starting composing\n");
        images_generator::generate_and_merge_images(
            &collection_schema,
            all_images_in_layers_iterator,
        )
        .expect("Composition failed.");

        let usage_examples_schema =
            images_generator::read_usage_generation_schema("usage_generation_schema.json").unwrap();
        images_generator::generate_usage_examples(usage_examples_schema)
            .expect("Failed to generate sample images.");
    }
}
