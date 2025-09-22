use crate::input::{Input, parse};
use crate::{printf, printlnf};
use libc::*;
use std::io::{Read, stdin};
use std::os::unix::io::AsRawFd;

pub fn clear() {
    printf!("\x1b[2J");
}

// (x, y)
pub fn termsz() -> (usize, usize) {
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
pub fn poll_input() -> Option<Input> {
    let mut buf = [0; 512];

    match stdin().read(&mut buf) {
        Ok(0) | Err(_) => None,
        Ok(_) => {
            let end = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
            parse(&buf[0..end])
        }
    }
}

pub struct Terminal {
    canonical_mode: termios,
    raw_mode: termios,
}

impl Terminal {
    pub fn initialize() -> Terminal {
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

        Terminal {
            canonical_mode,
            raw_mode,
        }
    }

    pub fn save() {
        printlnf!("\x1b[?47h");
    }

    pub fn restore() {
        printlnf!("\x1b[?47l\x1b[?25h");
    }

    pub fn make_canonical(&mut self) {
        unsafe {
            tcsetattr(STDIN_FILENO, TCSANOW, &self.canonical_mode);
        }
    }

    pub fn make_raw(&self) {
        let stdin_fd = stdin().as_raw_fd();

        unsafe {
            fcntl(
                stdin_fd,
                F_SETFL,
                libc::fcntl(stdin_fd, F_GETFL) | O_NONBLOCK,
            );

            tcsetattr(STDIN_FILENO, TCSANOW, &self.raw_mode);
        }
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        self.make_canonical();
    }
}
