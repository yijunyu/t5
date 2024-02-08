#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn getchar() -> libc::c_int;
    fn rand() -> libc::c_int;
    fn srand(__seed: libc::c_uint);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;
    fn tcsetattr(
        __fd: libc::c_int,
        __optional_actions: libc::c_int,
        __termios_p: *const termios,
    ) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
}
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type time_t = __time_t;
pub type cc_t = libc::c_uchar;
pub type speed_t = libc::c_uint;
pub type tcflag_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 32],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gamestate_struct__ {
    pub grid: [[libc::c_int; 4]; 4],
    pub have_moved: libc::c_int,
    pub total_score: libc::c_long,
    pub score_last_move: libc::c_long,
    pub blocks_in_play: libc::c_int,
}
#[no_mangle]
pub static mut values: [libc::c_long; 12] = [
    0 as libc::c_int as libc::c_long,
    2 as libc::c_int as libc::c_long,
    4 as libc::c_int as libc::c_long,
    8 as libc::c_int as libc::c_long,
    16 as libc::c_int as libc::c_long,
    32 as libc::c_int as libc::c_long,
    64 as libc::c_int as libc::c_long,
    128 as libc::c_int as libc::c_long,
    256 as libc::c_int as libc::c_long,
    512 as libc::c_int as libc::c_long,
    1024 as libc::c_int as libc::c_long,
    2048 as libc::c_int as libc::c_long,
];
#[no_mangle]
pub static mut colors: [*const libc::c_char; 12] = [
    b"39\0" as *const u8 as *const libc::c_char,
    b"31\0" as *const u8 as *const libc::c_char,
    b"32\0" as *const u8 as *const libc::c_char,
    b"33\0" as *const u8 as *const libc::c_char,
    b"34\0" as *const u8 as *const libc::c_char,
    b"35\0" as *const u8 as *const libc::c_char,
    b"36\0" as *const u8 as *const libc::c_char,
    b"37\0" as *const u8 as *const libc::c_char,
    b"91\0" as *const u8 as *const libc::c_char,
    b"92\0" as *const u8 as *const libc::c_char,
    b"93\0" as *const u8 as *const libc::c_char,
    b"94\0" as *const u8 as *const libc::c_char,
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
pub unsafe extern "C" fn do_draw() {
    printf(
        b"\x1B[2J\x1B[HScore: %ld\0" as *const u8 as *const libc::c_char,
        game.total_score,
    );
    if game.score_last_move != 0 {
        printf(b" (+%ld)\0" as *const u8 as *const libc::c_char, game.score_last_move);
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 25 as libc::c_int {
        printf(b"-\0" as *const u8 as *const libc::c_char);
        i += 1;
        i;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < 4 as libc::c_int {
        printf(b"|\0" as *const u8 as *const libc::c_char);
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < 4 as libc::c_int {
            if game.grid[x as usize][y as usize] != 0 {
                printf(
                    b"\x1B[7m\x1B[%sm%*zd \x1B[0m|\0" as *const u8
                        as *const libc::c_char,
                    colors[game.grid[x as usize][y as usize] as usize],
                    4 as libc::c_int,
                    values[game.grid[x as usize][y as usize] as usize],
                );
            } else {
                printf(
                    b"%*s |\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int,
                    b"\0" as *const u8 as *const libc::c_char,
                );
            }
            x += 1;
            x;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        y += 1;
        y;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 25 as libc::c_int {
        printf(b"-\0" as *const u8 as *const libc::c_char);
        i_0 += 1;
        i_0;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn do_merge(mut d: libc::c_int) {
    game.score_last_move = 0 as libc::c_int as libc::c_long;
    match d {
        4 => {
            let mut x: libc::c_int = 0 as libc::c_int;
            while x < 3 as libc::c_int {
                let mut y: libc::c_int = 0 as libc::c_int;
                while y < 4 as libc::c_int {
                    if game.grid[x as usize][y as usize] != 0
                        && game.grid[x as usize][y as usize]
                            == game
                                .grid[(x + 1 as libc::c_int)
                                as usize][(y + 0 as libc::c_int) as usize]
                    {
                        game.have_moved = 1 as libc::c_int;
                        game.grid[x as usize][y as usize] += game.have_moved;
                        let fresh0 = game.blocks_in_play;
                        game.blocks_in_play = game.blocks_in_play - 1;
                        game
                            .grid[(x + 1 as libc::c_int)
                            as usize][(y + 0 as libc::c_int)
                            as usize] = 0 as libc::c_int * fresh0;
                        game.score_last_move
                            += values[game.grid[x as usize][y as usize] as usize];
                        game.total_score
                            += values[game.grid[x as usize][y as usize] as usize];
                    }
                    y += 1 as libc::c_int;
                }
                x += 1 as libc::c_int;
            }
        }
        3 => {
            let mut x_0: libc::c_int = 3 as libc::c_int;
            while x_0 > 0 as libc::c_int {
                let mut y_0: libc::c_int = 0 as libc::c_int;
                while y_0 < 4 as libc::c_int {
                    if game.grid[x_0 as usize][y_0 as usize] != 0
                        && game.grid[x_0 as usize][y_0 as usize]
                            == game
                                .grid[(x_0 + -(1 as libc::c_int))
                                as usize][(y_0 + 0 as libc::c_int) as usize]
                    {
                        game.have_moved = 1 as libc::c_int;
                        game.grid[x_0 as usize][y_0 as usize] += game.have_moved;
                        let fresh1 = game.blocks_in_play;
                        game.blocks_in_play = game.blocks_in_play - 1;
                        game
                            .grid[(x_0 + -(1 as libc::c_int))
                            as usize][(y_0 + 0 as libc::c_int)
                            as usize] = 0 as libc::c_int * fresh1;
                        game.score_last_move
                            += values[game.grid[x_0 as usize][y_0 as usize] as usize];
                        game.total_score
                            += values[game.grid[x_0 as usize][y_0 as usize] as usize];
                    }
                    y_0 += 1 as libc::c_int;
                }
                x_0 += -(1 as libc::c_int);
            }
        }
        2 => {
            let mut y_1: libc::c_int = 3 as libc::c_int;
            while y_1 > 0 as libc::c_int {
                let mut x_1: libc::c_int = 0 as libc::c_int;
                while x_1 < 4 as libc::c_int {
                    if game.grid[x_1 as usize][y_1 as usize] != 0
                        && game.grid[x_1 as usize][y_1 as usize]
                            == game
                                .grid[(x_1 + 0 as libc::c_int)
                                as usize][(y_1 + -(1 as libc::c_int)) as usize]
                    {
                        game.have_moved = 1 as libc::c_int;
                        game.grid[x_1 as usize][y_1 as usize] += game.have_moved;
                        let fresh2 = game.blocks_in_play;
                        game.blocks_in_play = game.blocks_in_play - 1;
                        game
                            .grid[(x_1 + 0 as libc::c_int)
                            as usize][(y_1 + -(1 as libc::c_int))
                            as usize] = 0 as libc::c_int * fresh2;
                        game.score_last_move
                            += values[game.grid[x_1 as usize][y_1 as usize] as usize];
                        game.total_score
                            += values[game.grid[x_1 as usize][y_1 as usize] as usize];
                    }
                    x_1 += 1 as libc::c_int;
                }
                y_1 += -(1 as libc::c_int);
            }
        }
        1 => {
            let mut y_2: libc::c_int = 0 as libc::c_int;
            while y_2 < 3 as libc::c_int {
                let mut x_2: libc::c_int = 0 as libc::c_int;
                while x_2 < 4 as libc::c_int {
                    if game.grid[x_2 as usize][y_2 as usize] != 0
                        && game.grid[x_2 as usize][y_2 as usize]
                            == game
                                .grid[(x_2 + 0 as libc::c_int)
                                as usize][(y_2 + 1 as libc::c_int) as usize]
                    {
                        game.have_moved = 1 as libc::c_int;
                        game.grid[x_2 as usize][y_2 as usize] += game.have_moved;
                        let fresh3 = game.blocks_in_play;
                        game.blocks_in_play = game.blocks_in_play - 1;
                        game
                            .grid[(x_2 + 0 as libc::c_int)
                            as usize][(y_2 + 1 as libc::c_int)
                            as usize] = 0 as libc::c_int * fresh3;
                        game.score_last_move
                            += values[game.grid[x_2 as usize][y_2 as usize] as usize];
                        game.total_score
                            += values[game.grid[x_2 as usize][y_2 as usize] as usize];
                    }
                    x_2 += 1 as libc::c_int;
                }
                y_2 += 1 as libc::c_int;
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn do_gravity(mut d: libc::c_int) {
    match d {
        4 => {
            let mut break_cond: libc::c_int = 0 as libc::c_int;
            while break_cond == 0 {
                break_cond = 1 as libc::c_int;
                let mut x: libc::c_int = 0 as libc::c_int;
                while x < 3 as libc::c_int {
                    let mut y: libc::c_int = 0 as libc::c_int;
                    while y < 4 as libc::c_int {
                        if game.grid[x as usize][y as usize] == 0
                            && game
                                .grid[(x + 1 as libc::c_int)
                                as usize][(y + 0 as libc::c_int) as usize] != 0
                        {
                            game
                                .grid[x
                                as usize][y
                                as usize] = game
                                .grid[(x + 1 as libc::c_int)
                                as usize][(y + 0 as libc::c_int) as usize];
                            break_cond = 0 as libc::c_int;
                            game
                                .grid[(x + 1 as libc::c_int)
                                as usize][(y + 0 as libc::c_int) as usize] = break_cond;
                            game.have_moved = 1 as libc::c_int;
                        }
                        y += 1 as libc::c_int;
                    }
                    x += 1 as libc::c_int;
                }
                do_draw();
                usleep(40000 as libc::c_int as __useconds_t);
            }
        }
        3 => {
            let mut break_cond_0: libc::c_int = 0 as libc::c_int;
            while break_cond_0 == 0 {
                break_cond_0 = 1 as libc::c_int;
                let mut x_0: libc::c_int = 3 as libc::c_int;
                while x_0 > 0 as libc::c_int {
                    let mut y_0: libc::c_int = 0 as libc::c_int;
                    while y_0 < 4 as libc::c_int {
                        if game.grid[x_0 as usize][y_0 as usize] == 0
                            && game
                                .grid[(x_0 + -(1 as libc::c_int))
                                as usize][(y_0 + 0 as libc::c_int) as usize] != 0
                        {
                            game
                                .grid[x_0
                                as usize][y_0
                                as usize] = game
                                .grid[(x_0 + -(1 as libc::c_int))
                                as usize][(y_0 + 0 as libc::c_int) as usize];
                            break_cond_0 = 0 as libc::c_int;
                            game
                                .grid[(x_0 + -(1 as libc::c_int))
                                as usize][(y_0 + 0 as libc::c_int) as usize] = break_cond_0;
                            game.have_moved = 1 as libc::c_int;
                        }
                        y_0 += 1 as libc::c_int;
                    }
                    x_0 += -(1 as libc::c_int);
                }
                do_draw();
                usleep(40000 as libc::c_int as __useconds_t);
            }
        }
        2 => {
            let mut break_cond_1: libc::c_int = 0 as libc::c_int;
            while break_cond_1 == 0 {
                break_cond_1 = 1 as libc::c_int;
                let mut y_1: libc::c_int = 3 as libc::c_int;
                while y_1 > 0 as libc::c_int {
                    let mut x_1: libc::c_int = 0 as libc::c_int;
                    while x_1 < 4 as libc::c_int {
                        if game.grid[x_1 as usize][y_1 as usize] == 0
                            && game
                                .grid[(x_1 + 0 as libc::c_int)
                                as usize][(y_1 + -(1 as libc::c_int)) as usize] != 0
                        {
                            game
                                .grid[x_1
                                as usize][y_1
                                as usize] = game
                                .grid[(x_1 + 0 as libc::c_int)
                                as usize][(y_1 + -(1 as libc::c_int)) as usize];
                            break_cond_1 = 0 as libc::c_int;
                            game
                                .grid[(x_1 + 0 as libc::c_int)
                                as usize][(y_1 + -(1 as libc::c_int))
                                as usize] = break_cond_1;
                            game.have_moved = 1 as libc::c_int;
                        }
                        x_1 += 1 as libc::c_int;
                    }
                    y_1 += -(1 as libc::c_int);
                }
                do_draw();
                usleep(40000 as libc::c_int as __useconds_t);
            }
        }
        1 => {
            let mut break_cond_2: libc::c_int = 0 as libc::c_int;
            while break_cond_2 == 0 {
                break_cond_2 = 1 as libc::c_int;
                let mut y_2: libc::c_int = 0 as libc::c_int;
                while y_2 < 3 as libc::c_int {
                    let mut x_2: libc::c_int = 0 as libc::c_int;
                    while x_2 < 4 as libc::c_int {
                        if game.grid[x_2 as usize][y_2 as usize] == 0
                            && game
                                .grid[(x_2 + 0 as libc::c_int)
                                as usize][(y_2 + 1 as libc::c_int) as usize] != 0
                        {
                            game
                                .grid[x_2
                                as usize][y_2
                                as usize] = game
                                .grid[(x_2 + 0 as libc::c_int)
                                as usize][(y_2 + 1 as libc::c_int) as usize];
                            break_cond_2 = 0 as libc::c_int;
                            game
                                .grid[(x_2 + 0 as libc::c_int)
                                as usize][(y_2 + 1 as libc::c_int) as usize] = break_cond_2;
                            game.have_moved = 1 as libc::c_int;
                        }
                        x_2 += 1 as libc::c_int;
                    }
                    y_2 += 1 as libc::c_int;
                }
                do_draw();
                usleep(40000 as libc::c_int as __useconds_t);
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn do_check_end_condition() -> libc::c_int {
    let mut ret: libc::c_int = -(1 as libc::c_int);
    let mut x: libc::c_int = 0 as libc::c_int;
    while x < 4 as libc::c_int {
        let mut y: libc::c_int = 0 as libc::c_int;
        while y < 4 as libc::c_int {
            if values[game.grid[x as usize][y as usize] as usize]
                == 2048 as libc::c_int as libc::c_long
            {
                return 1 as libc::c_int;
            }
            if game.grid[x as usize][y as usize] == 0
                || (x + 1 as libc::c_int) < 4 as libc::c_int
                    && game.grid[x as usize][y as usize]
                        == game.grid[(x + 1 as libc::c_int) as usize][y as usize]
                || (y + 1 as libc::c_int) < 4 as libc::c_int
                    && game.grid[x as usize][y as usize]
                        == game.grid[x as usize][(y + 1 as libc::c_int) as usize]
            {
                ret = 0 as libc::c_int;
            }
            y += 1;
            y;
        }
        x += 1;
        x;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn do_tick(mut d: libc::c_int) -> libc::c_int {
    game.have_moved = 0 as libc::c_int;
    do_gravity(d);
    do_merge(d);
    do_gravity(d);
    return game.have_moved;
}
#[no_mangle]
pub unsafe extern "C" fn do_newblock() {
    if game.blocks_in_play >= 16 as libc::c_int {
        return;
    }
    let mut bn: libc::c_int = rand() % (16 as libc::c_int - game.blocks_in_play);
    let mut pn: libc::c_int = 0 as libc::c_int;
    let mut x: libc::c_int = 0 as libc::c_int;
    while x < 4 as libc::c_int {
        let mut y: libc::c_int = 0 as libc::c_int;
        while y < 4 as libc::c_int {
            if !(game.grid[x as usize][y as usize] != 0) {
                if pn == bn {
                    game
                        .grid[x
                        as usize][y
                        as usize] = if rand() % 10 as libc::c_int != 0 {
                        1 as libc::c_int
                    } else {
                        2 as libc::c_int
                    };
                    game.blocks_in_play += 1 as libc::c_int;
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
unsafe fn main_0() -> libc::c_int {
    let mut current_block: u64;
    tcgetattr(0 as libc::c_int, &mut oldt);
    newt = oldt;
    newt.c_lflag &= !(0o2 as libc::c_int | 0o10 as libc::c_int) as libc::c_uint;
    tcsetattr(0 as libc::c_int, 0 as libc::c_int, &mut newt);
    srand(time(0 as *mut time_t) as libc::c_uint);
    memset(
        &mut game as *mut gamestate_struct__ as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<gamestate_struct__>() as libc::c_ulong,
    );
    do_newblock();
    do_newblock();
    do_draw();
    's_31: loop {
        let mut found_valid_key: libc::c_int = 0;
        let mut direction: libc::c_int = 0;
        let mut value: libc::c_int = 0;
        loop {
            found_valid_key = 1 as libc::c_int;
            direction = -(1 as libc::c_int);
            value = getchar();
            match value {
                104 | 97 => {
                    direction = 4 as libc::c_int;
                }
                108 | 100 => {
                    direction = 3 as libc::c_int;
                }
                106 | 115 => {
                    direction = 2 as libc::c_int;
                }
                107 | 119 => {
                    direction = 1 as libc::c_int;
                }
                113 => {
                    current_block = 1947375109854664918;
                    break 's_31;
                }
                27 => {
                    if getchar() == 91 as libc::c_int {
                        value = getchar();
                        match value {
                            65 => {
                                direction = 1 as libc::c_int;
                            }
                            66 => {
                                direction = 2 as libc::c_int;
                            }
                            67 => {
                                direction = 3 as libc::c_int;
                            }
                            68 => {
                                direction = 4 as libc::c_int;
                            }
                            _ => {
                                found_valid_key = 0 as libc::c_int;
                            }
                        }
                    }
                }
                _ => {
                    found_valid_key = 0 as libc::c_int;
                }
            }
            if !(found_valid_key == 0) {
                break;
            }
        }
        do_tick(direction);
        if game.have_moved != 0 as libc::c_int {
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
    match current_block {
        267910298023665031 => {
            printf(b"You lose!\n\0" as *const u8 as *const libc::c_char);
        }
        340875295666714687 => {
            printf(b"You win!\n\0" as *const u8 as *const libc::c_char);
        }
        _ => {}
    }
    tcsetattr(0 as libc::c_int, 0 as libc::c_int, &mut oldt);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
