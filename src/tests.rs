use crate::build;

use std::{
    fs,
    path::{Path, PathBuf},
};

#[test]
fn integration_exr_files() {
    let test_dir = Path::new("tests");

    for entry in fs::read_dir(test_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.extension().and_then(|e| e.to_str()) != Some("exr") {
            continue;
        }

        let filename = path.file_name().unwrap().to_string_lossy();
        let stem = path.file_stem().unwrap().to_string_lossy();

        let contents = fs::read_to_string(&path).unwrap();

        let out_path: PathBuf = format!("tests/output/{stem}.out").into();
        let expected_path: PathBuf = format!("tests/expected/{stem}.out").into();

        let mut out_writer = fs::File::create(&out_path).unwrap();

        build(&filename, &contents, &mut out_writer, None, None, None).unwrap();

        let expected = fs::read_to_string(&expected_path).unwrap();
        let actual = fs::read_to_string(&out_path).unwrap();

        assert_eq!(actual, expected, "Mismatch in test {filename}");
    }
}
