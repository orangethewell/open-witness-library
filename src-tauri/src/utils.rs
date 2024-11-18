use std::fs;
use std::io;
use std::path::PathBuf;

use colored::Colorize;
use zip::ZipArchive;

const TARGET: &'static str = "utils";

pub fn unpack_zip<R: std::io::Seek + std::io::Read>(
    mut archive: ZipArchive<R>,
    destination: &PathBuf,
) {
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => destination.join(path),
            None => continue,
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }

        if (*file.name()).ends_with('/') {
            debug!(
                target: TARGET,
                "File {} extracted to \"{}\"",
                i.to_string().cyan(),
                outpath.display().to_string().bright_yellow(),
            );
            fs::create_dir_all(&outpath).unwrap();
        } else {
            debug!(
                target: TARGET,
                "File {} extracted to \"{}\" ({})",
                i.to_string().cyan(),
                outpath.display().to_string().bright_yellow(),
                format!("{} bytes", file.size()).yellow()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
}
