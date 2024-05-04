use std::path::PathBuf;

/// Utilities for the Miku Internet Daemon (mikuinetd).
#[derive(clap::Parser)]
pub enum Args {
    Precompress(PrecompressArgs),
    Server(ServerArgs),
}

/// Given an ASCII video, precompresses it into a stream of ASCII
/// characters.
#[derive(clap::Args)]
pub struct PrecompressArgs {
    /// An ASCII file to generate the video from.
    pub input: PathBuf,

    /// Height of each frame
    #[clap(short = 'h', long)]
    pub frame_height: usize,
}

/// Run the Miku Internet Daemon.
#[derive(clap::Args)]
pub struct ServerArgs {
    /// Path to a directory of compressed ASCII videos.
    pub miku_videos: PathBuf,

    /// Port to handle telnet connections on. If not provided, telnet daemon
    /// will not be started.
    pub telnet: Option<u16>,
}
