//Copyright 2016 William Cody Laeder
//
//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
//limitations under the License.

//!Make File Descriptors Nonblocking
//!
//!Creates a simple interface around fcntl to set file descriptors as non
//!blocking. This crate will work on all `unix` systems as classified by
//!Rust. 

extern crate mio;
use mio::unix::EventedFd;
extern crate libc;
use libc::{F_GETFD,F_SETFD,fcntl,O_NONBLOCK};
use std::io::Error;
use std::os::unix::io::{AsRawFd,RawFd};


///Make File Descriptor NonBlocking
///
///Sets the O_NONBLOCK for more information [follow this
///link](http://man7.org/linux/man-pages/man2/fcntl.2.html)
///
pub fn set_non_blocking<'a, F>(fd: &'a F) -> Result<(),Error>
where F: AsRawFd
{
    let fd = fd.as_raw_fd();
    unsafe{
        let flags = fcntl(fd, F_GETFD, 0);
        let status = fcntl(fd,F_SETFD, flags | O_NONBLOCK);
        if status < 0 {
            Err(Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

///Make an Evented Fd
///
///Creates a simple and safe interface to interface with MIO. The goal is
///to allow mio to work with File IO. Provided your system is unix this
///will let it interface with MIO event loop.
///
///The `RawFd` passed to this function will be made non-blocking.
///
pub fn evented_fd<'a>(fd: &'a RawFd) -> Result<EventedFd<'a>,Error> {
    let f_d = *fd;
    unsafe{
        let flags = fcntl(f_d, F_GETFD, 0);
        let status = fcntl(f_d,F_SETFD, flags | O_NONBLOCK);
        if status < 0 {
            Err(Error::last_os_error())
        } else {
            Ok(EventedFd(fd))
        }
    }
}
