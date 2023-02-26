use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

// #[cfg(debug_assertions)]
// fn maybedebug() {
//     use std::time::Duration;
//     println!("cargo:warning=maybedebug() should only be included if im debugging app");
//     let url = format!(
//         "vscode://vadimcn.vscode-lldb/launch/config?{{'request':'attach','pid':{}}}",
//         std::process::id()

//     );
//     std::process::Command::new("/mnt/c/Program Files/Microsoft VS Code Insiders/bin/code-insiders")
//         .arg("--open-url")
//         .arg(url)
//         .output()
//         .expect("couldnt spawn code");
//     std::thread::sleep(Duration::from_secs(10));
// maybedebug();
// }

fn main() {
    println!("Hello from build.rs");

    println!(
        "CARGO_MANIFEST_DIR is {:?}",
        env::var("CARGO_MANIFEST_DIR").unwrap()
    );

    println!("PROFILE is {:?}", env::var("PROFILE").unwrap());
    compilewindowicons();
    copyassets();
}

fn copyassets() {
    let output_path = get_output_path();
    println!("Calculated build path: {}", output_path.to_str().unwrap());

    let out_dir = env::var("OUT_DIR").unwrap();

    println!("Cargo out dir: {out_dir}");

    let input_path = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("gamedata/");
    let output_path = Path::new(&output_path).join("gamedata/");
    copy(input_path, output_path).expect("couldnt copy files, maybe the source doesnt exist? {}");
}

fn compilewindowicons() {
    let target = env::var("TARGET").unwrap();
    if target.contains("windows") {
        println!("embedding icon.rc ");
        embed_resource::compile("data/assets/ico/windows/icon.rc");
    }
}

fn get_output_path() -> PathBuf {
    let target = env::var("TARGET").unwrap();
    println!("target is: {target}");

    //<root or manifest path>/target/<profile>/
    let currentworkingdirectory = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = env::var("PROFILE").unwrap();
    let path = Path::new(&currentworkingdirectory)
        .join("target")
        .join(target)
        .join(build_type);
    path
}

/// # Errors
///
/// Will return `Err` if `path` does not exist or the user does not have
/// permission to read/write.
pub fn copy<U: AsRef<Path>, V: AsRef<Path>>(from: U, to: V) -> Result<(), std::io::Error> {
    let mut stack = vec![PathBuf::from(from.as_ref())];

    let output_root = PathBuf::from(to.as_ref());
    let input_root = PathBuf::from(from.as_ref()).components().count();

    while let Some(working_path) = stack.pop() {
        println!("process: {:?}", &working_path);

        // Generate a relative path
        let src: PathBuf = working_path.components().skip(input_root).collect();

        // Create a destination if missing
        let dest = if src.components().count() == 0 {
            output_root.clone()
        } else {
            output_root.join(&src)
        };
        if fs::metadata(&dest).is_err() {
            println!(" mkdir: {dest:?}");
            fs::create_dir_all(&dest)?;
        }

        for entry in fs::read_dir(working_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                match path.file_name() {
                    Some(filename) => {
                        let dest_path = dest.join(filename);
                        println!("  copy: {:?} -> {:?}", &path, &dest_path);
                        fs::copy(&path, &dest_path)?;
                    }
                    None => {
                        println!("failed: {path:?}");
                    }
                }
            }
        }
    }

    Ok(())
}
