use libc::*;
use std::io::{stdin, Read};
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

pub fn getch() -> Option<char> {
    let mut buf = [0; 1];

    match stdin().read(&mut buf) {
        Ok(0) | Err(_) => None,
        Ok(_) => Some(buf[0] as char),
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
