pub mod shell;

use crate::execution::{Request, Response};
use crate::execution::runner::Runner as ExecutionRunner;
use shell::Request as ShellRequest;
use shell::Shell;
use std::sync::mpsc::Sender;

pub struct Runner {}

impl Runner {
    /// Execute the given request, using the given producer to send the execution response
    /// asynchronously to the calling processor.
    pub fn execute(request: &Request, producer: &Sender<Response>) -> Result<(), ()> {
        match request.get_runner() {
            ExecutionRunner::Shell { command } => {
                Shell::execute(
                    ShellRequest::new(*request.get_identifier(), request.get_job().clone(), command.clone()),
                    producer,
                )
            },
            _ => {
                Err(())
            }
        }
    }
}
