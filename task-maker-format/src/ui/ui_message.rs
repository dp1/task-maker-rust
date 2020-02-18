use std::path::PathBuf;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use task_maker_exec::ExecutorStatus;

use crate::ioi::{SubtaskId, TestcaseId};
use crate::terry::{Seed, SolutionOutcome};
use crate::ui::UIExecutionStatus;
use crate::{ioi, terry};

/// A message sent to the UI.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UIMessage {
    /// A message asking the UI to exit.
    StopUI,

    /// An update on the status of the executor.
    ServerStatus {
        /// The status of the executor.
        status: ExecutorStatus<SystemTime>,
    },

    /// An update on the compilation status.
    Compilation {
        /// The compilation of this file.
        file: PathBuf,
        /// The status of the compilation.
        status: UIExecutionStatus,
    },

    /// An update on the stdout of a compilation.
    CompilationStdout {
        /// The compilation of this file.
        file: PathBuf,
        /// The prefix of the stdout of the compilation.
        content: String,
    },

    /// An update on the stderr of a compilation.
    CompilationStderr {
        /// The compilation of this file.
        file: PathBuf,
        /// The prefix of the stderr of the compilation.
        content: String,
    },

    /// The information about the task which is being run.
    IOITask {
        /// The task information.
        task: Box<ioi::Task>,
    },

    /// The generation of a testcase in a IOI task.
    IOIGeneration {
        /// The id of the subtask.
        subtask: SubtaskId,
        /// The id of the testcase.
        testcase: TestcaseId,
        /// The status of the generation.
        status: UIExecutionStatus,
    },

    /// An update on the stderr of the generation of a testcase.
    IOIGenerationStderr {
        /// The id of the subtask.
        subtask: SubtaskId,
        /// The id of the testcase.
        testcase: TestcaseId,
        /// The prefix of the stderr of the generation.
        content: String,
    },

    /// The validation of a testcase in a IOI task.
    IOIValidation {
        /// The id of the subtask.
        subtask: SubtaskId,
        /// The id of the testcase.
        testcase: TestcaseId,
        /// The status of the validation.
        status: UIExecutionStatus,
    },

    /// An update on the stderr of the validation of a testcase.
    IOIValidationStderr {
        /// The id of the subtask.
        subtask: SubtaskId,
        /// The id of the testcase.
        testcase: TestcaseId,
        /// The prefix of the stderr of the validator.
        content: String,
    },

    /// The solution of a testcase in a IOI task.
    IOISolution {
        /// The id of the subtask.
        subtask: SubtaskId,
        /// The id of the testcase.
        testcase: TestcaseId,
        /// The status of the solution.
        status: UIExecutionStatus,
    },

    /// The evaluation of a solution in a IOI task.
    IOIEvaluation {
        /// The id of the subtask.
        subtask: SubtaskId,
        /// The id of the testcase.
        testcase: TestcaseId,
        /// The path of the solution.
        solution: PathBuf,
        /// The status of the solution.
        status: UIExecutionStatus,
    },

    /// The checking of a solution in a IOI task.
    IOIChecker {
        /// The id of the subtask.
        subtask: SubtaskId,
        /// The id of the testcase.
        testcase: TestcaseId,
        /// The path of the solution.
        solution: PathBuf,
        /// The status of the solution. Note that a failure of this execution
        /// may not mean that the checker failed.
        status: UIExecutionStatus,
    },

    /// The score of a testcase is ready.
    IOITestcaseScore {
        /// The id of the subtask.
        subtask: SubtaskId,
        /// The id of the testcase.
        testcase: TestcaseId,
        /// The path of the solution.
        solution: PathBuf,
        /// The score of the testcase.
        score: f64,
        /// The message associated with the score.
        message: String,
    },

    /// The score of a subtask is ready.
    IOISubtaskScore {
        /// The id of the subtask.
        subtask: SubtaskId,
        /// The path of the solution.
        solution: PathBuf,
        /// The normalized score, a value between 0 and 1
        normalized_score: f64,
        /// The score of the subtask.
        score: f64,
    },

    /// The score of a task is ready.
    IOITaskScore {
        /// The path of the solution.
        solution: PathBuf,
        /// The score of the task.
        score: f64,
    },

    /// The compilation of a booklet.
    IOIBooklet {
        /// The name of the booklet.
        name: String,
        /// The status of the compilation.
        status: UIExecutionStatus,
    },

    /// The compilation of a dependency of a booklet. It can be processed many times, for example an
    /// asy file is compiled first, and then cropped.
    IOIBookletDependency {
        /// The name of the booklet.
        booklet: String,
        /// The name of the dependency.
        name: String,
        /// The index (0-based) of the step of this compilation.
        step: usize,
        /// The number of steps of the compilation of this dependency.
        num_steps: usize,
        /// The status of this step.
        status: UIExecutionStatus,
    },

    /// The information about the task which is being run.
    TerryTask {
        /// The task information.
        task: Box<terry::Task>,
    },

    /// The generation of a testcase in a Terry task.
    TerryGeneration {
        /// The path of the solution.
        solution: PathBuf,
        /// The seed used to generate the input file.
        seed: Seed,
        /// The status of the generation.
        status: UIExecutionStatus,
    },

    /// The validation of a testcase in a Terry task.
    TerryValidation {
        /// The path of the solution.
        solution: PathBuf,
        /// The status of the validation.
        status: UIExecutionStatus,
    },

    /// The solution of a testcase in a Terry task.
    TerrySolution {
        /// The path of the solution.
        solution: PathBuf,
        /// The status of the solution.
        status: UIExecutionStatus,
    },

    /// The checking of a solution in a Terry task.
    TerryChecker {
        /// The path of the solution.
        solution: PathBuf,
        /// The status of the checker.
        status: UIExecutionStatus,
    },

    /// The outcome of a solution in a Terry task.
    TerrySolutionOutcome {
        /// The path of the solution.
        solution: PathBuf,
        /// The outcome of the solution. `Err` is caused by an invalid response from the checker.
        outcome: Result<SolutionOutcome, String>,
    },

    /// A warning has been emitted.
    Warning {
        /// The message of the warning.
        message: String,
    },
}
