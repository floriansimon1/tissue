use std::{process, sync};

use command_fds::{CommandFdExt, FdMapping};

use crate::system::libc_wrappers;
use crate::logging::logger;
use crate::io::safe_stdio;

const DEFAULT_PAGER:           &'static str = "less";
const DEFAULT_PAGER_ARGUMENTS: &'static str = "-FRX";

lazy_static::lazy_static! {
    static ref CURRENTLY_HIJACKING_STDIO: sync::Mutex<()> = sync::Mutex::new(());
}

pub struct Pager {
    guard:        sync::MutexGuard<'static, ()>,
    child:        sync::Arc<process::Child>,
    closed:       bool,
    real_stdout:  i32,
}

impl Drop for Pager {
    fn drop(&mut self) {
        self.close_if_necessary();

        drop(&self.guard);
    }
}

impl Pager {
    pub fn new() -> Pager {
        let guard = CURRENTLY_HIJACKING_STDIO.try_lock().expect("Trying to hold multiple Pager instances!");

        let (piped_stdout, new_stdout) = libc_wrappers::pipe();
        let real_stdout                = libc_wrappers::dup(libc::STDOUT_FILENO);

        // println!() in Tissue will now write to the pipe.
        libc_wrappers::dup2(new_stdout, libc::STDOUT_FILENO);

        libc_wrappers::close(new_stdout);

        let child = sync
        ::Arc
        ::new(
            process
            ::Command
            ::new(DEFAULT_PAGER)
            .arg(DEFAULT_PAGER_ARGUMENTS)
            .fd_mappings(vec![
                FdMapping {
                    parent_fd: piped_stdout,
                    child_fd:  libc::STDIN_FILENO,
                },

                FdMapping {
                    parent_fd: real_stdout,
                    child_fd:  libc::STDOUT_FILENO,
                },

                FdMapping {
                    parent_fd: libc::STDERR_FILENO,
                    child_fd:  libc::STDERR_FILENO,
                },
            ])
            .unwrap()
            .spawn()
            .unwrap()
        );

        // It's become useless now that it's been copied in the child process.
        libc_wrappers::close(piped_stdout);

        Pager { guard, child, real_stdout, closed: false }
    }

    pub fn page_lines(&mut self, logger: &logger::Logger, iterator: impl Iterator<Item = String>) -> bool {
        let mut empty = true;

        /*
        * We want whatever should have been printed to stdout by now to be flushed before
        * we start writing our paged data to preserve order.
        */
        logger.try_flush_all();

        for output_text in iterator {
            empty = false;

            let wait_result = sync::Arc::get_mut(&mut self.child).unwrap().try_wait();

            if wait_result.map(|status| status.is_some() ).unwrap_or(false) {
                break;
            }

            safe_stdio::safe_println(&output_text);
        }

        empty
    }

    fn close_if_necessary(&mut self) {
        if self.closed {
            return;
        }

        libc_wrappers::dup2(self.real_stdout, libc::STDOUT_FILENO);

        self.closed = true;
    }

    pub fn wait(mut self) {
        self.close_if_necessary();

        let _ = sync::Arc::get_mut(&mut self.child).unwrap().wait();
    }
}
