use nix::sys::wait::*;
use nix::unistd::Pid;
use nix::Error;

// @todo not sure how to handle resluts from the waitpid call
//       will worry about that when the main io loop is more developed
pub fn wait_pid(pid: Pid) -> Result<WaitStatus, Error> {
    waitpid(pid, None)
}
