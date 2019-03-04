use crate::execution::*;
use crate::executor::*;
use crate::store::*;
use failure::Error;

/// This is a client of the Executor, the client is who sends a DAG for an
/// evaluation, provides some files and receives the callbacks from the server.
/// When the server notifies a callback function is called by the client.
pub struct ExecutorClient;

impl ExecutorClient {
    /// Begin the evaluation sending the DAG to the server, sending the files
    /// as needed and storing the files from the server. This method is
    /// blocking until the server ends the computation.
    ///
    /// * `dag` - The ExecutionDAG to evaluate
    /// * `sender` - A channel that sends messages to the server
    /// * `receiver` - A channel that receives messages from the server
    pub fn evaluate(
        dag: ExecutionDAG,
        sender: ChannelSender,
        receiver: ChannelReceiver,
    ) -> Result<(), Error> {
        trace!("ExecutorClient started");
        // list all the files/executions that want callbacks
        let dag_callbacks = ExecutionDAGCallbacks {
            executions: dag.execution_callbacks.keys().map(|k| k.clone()).collect(),
            files: dag.file_callbacks.keys().map(|k| k.clone()).collect(),
        };
        let provided_files = dag.data.provided_files.clone();
        serialize_into(
            &ExecutorClientMessage::Evaluate {
                dag: dag.data,
                callbacks: dag_callbacks,
            },
            &sender,
        )?;
        loop {
            match deserialize_from::<ExecutorServerMessage>(&receiver) {
                Ok(ExecutorServerMessage::AskFile(uuid)) => {
                    info!("Server is asking for {}", uuid);
                    let path = &provided_files
                        .get(&uuid)
                        .expect("Server asked for non provided file")
                        .local_path;
                    let key = FileStoreKey::from_file(path)?;
                    serialize_into(&ExecutorClientMessage::ProvideFile(uuid, key), &sender)?;
                    ChannelFileSender::send(&path, &sender)?;
                }
                Ok(ExecutorServerMessage::ProvideFile(uuid)) => {
                    info!("Server sent the file {}", uuid);
                    if let Some(callback) = dag.file_callbacks.get(&uuid) {
                        if let Some(write_to) = callback.write_to.as_ref() {
                            info!("Writing {} to {:?}", uuid, write_to);
                            // TODO write file
                        }
                        if let Some((_limit, get_content)) = callback.get_content.as_ref() {
                            get_content(vec![1, 2, 3, 42]);
                            // TODO send file
                        }
                    }
                }
                Ok(ExecutorServerMessage::NotifyStart(uuid, worker)) => {
                    info!("Execution {} started on {}", uuid, worker);
                    if let Some(callbacks) = dag.execution_callbacks.get(&uuid) {
                        if let Some(callback) = &callbacks.on_start {
                            callback(worker);
                        }
                    }
                }
                Ok(ExecutorServerMessage::NotifyDone(uuid, result)) => {
                    info!("Execution {} completed with {:?}", uuid, result);
                    if let Some(callbacks) = dag.execution_callbacks.get(&uuid) {
                        if let Some(callback) = &callbacks.on_done {
                            callback(result);
                        }
                    }
                }
                Ok(ExecutorServerMessage::NotifySkip(uuid)) => {
                    info!("Execution {} skipped", uuid);
                    if let Some(callbacks) = dag.execution_callbacks.get(&uuid) {
                        if let Some(callback) = &callbacks.on_skip {
                            callback();
                        }
                    }
                }
                Ok(ExecutorServerMessage::Error(error)) => {
                    info!("Error occurred: {}", error);
                    // TODO abort
                    drop(receiver);
                    break;
                }
                Ok(ExecutorServerMessage::Status(status)) => {
                    info!("Server status: {:#?}", status);
                }
                Ok(ExecutorServerMessage::Done) => {
                    info!("Execution completed!");
                    drop(receiver);
                    break;
                }
                Err(e) => {
                    let cause = e.find_root_cause().to_string();
                    if cause == "receiving on a closed channel" {
                        trace!("Connection closed: {}", cause);
                        break;
                    } else {
                        error!("Connection error: {}", cause);
                    }
                }
            }
        }
        Ok(())
    }
}
