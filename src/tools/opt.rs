use crate::{LoggerOpt, StorageOpt};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "task-maker-tools",
    setting = structopt::clap::AppSettings::ColoredHelp,
)]
pub struct Opt {
    #[structopt(flatten)]
    pub logger: LoggerOpt,

    /// Which tool to use
    #[structopt(subcommand)]
    pub tool: Tool,
}

#[derive(StructOpt, Debug)]
pub enum Tool {
    /// Clear a task directory
    Clear(ClearOpt),
    /// Generate the autocompletion files for the shell
    GenAutocompletion(GenAutocompletionOpt),
    /// Spawn an instance of the server
    Server(ServerOpt),
    /// Spawn an instance of a worker
    Worker(WorkerOpt),
    /// Print the TypeScript type definitions
    Typescriptify,
    /// Wipe the internal storage of task-maker
    ///
    /// Warning: no other instances of task-maker should be running when this flag is provided.
    Reset(ResetOpt),
    /// Run a command inside a sandbox similar to the one used by task-maker
    Sandbox(SandboxOpt),
}

#[derive(StructOpt, Debug)]
pub struct ClearOpt {
    /// Directory of the task to clear
    #[structopt(short = "t", long = "task-dir", default_value = "")]
    pub task_dir: PathBuf,

    /// Look at most for this number of parents for searching the task
    #[structopt(long = "max-depth", default_value = "3")]
    pub max_depth: u32,
}

#[derive(StructOpt, Debug)]
pub struct GenAutocompletionOpt {
    /// Where to write the autocompletion files
    #[structopt(short = "t", long = "target")]
    pub target: Option<PathBuf>,
}

#[derive(StructOpt, Debug, Clone)]
pub struct ServerOpt {
    /// Address to bind the server on for listening for the clients
    #[structopt(default_value = "0.0.0.0:27182")]
    pub client_addr: String,

    /// Address to bind the server on for listening for the workers
    #[structopt(default_value = "0.0.0.0:27183")]
    pub worker_addr: String,

    /// Password for the connection of the clients
    #[structopt(long = "client-password")]
    pub client_password: Option<String>,

    /// Password for the connection of the workers
    #[structopt(long = "worker-password")]
    pub worker_password: Option<String>,

    #[structopt(flatten)]
    pub storage: StorageOpt,
}

#[derive(StructOpt, Debug, Clone)]
pub struct WorkerOpt {
    /// Address to use to connect to a remote server
    pub server_addr: String,

    /// ID of the worker (to differentiate between multiple workers on the same machine).
    pub worker_id: Option<u32>,

    /// The name to use for the worker in remote executions
    #[structopt(long)]
    pub name: Option<String>,

    #[structopt(flatten)]
    pub storage: StorageOpt,
}

#[derive(StructOpt, Debug, Clone)]
pub struct ResetOpt {
    #[structopt(flatten)]
    pub storage: StorageOpt,
}

#[derive(StructOpt, Debug, Clone)]
pub struct SandboxOpt {
    /// Working directory of the sandbox.
    ///
    /// Will be mounted in /box inside the sandbox. Defaults to current working directory.
    #[structopt(long, short)]
    pub workdir: Option<PathBuf>,

    /// Memory limit for the sandbox, in KiB.
    #[structopt(long, short)]
    pub memory_limit: Option<u64>,

    /// Stack limit for the sandbox, in KiB.
    #[structopt(long, short)]
    pub stack_limit: Option<u64>,

    /// Prevent forking.
    #[structopt(long)]
    pub single_process: bool,

    /// List of additional directory mounted read-only inside the sandbox.
    #[structopt(long, short)]
    pub readable_dirs: Vec<PathBuf>,

    /// Mount /tmp and /dev/null inside the sandbox
    #[structopt(long)]
    pub mount_tmpfs: bool,

    /// User id.
    #[structopt(long, default_value = "1000")]
    pub uid: usize,

    /// User id.
    #[structopt(long, default_value = "1000")]
    pub gid: usize,

    /// Command to execute inside the sandbox. If not specified, bash is executed.
    pub command: Vec<String>,
}
