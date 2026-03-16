use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Serialize, Deserialize)]
pub struct AudioMeta {
    pub artist: String,
    pub album: String,
    pub title: String,
    pub track_number: u32,
    pub duration: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct InstalledTrack {
    pub dev_name: String,
    pub title: String,
    pub artist: String,
    pub track: String,
    pub start: i32,
}

// returning path to ffmpeg bundled on Win, system on Linux
fn ffmpeg_path() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        if let Ok(exe) = std::env::current_exe() {
            if let Some(dir) = exe.parent() {
                let bundled = dir.join("ffmpeg.exe");
                if bundled.exists() {
                    return bundled;
                }
            }
        }
        PathBuf::from("ffmpeg.exe")
    }
    #[cfg(not(target_os = "windows"))]
    {
        PathBuf::from("ffmpeg")
    }
}

// returning path to ffprobe bundled on Win, system on Linux
fn ffprobe_path() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        if let Ok(exe) = std::env::current_exe() {
            if let Some(dir) = exe.parent() {
                let bundled = dir.join("ffprobe.exe");
                if bundled.exists() {
                    return bundled;
                }
            }
        }
        PathBuf::from("ffprobe.exe")
    }
    #[cfg(not(target_os = "windows"))]
    {
        PathBuf::from("ffprobe")
    }
}

#[tauri::command]
fn read_audio_meta(file_path: String) -> Result<AudioMeta, String> {
    let out = Command::new(ffprobe_path())
        .args([
            "-v",
            "quiet",
            "-print_format",
            "json",
            "-show_format",
            &file_path,
        ])
        .output()
        .map_err(|_| "ffprobe not found. Install ffmpeg and make sure it's in PATH.".to_string())?;

    let json: serde_json::Value =
        serde_json::from_slice(&out.stdout).map_err(|e| format!("ffprobe parse error: {}", e))?;

    let tags = &json["format"]["tags"];

    let tag = |key: &str| -> String {
        let lo = tags[key].as_str().unwrap_or("").trim().to_string();
        if !lo.is_empty() {
            return lo;
        }
        tags[key.to_uppercase().as_str()]
            .as_str()
            .unwrap_or("")
            .trim()
            .to_string()
    };

    let track_number = tag("track")
        .split('/')
        .next()
        .and_then(|s| s.trim().parse::<u32>().ok())
        .unwrap_or(1);

    let artist = {
        let a = tag("artist");
        if a.is_empty() {
            tag("album_artist")
        } else {
            a
        }
    };

    let duration = json["format"]["duration"]
        .as_str()
        .and_then(|s| s.parse::<f64>().ok());

    Ok(AudioMeta {
        artist,
        album: tag("album"),
        title: tag("title"),
        track_number,
        duration,
    })
}

fn transliterate(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        let t = match c {
            'а' => "a",
            'б' => "b",
            'в' => "v",
            'г' => "g",
            'д' => "d",
            'е' => "e",
            'ё' => "yo",
            'ж' => "zh",
            'з' => "z",
            'и' => "i",
            'й' => "y",
            'к' => "k",
            'л' => "l",
            'м' => "m",
            'н' => "n",
            'о' => "o",
            'п' => "p",
            'р' => "r",
            'с' => "s",
            'т' => "t",
            'у' => "u",
            'ф' => "f",
            'х' => "kh",
            'ц' => "ts",
            'ч' => "ch",
            'ш' => "sh",
            'щ' => "sch",
            'ъ' => "",
            'ы' => "y",
            'ь' => "",
            'э' => "e",
            'ю' => "yu",
            'я' => "ya",
            'А' => "A",
            'Б' => "B",
            'В' => "V",
            'Г' => "G",
            'Д' => "D",
            'Е' => "E",
            'Ё' => "Yo",
            'Ж' => "Zh",
            'З' => "Z",
            'И' => "I",
            'Й' => "Y",
            'К' => "K",
            'Л' => "L",
            'М' => "M",
            'Н' => "N",
            'О' => "O",
            'П' => "P",
            'Р' => "R",
            'С' => "S",
            'Т' => "T",
            'У' => "U",
            'Ф' => "F",
            'Х' => "Kh",
            'Ц' => "Ts",
            'Ч' => "Ch",
            'Ш' => "Sh",
            'Щ' => "Sch",
            'Ъ' => "",
            'Ы' => "Y",
            'Ь' => "",
            'Э' => "E",
            'Ю' => "Yu",
            'Я' => "Ya",
            _ => {
                result.push(c);
                continue;
            }
        };
        result.push_str(t);
    }
    result
}

#[tauri::command]
fn check_deps() -> Result<(), String> {
    Command::new(ffmpeg_path())
        .arg("-version")
        .output()
        .map_err(|_| "ffmpeg not found. Install ffmpeg and make sure it's in PATH.".to_string())?;

    Command::new(ffprobe_path())
        .arg("-version")
        .output()
        .map_err(|_| "ffprobe not found. Install ffmpeg and make sure it's in PATH.".to_string())?;

    Ok(())
}

#[tauri::command]
fn read_music_kdr(game_path: String) -> Result<Vec<InstalledTrack>, String> {
    let path = Path::new(&game_path).join("music.kdr");
    let content = fs::read_to_string(&path).map_err(|e| format!("Can't read music.kdr: {}", e))?;

    let mut tracks = Vec::new();
    let mut block: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    let mut in_block = false;

    for line in content.lines() {
        let line = line.trim();
        if line == "{" {
            in_block = true;
            block.clear();
        } else if line == "}" && in_block {
            if let Some(dev_name) = block.get("dev_name") {
                tracks.push(InstalledTrack {
                    dev_name: dev_name.clone(),
                    title: block.get("title").cloned().unwrap_or_default(),
                    artist: block.get("artist").cloned().unwrap_or_default(),
                    track: block.get("track").cloned().unwrap_or_default(),
                    start: block.get("start").and_then(|s| s.parse().ok()).unwrap_or(0),
                });
            }
            in_block = false;
        } else if in_block {
            if let Some((k, v)) = line.split_once(':') {
                block.insert(k.trim().to_string(), v.trim().to_string());
            }
        }
    }

    Ok(tracks)
}

#[tauri::command]
fn backup_game_files(game_path: String) -> Result<(), String> {
    let files = ["data.win", "audiogroup3.dat", "music.kdr"];
    for f in &files {
        let src = Path::new(&game_path).join(f);
        let dst = Path::new(&game_path).join(format!("{}.bak", f));
        if dst.exists() {
            continue;
        }
        fs::copy(&src, &dst).map_err(|e| format!("Can't backup {}: {}", f, e))?;
    }
    Ok(())
}

#[tauri::command]
fn reset_custom_music(game_path: String) -> Result<(), String> {
    let files = ["data.win", "audiogroup3.dat", "music.kdr"];
    for f in &files {
        let bak = Path::new(&game_path).join(format!("{}.bak", f));
        let dst = Path::new(&game_path).join(f);
        if !bak.exists() {
            return Err(format!("{}.bak not found — backup missing", f));
        }
        fs::copy(&bak, &dst).map_err(|e| format!("Can't restore {}: {}", f, e))?;
    }

    Ok(())
}

#[tauri::command]
fn remove_track(game_path: String, dev_name: String) -> Result<(), String> {
    let path = Path::new(&game_path).join("music.kdr");
    let content = fs::read_to_string(&path).map_err(|e| format!("Can't read music.kdr: {}", e))?;

    let mut result = String::new();
    let mut block = String::new();
    let mut current_dev = String::new();
    let mut in_block = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed == "{" {
            in_block = true;
            block.clear();
            current_dev.clear();
            block.push_str(line);
            block.push('\n');
        } else if trimmed == "}" && in_block {
            block.push_str(line);
            block.push('\n');
            if current_dev != dev_name {
                result.push_str(&block);
            }
            in_block = false;
        } else if in_block {
            if let Some((k, v)) = trimmed.split_once(':') {
                if k.trim() == "dev_name" {
                    current_dev = v.trim().to_string();
                }
            }
            block.push_str(line);
            block.push('\n');
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }

    fs::write(&path, result).map_err(|e| format!("Can't write music.kdr: {}", e))
}

#[tauri::command]
fn install_track(
    game_path: String,
    file_path: String,
    track_id: String,
    dev_name: String,
    title: String,
    artist: String,
) -> Result<(), String> {
    let ag_dir = Path::new(&game_path).join("ag_music");
    fs::create_dir_all(&ag_dir).map_err(|e| format!("Can't create ag_music dir: {}", e))?;

    // convert to ogg and copy
    let input_ext = Path::new(&file_path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let ogg_path = ag_dir.join(format!("{}.ogg", track_id));

    if input_ext == "ogg" {
        fs::copy(&file_path, &ogg_path).map_err(|e| format!("Failed to copy ogg: {}", e))?;
    } else {
        let ff = Command::new(ffmpeg_path())
            .args([
                "-y",
                "-i",
                &file_path,
                "-c:a",
                "libvorbis",
                "-q:a",
                "4",
                "-ar",
                "44100",
                ogg_path.to_str().unwrap(),
            ])
            .output()
            .map_err(|_| {
                "ffmpeg not found. Install ffmpeg and make sure it's in PATH.".to_string()
            })?;

        if !ff.status.success() {
            return Err(format!(
                "ffmpeg failed: {}",
                String::from_utf8_lossy(&ff.stderr)
            ));
        }
    }

    let exe_dir = std::env::current_exe()
        .map_err(|e| format!("Can't get exe path: {}", e))?
        .parent()
        .ok_or("Can't get exe dir")?
        .to_path_buf();
    let res_dir = exe_dir.join("UTMT");

    let csx = std::env::temp_dir().join("kd_inject_music.csx");

    let ag_dir_str = ag_dir.to_str().unwrap().replace('\\', "/");

    let script = format!(
        r#"using UndertaleModLib;
using UndertaleModLib.Models;
using static UndertaleModLib.Models.UndertaleSound;
using static UndertaleModLib.UndertaleData;
using System.Threading.Tasks;

EnsureDataLoaded();

int audioGroupID = -1;
bool usesAGRP = (Data.AudioGroups.Count > 0);
string importFolder = @"{ag_dir}";
string folderName = new DirectoryInfo(importFolder).Name;

if (usesAGRP)
{{
    for (int i = 0; i < Data.AudioGroups.Count; i++)
    {{
        if (Data.AudioGroups[i]?.Name?.Content == folderName)
        {{
            audioGroupID = i;
            break;
        }}
    }}
    if (audioGroupID == -1)
    {{
        audioGroupID = Data.AudioGroups.Count;
        File.WriteAllBytes(
            Path.Combine(Path.GetDirectoryName(FilePath), $"audiogroup{{audioGroupID}}.dat"),
            Convert.FromBase64String("Rk9STQwAAABBVURPBAAAAAAAAAA=")
        );
        Data.AudioGroups.Add(new UndertaleAudioGroup() {{
            Name = Data.Strings.MakeString(folderName)
        }});
    }}
}}

bool needAGRP = usesAGRP && audioGroupID != Data.GetBuiltinSoundGroupID();

foreach (string file in Directory.GetFiles(importFolder))
{{
    string filename = Path.GetFileName(file);
    if (!filename.EndsWith(".ogg", StringComparison.InvariantCultureIgnoreCase) &&
        !filename.EndsWith(".wav", StringComparison.InvariantCultureIgnoreCase))
        continue;

    string soundName = Path.GetFileNameWithoutExtension(file);
    bool isOGG = Path.GetExtension(filename).ToLower() == ".ogg";

    UndertaleSound existingSound = null;
    for (int i = 0; i < Data.Sounds.Count; i++)
    {{
        if (Data.Sounds[i]?.Name?.Content == soundName)
        {{
            existingSound = Data.Sounds[i];
            break;
        }}
    }}
    if (existingSound != null) continue;

    UndertaleEmbeddedAudio soundData = new() {{ Data = File.ReadAllBytes(file) }};
    int audioID = -1;

    if (needAGRP)
    {{
        string agPath = Path.Combine(Path.GetDirectoryName(FilePath), $"audiogroup{{audioGroupID}}.dat");
        UndertaleData agDat;
        using (var fs = new FileStream(agPath, FileMode.Open, FileAccess.Read))
            agDat = UndertaleIO.Read(fs);
        agDat.EmbeddedAudio.Add(soundData);
        audioID = agDat.EmbeddedAudio.Count - 1;
        using (var fs = new FileStream(agPath, FileMode.Create))
            UndertaleIO.Write(fs, agDat);
    }}
    else
    {{
        Data.EmbeddedAudio.Add(soundData);
        audioID = Data.EmbeddedAudio.Count - 1;
    }}

    AudioEntryFlags flags = isOGG
        ? AudioEntryFlags.IsCompressed | AudioEntryFlags.Regular
        : AudioEntryFlags.IsEmbedded | AudioEntryFlags.Regular;

    UndertaleAudioGroup finalGroup = usesAGRP
        ? Data.AudioGroups[needAGRP ? audioGroupID : Data.GetBuiltinSoundGroupID()]
        : null;

    Data.Sounds.Add(new UndertaleSound()
    {{
        Name = Data.Strings.MakeString(soundName),
        Flags = flags,
        Type = Data.Strings.MakeString(isOGG ? ".ogg" : ".wav"),
        File = Data.Strings.MakeString(filename),
        Effects = 0,
        Volume = 1.0f,
        Pitch = 0.0f,
        AudioID = audioID,
        AudioFile = needAGRP ? null : Data.EmbeddedAudio[audioID],
        AudioGroup = finalGroup,
        GroupID = needAGRP ? audioGroupID : Data.GetBuiltinSoundGroupID()
    }});
}}
"#,
        ag_dir = ag_dir_str
    );

    fs::write(&csx, &script).map_err(|e| format!("Can't write csx to {:?}: {}", csx, e))?;

    // running UMT on win, dotnet on linux
    let data_win = Path::new(&game_path).join("data.win");
    let umt_out = if cfg!(target_os = "windows") {
        let umt = res_dir.join("UndertaleModCLI.exe");
        Command::new(&umt)
            .args([
                "load",
                data_win.to_str().unwrap(),
                "-s",
                csx.to_str().unwrap(),
                "-o",
                data_win.to_str().unwrap(),
            ])
            .output()
            .map_err(|e| format!("UndertaleModCLI not found: {}", e))?
    } else {
        let umt_bin = res_dir.join("UndertaleModCli");
        Command::new(&umt_bin)
            .args([
                "load",
                data_win.to_str().unwrap(),
                "-s",
                csx.to_str().unwrap(),
                "-o",
                data_win.to_str().unwrap(),
            ])
            .output()
            .map_err(|e| format!("UMT bin not found at {:?}: {}", umt_bin, e))?
    };

    if !umt_out.status.success() {
        return Err(format!(
            "UMT error: {}",
            String::from_utf8_lossy(&umt_out.stderr)
        ));
    }

    // clearing ag_music after injection
    let _ = fs::remove_file(&ogg_path);
    if let Ok(mut entries) = fs::read_dir(&ag_dir) {
        if entries.next().is_none() {
            let _ = fs::remove_dir(&ag_dir);
        }
    }

    // writing to music.kdr
    let kdr = Path::new(&game_path).join("music.kdr");
    let mut content =
        fs::read_to_string(&kdr).map_err(|e| format!("Can't read music.kdr: {}", e))?;
    content.push_str(&format!(
        "\n{{\n\tdev_name: {}\n\ttitle: {}\n\tartist: {}\n\ttrack: {}\n\tstart: 1\n}}",
        dev_name,
        transliterate(&title),
        transliterate(&artist),
        track_id
    ));
    fs::write(&kdr, content).map_err(|e| format!("Can't write music.kdr: {}", e))?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            read_audio_meta,
            check_deps,
            read_music_kdr,
            install_track,
            remove_track,
            backup_game_files,
            reset_custom_music,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
