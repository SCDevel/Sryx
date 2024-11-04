use std::path::PathBuf;

fn main() -> miette::Result<()> {
    let include_paths = get_includes();
    let mut b = autocxx_build::Builder::new("src/lib.rs", &include_paths)
        .extra_clang_args(&["-std=c++17"])
        .build()?;
    b.flag_if_supported("-std=c++17")
        .compile("cgal-cxx");
    println!("cargo:rerun-if-changed=src/lib.rs");
    Ok(())
}

fn get_includes() -> Vec<PathBuf> {
    // BOOST
    let boost_libs = vec![
        "config", "predef", "iterator",
        "static_assert", "core", "mpl",
        "preprocessor", "type_traits", "detail",
        "utility", "functional", "container_hash",
        "describe", "mp11", "any",
        "type_index", "throw_exception", "source_location",
        "assert", "math", "tuple",
        "random", "integer", "array",
        "range", "io", "concept_check",
        "algorithm", "optional", "container",
        "move", "intrusive",
    ];
    let mut boost_lib_paths: Vec<PathBuf> = boost_libs.iter().map(|p | {
        std::path::PathBuf::from(format!("includes/boost/libs/{}/include", p))
    }).collect();

    // CGAL
    let cgal_path = std::path::PathBuf::from("includes/CGAL/include");

    // FINISH
    let mut include_paths = Vec::new();

    include_paths.append(&mut boost_lib_paths);
    include_paths.push(cgal_path);

    include_paths
}