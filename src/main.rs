use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use regex::Regex;
use std::process::Command;

/// Fast and efficient YouTube video clip downloader
#[derive(Parser, Debug)]
#[command(name = "ytclip")]
#[command(author = "Eden Chan")]
#[command(version = "1.0.0")]
#[command(about = "Download specific clips from YouTube videos", long_about = None)]
struct Args {
    /// YouTube URL to download from
    url: String,

    /// Start time (e.g., 1:30, 90, 1:30:45)
    start_time: String,

    /// End time (e.g., 2:45, 165, 2:45:30)
    end_time: String,

    /// Custom output filename (optional)
    #[arg(short, long)]
    output: Option<String>,

    /// Playback speed (0.5 to 4.0)
    #[arg(short, long, default_value = "1.0")]
    speed: f64,
}

fn parse_time(time_str: &str) -> Result<f64> {
    let parts: Vec<&str> = time_str.split(':').collect();

    let seconds = match parts.len() {
        1 => {
            // Just seconds
            parts[0].parse::<f64>()
                .context("Invalid seconds format")?
        }
        2 => {
            // MM:SS
            let minutes = parts[0].parse::<f64>()
                .context("Invalid minutes format")?;
            let seconds = parts[1].parse::<f64>()
                .context("Invalid seconds format")?;
            minutes * 60.0 + seconds
        }
        3 => {
            // HH:MM:SS
            let hours = parts[0].parse::<f64>()
                .context("Invalid hours format")?;
            let minutes = parts[1].parse::<f64>()
                .context("Invalid minutes format")?;
            let seconds = parts[2].parse::<f64>()
                .context("Invalid seconds format")?;
            hours * 3600.0 + minutes * 60.0 + seconds
        }
        _ => return Err(anyhow::anyhow!("Invalid time format: {}", time_str))
    };

    Ok(seconds)
}

fn extract_video_id(url: &str) -> Option<String> {
    // Try standard youtube.com format
    let re = Regex::new(r"(?:youtube\.com/watch\?v=|youtu\.be/|youtube\.com/embed/)([a-zA-Z0-9_-]{11})").unwrap();

    if let Some(captures) = re.captures(url) {
        return captures.get(1).map(|m| m.as_str().to_string());
    }

    None
}

fn get_video_title(url: &str) -> Result<String> {
    println!("{}", "[INFO] Fetching video title...".blue());

    let output = Command::new("yt-dlp")
        .args(&["--get-title", "--no-playlist", url])
        .output()
        .context("Failed to execute yt-dlp. Is it installed?")?;

    if !output.status.success() {
        return Ok("video".to_string());
    }

    let title = String::from_utf8_lossy(&output.stdout)
        .trim()
        .to_string();

    // Clean filename
    let safe_title = title
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            _ => c
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    Ok(safe_title)
}

fn build_ffmpeg_command(
    url: &str,
    start_seconds: f64,
    duration: f64,
    output_file: &str,
    speed: f64,
) -> Vec<String> {
    let mut args = vec![
        "-ss".to_string(),
        start_seconds.to_string(),
        "-i".to_string(),
        url.to_string(),
        "-t".to_string(),
        duration.to_string(),
    ];

    // Add speed adjustment if needed
    if (speed - 1.0).abs() > 0.01 {
        let video_filter = format!("setpts={:.2}*PTS", 1.0 / speed);
        let audio_filter = format!("atempo={:.2}", speed.min(2.0));

        args.push("-filter:v".to_string());
        args.push(video_filter);

        if speed > 2.0 {
            // For speeds > 2x, chain atempo filters
            let mut tempo = speed;
            let mut atempo_chain = Vec::new();
            while tempo > 2.0 {
                atempo_chain.push("atempo=2.0".to_string());
                tempo /= 2.0;
            }
            if tempo > 1.0 {
                atempo_chain.push(format!("atempo={:.2}", tempo));
            }
            args.push("-filter:a".to_string());
            args.push(atempo_chain.join(","));
        } else {
            args.push("-filter:a".to_string());
            args.push(audio_filter);
        }
    }

    // QuickTime compatible encoding
    args.extend_from_slice(&[
        "-c:v".to_string(), "libx264".to_string(),
        "-c:a".to_string(), "aac".to_string(),
        "-pix_fmt".to_string(), "yuv420p".to_string(),
        "-movflags".to_string(), "faststart".to_string(),
        "-preset".to_string(), "fast".to_string(),
        "-crf".to_string(), "23".to_string(),
        "-y".to_string(),  // Overwrite output
        output_file.to_string(),
    ]);

    args
}

fn download_clip(
    url: &str,
    start_time: &str,
    end_time: &str,
    output_name: Option<String>,
    speed: f64,
) -> Result<()> {
    // Parse times
    let start_seconds = parse_time(start_time)?;
    let end_seconds = parse_time(end_time)?;

    if end_seconds <= start_seconds {
        return Err(anyhow::anyhow!("End time must be after start time"));
    }

    let duration = end_seconds - start_seconds;

    // Get video ID
    let video_id = extract_video_id(url)
        .ok_or_else(|| anyhow::anyhow!("Could not extract video ID from URL"))?;

    println!("{} Video ID: {}", "[INFO]".blue(), video_id);
    println!("{} Clipping from {} to {} (duration: {:.1}s)",
             "[TIME]".yellow(), start_time, end_time, duration);

    if (speed - 1.0).abs() > 0.01 {
        println!("{} Speed: {:.1}x", "[SPEED]".magenta(), speed);
    }

    // Get video title for output filename
    let title = get_video_title(url).unwrap_or_else(|_| "video".to_string());

    let output_file = output_name.unwrap_or_else(|| {
        if (speed - 1.0).abs() > 0.01 {
            format!("{}_clip_{}-{}_{}x.mp4",
                    title,
                    start_time.replace(':', "-"),
                    end_time.replace(':', "-"),
                    speed)
        } else {
            format!("{}_clip_{}_{}.mp4",
                    title,
                    start_time.replace(':', "-"),
                    end_time.replace(':', "-"))
        }
    });

    println!("{} Streaming clip...", "[INFO]".blue());

    // Get direct URL using yt-dlp
    let direct_url_output = Command::new("yt-dlp")
        .args(&[
            "--no-playlist",
            "-f", "best[ext=mp4]/best",
            "--get-url",
            url
        ])
        .output()
        .context("Failed to get video URL with yt-dlp")?;

    if !direct_url_output.status.success() {
        return Err(anyhow::anyhow!("Failed to extract video URL"));
    }

    let direct_url = String::from_utf8_lossy(&direct_url_output.stdout)
        .trim()
        .to_string();

    // Build and run ffmpeg command
    let ffmpeg_args = build_ffmpeg_command(
        &direct_url,
        start_seconds,
        duration,
        &output_file,
        speed,
    );

    let status = Command::new("ffmpeg")
        .args(&ffmpeg_args)
        .status()
        .context("Failed to execute ffmpeg. Is it installed?")?;

    if !status.success() {
        return Err(anyhow::anyhow!("FFmpeg failed to process the video"));
    }

    println!("{} Clip saved as: {}",
             "[SUCCESS]".green().bold(),
             output_file.cyan());

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Validate speed
    if args.speed < 0.5 || args.speed > 4.0 {
        return Err(anyhow::anyhow!("Speed must be between 0.5 and 4.0"));
    }

    // Download the clip
    download_clip(
        &args.url,
        &args.start_time,
        &args.end_time,
        args.output,
        args.speed,
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_time() {
        assert_eq!(parse_time("30").unwrap(), 30.0);
        assert_eq!(parse_time("1:30").unwrap(), 90.0);
        assert_eq!(parse_time("1:30:45").unwrap(), 5445.0);
    }

    #[test]
    fn test_extract_video_id() {
        assert_eq!(
            extract_video_id("https://www.youtube.com/watch?v=dQw4w9WgXcQ"),
            Some("dQw4w9WgXcQ".to_string())
        );
        assert_eq!(
            extract_video_id("https://youtu.be/dQw4w9WgXcQ"),
            Some("dQw4w9WgXcQ".to_string())
        );
    }
}