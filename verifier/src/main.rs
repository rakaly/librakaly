use argh::FromArgs;
use libc::{c_char, c_int, size_t};
use std::fs;
use std::io::{Write, Read, Cursor};
use std::path::Path;

fn default_game() -> String {
    // Because eu4 is the best
    String::from("eu4")
}

#[derive(FromArgs)]
/// Melt PDS game data into plaintext.
struct Arguments {
    /// game mode to support
    #[argh(option, default = "default_game()")]
    game: String,
}

enum MeltedBuffer {}

#[cfg_attr(target_os = "windows", link(name = "rakaly.dll"))]
#[cfg_attr(target_os = "linux", link(name = "rakaly"))]
extern "C" {
    fn rakaly_melt_error_code(res: *const MeltedBuffer) -> c_int;
    fn rakaly_free_melt(res: *mut MeltedBuffer);
    fn rakaly_melt_data_length(res: *const MeltedBuffer) -> size_t;
    fn rakaly_melt_write_data(res: *const MeltedBuffer, buffer: *mut c_char, length: size_t);
    fn rakaly_eu4_melt(data_ptr: *const c_char, data_len: size_t) -> *mut MeltedBuffer;
    fn rakaly_ck3_melt(data_ptr: *const c_char, data_len: size_t) -> *mut MeltedBuffer;
    fn rakaly_imperator_melt(data_ptr: *const c_char, data_len: size_t) -> *mut MeltedBuffer;
    fn rakaly_hoi4_melt(data_ptr: *const c_char, data_len: size_t) -> *mut MeltedBuffer;
}

fn main() {
    let args: Arguments = argh::from_env();
    match args.game.to_lowercase().as_str() {
        "eu4" => {
            let data = request("eu4saves-test-cases", "kandy2.bin.eu4");
            unsafe {
                let melted = rakaly_eu4_melt(data.as_ptr() as *const i8, data.len());
                if rakaly_melt_error_code(melted) != 0 {
                    rakaly_free_melt(melted);
                    panic!("eu4 melt failed");
                }

                let out_len = rakaly_melt_data_length(melted);
                let mut out: Vec<u8> = vec![0; out_len];
                let _wrote_len =
                    rakaly_melt_write_data(melted, out.as_mut_ptr() as *mut i8, out.len());
                let _ = std::io::stdout().write_all(out.as_slice());
                rakaly_free_melt(melted);
            }
        }
        "ck3" => {
            let data = request("ck3saves-test-cases", "af_Munso_867_Ironman.ck3");
            unsafe {
                let melted = rakaly_ck3_melt(data.as_ptr() as *const i8, data.len());
                if rakaly_melt_error_code(melted) != 0 {
                    panic!("ck3 melt failed");
                }

                let out_len = rakaly_melt_data_length(melted);
                let mut out: Vec<u8> = vec![0; out_len];
                let _wrote_len =
                    rakaly_melt_write_data(melted, out.as_mut_ptr() as *mut i8, out.len());
                let _ = std::io::stdout().write_all(out.as_slice());
                rakaly_free_melt(melted);
            }
        }
        "imperator" => {
            let data = request("imperator-test-cases", "observer1.5.rome");
            unsafe {
                let melted = rakaly_imperator_melt(data.as_ptr() as *const i8, data.len());
                if rakaly_melt_error_code(melted) != 0 {
                    panic!("imperator melt failed");
                }

                let out_len = rakaly_melt_data_length(melted);
                let mut out: Vec<u8> = vec![0; out_len];
                let _wrote_len =
                    rakaly_melt_write_data(melted, out.as_mut_ptr() as *mut i8, out.len());
                let _ = std::io::stdout().write_all(out.as_slice());
                rakaly_free_melt(melted);
            }
        }
        "hoi4" => {
            let data = request("hoi4saves-test-cases", "1.10-ironman.zip");
            let reader = Cursor::new(&data[..]);
            let mut zip = zip::ZipArchive::new(reader).unwrap();
            let mut zip_file = zip.by_index(0).unwrap();
            let mut buffer = Vec::with_capacity(0);
            zip_file.read_to_end(&mut buffer).unwrap();

            unsafe {
                let melted = rakaly_hoi4_melt(buffer.as_ptr() as *const i8, buffer.len());
                if rakaly_melt_error_code(melted) != 0 {
                    panic!("imperator melt failed");
                }

                let out_len = rakaly_melt_data_length(melted);
                let mut out: Vec<u8> = vec![0; out_len];
                let _wrote_len =
                    rakaly_melt_write_data(melted, out.as_mut_ptr() as *mut i8, out.len());
                let _ = std::io::stdout().write_all(out.as_slice());
                rakaly_free_melt(melted);
            }
        }
        _ => panic!("unrecognized game mode"),
    };
}

/// Request data from s3 and cache it locally
pub fn request<S: AsRef<str>>(bucket_name: &str, input: S) -> Vec<u8> {
    let reffed = input.as_ref();
    let cache = Path::new("assets").join("saves").join(reffed);
    if cache.exists() {
        fs::read(cache).unwrap()
    } else {
        let url = format!("https://{}.s3.us-west-002.backblazeb2.com/{}", bucket_name, reffed);
        let resp = attohttpc::get(&url).send().unwrap();

        if !resp.is_success() {
            panic!("expected a 200 code from s3");
        } else {
            let data = resp.bytes().unwrap();
            std::fs::create_dir_all(cache.parent().unwrap()).unwrap();
            std::fs::write(&cache, &data).unwrap();
            data
        }
    }
}
