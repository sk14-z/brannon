use crate::input::key::Protocol;
use crate::input::{Input, parse};
use crate::{printf, printlnf};
use libc::*;
use std::io::{Read, stdin};
use std::os::unix::io::AsRawFd;

pub(crate) fn clear() {
    printf!("\x1b[2J");
}

// (x, y)
pub(crate) fn termsz() -> (usize, usize) {
    let mut winsz: winsize = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };

    unsafe {
        ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut winsz);
    }

    (winsz.ws_col as usize, winsz.ws_row as usize)
}

// Temporary until events are implemented
pub(crate) fn poll_input() -> Option<Input> {
    let mut buf = [0; 512];

    match stdin().read(&mut buf) {
        Ok(0) | Err(_) => None,
        Ok(n) => parse(&buf[..n]),
    }
    // None
}

pub(crate) fn poll_until_i_can_code() -> Vec<Input> {
    let mut inputs = vec![];

    let mut buf = [0; 512];

    // match stdin().read(&mut buf) {
    //     Ok(n) => {
    //         if n > 2 {
    //             let input_parts = buf[..n].split(|b| *b == b'u');
    //
    //             for part in input_parts {
    //                 if let Some(input) = parse(part) {
    //                     inputs.push(input);
    //                 }
    //             }
    //         } else {
    //             if let Some(input) = parse(&buf[..n]) {
    //                 inputs.push(input);
    //             }
    //         }
    //     }
    //     _ => {}
    // }

    if let Ok(n) = stdin().read(&mut buf) {
        if n > 2 {
            let input_parts = buf[..n].split_inclusive(|b| *b == b'u');

            for part in input_parts {
                if let Some(input) = parse(part) {
                    inputs.push(input);
                }
            }
        } else if let Some(input) = parse(&buf[..n]) {
            inputs.push(input);
        }
    }

    inputs
}

pub(crate) struct Terminal {
    // Modes
    canonical_mode: termios,
}

impl Terminal {
    pub(crate) fn initialize() -> Terminal {
        let mut canonical_mode: termios = termios {
            c_iflag: 0,
            c_oflag: 0,
            c_cflag: 0,
            c_lflag: 0,
            c_line: 0,
            c_cc: [0; 32],
            c_ispeed: 0,
            c_ospeed: 0,
        };

        let mut raw_mode: termios;

        unsafe {
            tcgetattr(STDIN_FILENO, &mut canonical_mode);
            raw_mode = canonical_mode;
            raw_mode.c_lflag &= !(ICANON | ECHO);
        }

        printlnf!("\x1b[?47h");

        let stdin_fd = stdin().as_raw_fd();

        unsafe {
            fcntl(
                stdin_fd,
                F_SETFL,
                libc::fcntl(stdin_fd, F_GETFL) | O_NONBLOCK,
            );

            tcsetattr(STDIN_FILENO, TCSANOW, &raw_mode);
        }

        Terminal { canonical_mode }
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        unsafe {
            tcsetattr(STDIN_FILENO, TCSANOW, &self.canonical_mode);
        }

        printlnf!("\x1b[?47l\x1b[?25h");

        Protocol::Default.activate();
    }
}
