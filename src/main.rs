use clap::Parser;
use std::fs::File;
use std::io::prelude::*;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    lang: String,

    #[arg(short, long)]
    path: String,
}

fn main() {
    let args = Args::parse();
    let mut project_name: String = String::new();
    std::io::stdin()
        .read_line(&mut project_name)
        .expect("Error while reading project name!");
    project_name = project_name.trim().to_string();
    let project_path = format!("{}/{}", args.path, project_name);
    match args.lang.as_str() {
        "cpp" => {
            std::fs::create_dir(&project_path).expect("Error while creating project folder!");
            let mut main_cpp_file = File::create(format!("{}/main.cpp", project_path))
                .expect("Error while creating project default files!");
            main_cpp_file
                .write_all(
                    b"#include <iostream>

int main(){
    std::cout << \"Hello world\" << std::endl;
    return 0;
}",
                )
                .expect("Error while writing to project default file!");
            let mut cmake_file = File::create(format!("{}/CMakeLists.txt", project_path))
                .expect("Error while creating project default files!");
            cmake_file
                .write_all(
                    format!(
                        "cmake_minimum_required(VERSION 3.13)  # CMake version check
project({0})                          # Create project \"{0}\"
set(CMAKE_CXX_STANDARD 20)            # Enable c++20 standard

# Add main.cpp file of project root directory as source file
set(SOURCE_FILES main.cpp)

# Add executable target with source files listed in SOURCE_FILES variable
add_executable({0} ${{SOURCE_FILES}})
                    ",
                        project_name
                    )
                    .as_bytes(),
                )
                .expect("Error while writing to project file!");
            println!(
                "Cmake project {} was created with path {}",
                project_name, project_path
            );
        }
        _ => {
            println!("No such language!");
        }
    }
}
