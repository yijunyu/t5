#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use std::io::Read;
fn rust_getchar() -> u8 {
    return std::io::stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte)
        .unwrap();
}

fn build_str_from_raw_ptr(raw_ptr: *mut u8) -> String {
    unsafe {
        let mut str_size: usize = 0;
        while *raw_ptr.offset(str_size as isize) != 0 {
            str_size += 1;
        }
        return std::str::from_utf8_unchecked(std::slice::from_raw_parts(raw_ptr, str_size))
            .to_owned();
    }
}

use std::time::SystemTime;
pub fn rust_time(ref_result: Option<&mut i64>) -> i64 {
    let result = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    };
    match ref_result {
        Some(r) => *r = result,
        None => {}
    }
    return result as i64;
}

use c2rust_out::*;
extern "C" {
    fn rand() -> i32;
    fn srand(__seed: u32);
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn tcgetattr(__fd: i32, __termios_p: *mut termios) -> i32;
    fn tcsetattr(__fd: i32, __optional_actions: i32, __termios_p: *const termios) -> i32;
    fn usleep(__useconds: u32) -> i32;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct termios {
    pub c_iflag: u32,
    pub c_oflag: u32,
    pub c_cflag: u32,
    pub c_lflag: u32,
    pub c_line: u8,
    pub c_cc: [u8; 32],
    pub c_ispeed: u32,
    pub c_ospeed: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gamestate_struct__ {
    pub grid: [[i32; 4]; 4],
    pub have_moved: i32,
    pub total_score: i64,
    pub score_last_move: i64,
    pub blocks_in_play: i32,
}
#[no_mangle]
pub static mut values: [i64; 12] = [0, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048];
#[no_mangle]
pub static mut colors: [*const i8; 12] = [
    b"39\0" as *const u8 as *const i8,
    b"31\0" as *const u8 as *const i8,
    b"32\0" as *const u8 as *const i8,
    b"33\0" as *const u8 as *const i8,
    b"34\0" as *const u8 as *const i8,
    b"35\0" as *const u8 as *const i8,
    b"36\0" as *const u8 as *const i8,
    b"37\0" as *const u8 as *const i8,
    b"91\0" as *const u8 as *const i8,
    b"92\0" as *const u8 as *const i8,
    b"93\0" as *const u8 as *const i8,
    b"94\0" as *const u8 as *const i8,
];
#[no_mangle]
pub static mut game: gamestate_struct__ = gamestate_struct__ {
    grid: [[0; 4]; 4],
    have_moved: 0,
    total_score: 0,
    score_last_move: 0,
    blocks_in_play: 0,
};
#[no_mangle]
pub static mut oldt: termios = termios {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; 32],
    c_ispeed: 0,
    c_ospeed: 0,
};
#[no_mangle]
pub static mut newt: termios = termios {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; 32],
    c_ispeed: 0,
    c_ospeed: 0,
};
#[no_mangle]
pub extern "C" fn do_draw() {
    unsafe {
        print!("\x1B[2J\x1B[HScore: {}", game.total_score);
        if game.score_last_move != 0 {
            print!(" (+{})", game.score_last_move);
        }
    }
    print!("\n");
    let mut i: i32 = 0;
    while i < 25 {
        print!("-");
        i += 1;
        i;
    }
    print!("\n");
    let mut y: i32 = 0;
    unsafe {
        while y < 4 {
            print!("|");
            let mut x: i32 = 0;
            while x < 4 {
                if game.grid[x as usize][y as usize] != 0 {
                    print!(
                        "\x1B[7m\x1B[{0:}m{2:1$} \x1B[0m|",
                        build_str_from_raw_ptr(
                            colors[game.grid[x as usize][y as usize] as usize] as *mut u8
                        ),
                        4,
                        values[game.grid[x as usize][y as usize] as usize]
                    );
                } else {
                    print!("{1:0$} |", 4, "\0");
                }
                x += 1;
                x;
            }
            print!("\n");
            y += 1;
            y;
        }
    }
    let mut i_0: i32 = 0;
    while i_0 < 25 {
        print!("-");
        i_0 += 1;
        i_0;
    }
    print!("\n");
}

#[no_mangle]
pub extern "C" fn do_merge(mut d: i32) {
    unsafe {
        game.score_last_move = 0;
        match d {
            4 => {
                let mut x: i32 = 0;
                while x < 3 {
                    let mut y: i32 = 0;
                    while y < 4 {
                        if game.grid[x as usize][y as usize] != 0
                            && game.grid[x as usize][y as usize]
                                == game.grid[(x + 1i32) as usize][(y + 0i32) as usize]
                        {
                            game.have_moved = 1;
                            game.grid[x as usize][y as usize] += game.have_moved;
                            let fresh0 = game.blocks_in_play;
                            game.blocks_in_play = game.blocks_in_play - 1;
                            game.grid[(x + 1i32) as usize][(y + 0i32) as usize] = 0 * fresh0;
                            game.score_last_move +=
                                values[game.grid[x as usize][y as usize] as usize];
                            game.total_score += values[game.grid[x as usize][y as usize] as usize];
                        }
                        y += 1;
                    }
                    x += 1;
                }
            }
            3 => {
                let mut x_0: i32 = 3;
                while x_0 > 0 {
                    let mut y_0: i32 = 0;
                    while y_0 < 4 {
                        if game.grid[x_0 as usize][y_0 as usize] != 0
                            && game.grid[x_0 as usize][y_0 as usize]
                                == game.grid[(x_0 + -1i32) as usize][(y_0 + 0i32) as usize]
                        {
                            game.have_moved = 1;
                            game.grid[x_0 as usize][y_0 as usize] += game.have_moved;
                            let fresh1 = game.blocks_in_play;
                            game.blocks_in_play = game.blocks_in_play - 1;
                            game.grid[(x_0 + -1i32) as usize][(y_0 + 0i32) as usize] = 0 * fresh1;
                            game.score_last_move +=
                                values[game.grid[x_0 as usize][y_0 as usize] as usize];
                            game.total_score +=
                                values[game.grid[x_0 as usize][y_0 as usize] as usize];
                        }
                        y_0 += 1;
                    }
                    x_0 += -1;
                }
            }
            2 => {
                let mut y_1: i32 = 3;
                while y_1 > 0 {
                    let mut x_1: i32 = 0;
                    while x_1 < 4 {
                        if game.grid[x_1 as usize][y_1 as usize] != 0
                            && game.grid[x_1 as usize][y_1 as usize]
                                == game.grid[(x_1 + 0i32) as usize][(y_1 + -1i32) as usize]
                        {
                            game.have_moved = 1;
                            game.grid[x_1 as usize][y_1 as usize] += game.have_moved;
                            let fresh2 = game.blocks_in_play;
                            game.blocks_in_play = game.blocks_in_play - 1;
                            game.grid[(x_1 + 0i32) as usize][(y_1 + -1i32) as usize] = 0 * fresh2;
                            game.score_last_move +=
                                values[game.grid[x_1 as usize][y_1 as usize] as usize];
                            game.total_score +=
                                values[game.grid[x_1 as usize][y_1 as usize] as usize];
                        }
                        x_1 += 1;
                    }
                    y_1 += -1;
                }
            }
            1 => {
                let mut y_2: i32 = 0;
                while y_2 < 3 {
                    let mut x_2: i32 = 0;
                    while x_2 < 4 {
                        if game.grid[x_2 as usize][y_2 as usize] != 0
                            && game.grid[x_2 as usize][y_2 as usize]
                                == game.grid[(x_2 + 0i32) as usize][(y_2 + 1i32) as usize]
                        {
                            game.have_moved = 1;
                            game.grid[x_2 as usize][y_2 as usize] += game.have_moved;
                            let fresh3 = game.blocks_in_play;
                            game.blocks_in_play = game.blocks_in_play - 1;
                            game.grid[(x_2 + 0i32) as usize][(y_2 + 1i32) as usize] = 0 * fresh3;
                            game.score_last_move +=
                                values[game.grid[x_2 as usize][y_2 as usize] as usize];
                            game.total_score +=
                                values[game.grid[x_2 as usize][y_2 as usize] as usize];
                        }
                        x_2 += 1;
                    }
                    y_2 += 1;
                }
            }
            _ => {}
        };
    }
}

#[no_mangle]
pub extern "C" fn do_gravity(mut d: i32) {
    unsafe {
        match d {
            4 => {
                let mut break_cond: i32 = 0;
                while break_cond == 0 {
                    break_cond = 1;
                    let mut x: i32 = 0;
                    while x < 3 {
                        let mut y: i32 = 0;
                        while y < 4 {
                            if game.grid[x as usize][y as usize] == 0
                                && game.grid[(x + 1i32) as usize][(y + 0i32) as usize] != 0
                            {
                                game.grid[x as usize][y as usize] =
                                    game.grid[(x + 1i32) as usize][(y + 0i32) as usize];
                                break_cond = 0;
                                game.grid[(x + 1i32) as usize][(y + 0i32) as usize] = break_cond;
                                game.have_moved = 1;
                            }
                            y += 1;
                        }
                        x += 1;
                    }
                    do_draw();
                    usleep(40000);
                }
            }
            3 => {
                let mut break_cond_0: i32 = 0;
                while break_cond_0 == 0 {
                    break_cond_0 = 1;
                    let mut x_0: i32 = 3;
                    while x_0 > 0 {
                        let mut y_0: i32 = 0;
                        while y_0 < 4 {
                            if game.grid[x_0 as usize][y_0 as usize] == 0
                                && game.grid[(x_0 + -1i32) as usize][(y_0 + 0i32) as usize] != 0
                            {
                                game.grid[x_0 as usize][y_0 as usize] =
                                    game.grid[(x_0 + -1i32) as usize][(y_0 + 0i32) as usize];
                                break_cond_0 = 0;
                                game.grid[(x_0 + -1i32) as usize][(y_0 + 0i32) as usize] =
                                    break_cond_0;
                                game.have_moved = 1;
                            }
                            y_0 += 1;
                        }
                        x_0 += -1;
                    }
                    do_draw();
                    usleep(40000);
                }
            }
            2 => {
                let mut break_cond_1: i32 = 0;
                while break_cond_1 == 0 {
                    break_cond_1 = 1;
                    let mut y_1: i32 = 3;
                    while y_1 > 0 {
                        let mut x_1: i32 = 0;
                        while x_1 < 4 {
                            if game.grid[x_1 as usize][y_1 as usize] == 0
                                && game.grid[(x_1 + 0i32) as usize][(y_1 + -1i32) as usize] != 0
                            {
                                game.grid[x_1 as usize][y_1 as usize] =
                                    game.grid[(x_1 + 0i32) as usize][(y_1 + -1i32) as usize];
                                break_cond_1 = 0;
                                game.grid[(x_1 + 0i32) as usize][(y_1 + -1i32) as usize] =
                                    break_cond_1;
                                game.have_moved = 1;
                            }
                            x_1 += 1;
                        }
                        y_1 += -1;
                    }
                    do_draw();
                    usleep(40000);
                }
            }
            1 => {
                let mut break_cond_2: i32 = 0;
                while break_cond_2 == 0 {
                    break_cond_2 = 1;
                    let mut y_2: i32 = 0;
                    while y_2 < 3 {
                        let mut x_2: i32 = 0;
                        while x_2 < 4 {
                            if game.grid[x_2 as usize][y_2 as usize] == 0
                                && game.grid[(x_2 + 0i32) as usize][(y_2 + 1i32) as usize] != 0
                            {
                                game.grid[x_2 as usize][y_2 as usize] =
                                    game.grid[(x_2 + 0i32) as usize][(y_2 + 1i32) as usize];
                                break_cond_2 = 0;
                                game.grid[(x_2 + 0i32) as usize][(y_2 + 1i32) as usize] =
                                    break_cond_2;
                                game.have_moved = 1;
                            }
                            x_2 += 1;
                        }
                        y_2 += 1;
                    }
                    do_draw();
                    usleep(40000);
                }
            }
            _ => {}
        };
    }
}

#[no_mangle]
pub extern "C" fn do_check_end_condition() -> i32 {
    let mut ret: i32 = -1;
    let mut x: i32 = 0;
    unsafe {
        while x < 4 {
            let mut y: i32 = 0;
            while y < 4 {
                if values[game.grid[x as usize][y as usize] as usize] == 2048 {
                    return 1;
                }
                if game.grid[x as usize][y as usize] == 0
                    || (x + 1) < 4
                        && game.grid[x as usize][y as usize]
                            == game.grid[(x + 1i32) as usize][y as usize]
                    || (y + 1) < 4
                        && game.grid[x as usize][y as usize]
                            == game.grid[x as usize][(y + 1i32) as usize]
                {
                    ret = 0;
                }
                y += 1;
                y;
            }
            x += 1;
            x;
        }
    }
    return ret;
}

#[no_mangle]
pub extern "C" fn do_tick(mut d: i32) -> i32 {
    unsafe {
        game.have_moved = 0;
    }
    do_gravity(d);
    do_merge(d);
    do_gravity(d);
    unsafe {
        return game.have_moved;
    }
}

#[no_mangle]
pub extern "C" fn do_newblock() {
    unsafe {
        if game.blocks_in_play >= 16 {
            return;
        }
        let mut bn: i32 = rand() % (16 - game.blocks_in_play);
        let mut pn: i32 = 0;
        let mut x: i32 = 0;
        while x < 4 {
            let mut y: i32 = 0;
            while y < 4 {
                if !(game.grid[x as usize][y as usize] != 0) {
                    if pn == bn {
                        game.grid[x as usize][y as usize] = if rand() % 10 != 0 { 1 } else { 2 };
                        game.blocks_in_play += 1;
                        return;
                    } else {
                        pn += 1;
                        pn;
                    }
                }
                y += 1;
                y;
            }
            x += 1;
            x;
        }
    }
}

fn main_0() -> i32 {
    let mut current_block: u64;
    unsafe {
        tcgetattr(0, &mut oldt);
        newt = oldt;
        newt.c_lflag &= !(0o2 | 0o10i32) as u32;
        tcsetattr(0, 0, &mut newt);
        srand(rust_time(None) as u32);
        memset(
            &mut game as *mut gamestate_struct__ as *mut libc::c_void,
            0,
            ::core::mem::size_of::<gamestate_struct__>() as u64,
        );
    }
    do_newblock();
    do_newblock();
    do_draw();
    unsafe {
        's_31: loop {
            let mut found_valid_key: i32 = 0;
            let mut direction: i32 = 0;
            let mut value: i32 = 0;
            loop {
                found_valid_key = 1;
                direction = -1;
                value = rust_getchar() as i32;
                match value {
                    104 | 97 => {
                        direction = 4;
                    }
                    108 | 100 => {
                        direction = 3;
                    }
                    106 | 115 => {
                        direction = 2;
                    }
                    107 | 119 => {
                        direction = 1;
                    }
                    113 => {
                        current_block = 1947375109854664918;
                        break 's_31;
                    }
                    27 => {
                        if rust_getchar() as i32 == 91 {
                            value = rust_getchar() as i32;
                            match value {
                                65 => {
                                    direction = 1;
                                }
                                66 => {
                                    direction = 2;
                                }
                                67 => {
                                    direction = 3;
                                }
                                68 => {
                                    direction = 4;
                                }
                                _ => {
                                    found_valid_key = 0;
                                }
                            }
                        }
                    }
                    _ => {
                        found_valid_key = 0;
                    }
                }
                if !(found_valid_key == 0) {
                    break;
                }
            }
            do_tick(direction);
            if game.have_moved != 0 {
                do_newblock();
            }
            do_draw();
            match do_check_end_condition() {
                -1 => {
                    current_block = 267910298023665031;
                    break;
                }
                1 => {
                    current_block = 340875295666714687;
                    break;
                }
                0 | _ => {}
            }
        }
    }
    match current_block {
        267910298023665031 => {
            print!("You lose!\n");
        }
        340875295666714687 => {
            print!("You win!\n");
        }
        _ => {}
    }
    unsafe {
        tcsetattr(0, 0, &mut oldt);
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
