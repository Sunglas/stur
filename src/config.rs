#![allow(dead_code)]
/* See LICENSE file for copyright and license details. */

/*
 * appearance
 *
 * font: see http://freedesktop.org/software.org/software/fontconfig/fontconfig-user.html
 * */
static FONT: &str = "Liberation Mono:pixelsize=12:antialias=true:autohint=true";
static BORDERPX: i32 = 2;

/*
 * What program is execed by stur depends of these precedence rules:
 * 1: program passed with -e
 * 2: scroll and/or utmp
 * 3: SHELL environment variable
 * 4: value of shell in /etc/passwd
 * 5: value of shell in config.h
 */
static SHELL: &str = "/bin/sh";
const UTMP: &str = "";
/* scroll program: to enable usa string like "scroll" */
const SCROLL: &str = "";
const STTY_ARGS: &str = "stty raw pass8 nl -echo -iexten -cstopb 38400";

/* identification sequence returned in DA and DECID */
const VTIDEN: &str = "\x33[?6c";

/* Kerning / character bounding-box multipliers */
static CWSCALE: f32 = 1.0;
static CHSCALE: f32 = 1.0;

/*
 * word delimiter string
 *
 * More advanced example: wch!(" `'\"()[]{}")
 */ // TODO: type wchar_t = ???
// const WORDDELIMITERS: wchar_t = wch!(" ");

/* selection timeoutst (in milliseconds) */
static DOUBLECLICKTIMEOUT: u32 = 300;
static TRIPLECLICKTIMEOUT: u32 = 600;

/* alt screens */
const ALLOWALTSCREEN: i32 = 1;

/* allow certain non-interactive (insecure) window operations such as:
 * setting the clipborad text */
const ALLOWWINDOWOPS: i32 = 0;

/*
 * draw latency range in ms - from new content/keypress/etc until drawing.
 * within this range, stur draws when content stops arriving (idle). mostly
 * it's near minlatency, but it waits longer for slow updates to avoid partial
 * draw. low minlatency will tear/flicker more more, as it can "detect" idle too early.
 */
static MINLATENCY: f64 = 8.0;
static MAXLATENCY: f64 = 33.0;

/*
 * blinking timeout (set to 0 to disable blinking) for the terminal blinking
 * attribute.
 */
static BLINKTIMEOUT: u32 = 800;

/*
 * thickness of underline and bar cursors
 */
static CURSORTHICKNESS: u32 = 2;

/* bell volume. It must be a value between -100 and 100. Use 0 for disabling
 * it
 */
static BELLVOLUME: i32 = 0;

/* default TERM value */
const TERMNAME: &str = "st-256color";

/*
 * spaces per tab
 *
 * When you are changing this value, don't forget to adapt the `it` value in the
 * stur.info and appropriathely install the stur.info in the environment where
 * you use this stur version.
 *
 *      it#$tabspaces,
 *
 * Secondly make sure your kernel is not expanding tabs. When running `stty -a`
 * *tab0* should appear. You can tell the terminal to not expand tabs by
 * running the following command:
 *
 *      stty tabs
 *
 */
const TABSPACES: u32 = 8;

/* Terminal colors (16 first used in escape sequence) */
static COLORNAME: [&str; 20] = [
    /* 8 normal colors */
    "black",
    "red3",
    "green3",
    "yellow3",
    "blue2",
    "magenta3",
    "cyan3",
    "gray90",

    /* 8 bright colors */
    "gray50",
    "red",
    "green",
    "yellow",
    "#5C5CFF",
    "magenta",
    "cyan",
    "white",

    // TODO: does anyone need more than 255 colors ???
    //[255] = 0,

    /* more colors can be added after 255 to use with DefaultXX */
    "#CCCCCC",
    "#555555",
    "gray90", /* default foreground colour */
    "black", /* default background colour */
];

/*
 * Default colors (colorname index)
 * foreground, background, cursor, reverse cursor
 */
const DEFAULTFG: u32 = 258;
const DEFAULTBG: u32 = 259;
const DEFAULTCS: u32 = 256;
static DEFAULTRCS: u32 = 257;

/*
 * Default shape of cursor
 * 2: Block ("█")
 * 4: Underline ("_")
 * 6: Bar ("|")
 * 7: Snowman ("☃")
 */
static CURSORSHAPE: u32 = 2;

/*
 * Default columns and rows numbers
 */

static COLS: u32 = 80;
static ROWS: u32 = 24;

/*
 * Default colour and shape of the mouse cursor
 */ // TODO: type XC_xterm = ???
// static MOUSESHAPE: u32 = XC_xterm;
static MOUSEFG: u32 = 7;
static MOUSEBG: u32 = 0;

/*
 * Color used to display font attributes when fontconfig selected a font which
 * doesn't match the ones requested.
 */
static DEFAULTATTR: u32 = 11;

/*
 * Force mouse select/shortcuts while mask is active (when MODE_MOUSE is set).
 * Note that if you want to use ShiftMask with selmasks, set this to an other
 * modifier, set to 0 to not use it.
 */
static FORCEMOUSEMOD: u32 = x11::xlib::ShiftMask;

/*
 * Internal mouse shortcuts.
 * Beware that overloading Button1 will disable the selection.
 */
// static MSHORTCUTS: MouseShortcut[] = { // TODO: type MouseShortcut = ???
// 	/* mask                 button   function        argument       release */
// 	{ XK_ANY_MOD,           Button2, selpaste,       {.i = 0},      1 },
// 	{ ShiftMask,            Button4, ttysend,        {.s = "\033[5;2~"} },
// 	{ XK_ANY_MOD,           Button4, ttysend,        {.s = "\031"} },
// 	{ ShiftMask,            Button5, ttysend,        {.s = "\033[6;2~"} },
// 	{ XK_ANY_MOD,           Button5, ttysend,        {.s = "\005"} },
// };
// 
// /* Internal keyboard shortcuts. */
// const MODKEY = Mod1Mask;
// const TERMMOD = (ControlMask|ShiftMask);
// 
// static SHORTCUTS: Shortcut[] = { // TODO: type Shortcut = ???
// 	/* mask                 keysym          function        argument */
// 	{ XK_ANY_MOD,           XK_Break,       sendbreak,      {.i =  0} },
// 	{ ControlMask,          XK_Print,       toggleprinter,  {.i =  0} },
// 	{ ShiftMask,            XK_Print,       printscreen,    {.i =  0} },
// 	{ XK_ANY_MOD,           XK_Print,       printsel,       {.i =  0} },
// 	{ TERMMOD,              XK_Prior,       zoom,           {.f = +1} },
// 	{ TERMMOD,              XK_Next,        zoom,           {.f = -1} },
// 	{ TERMMOD,              XK_Home,        zoomreset,      {.f =  0} },
// 	{ TERMMOD,              XK_C,           clipcopy,       {.i =  0} },
// 	{ TERMMOD,              XK_V,           clippaste,      {.i =  0} },
// 	{ TERMMOD,              XK_Y,           selpaste,       {.i =  0} },
// 	{ ShiftMask,            XK_Insert,      selpaste,       {.i =  0} },
// 	{ TERMMOD,              XK_Num_Lock,    numlock,        {.i =  0} },
// };

/*
 * Special keys (change & recompile stur.info accordingly)
 *
 * Mask value:
 * * Use XK_ANY_MOD to match the key no matter modifiers state
 * * Use XK_NO_MOD to match the key alone (no modifiers)
 * appkey value:
 * * 0: no value
 * * > 0: keypad application mode enabled
 * *   = 2: term.numlock = 1
 * * < 0: keypad application mode disabled
 * appcursor value:
 * * 0: no value
 * * > 0: cursor application mode enabled
 * * < 0: cursor application mode disabled
 *
 * Be careful with the order of the definitions because st searches in
 * this table sequentially, so any XK_ANY_MOD must be in the last
 * position for a key.
 */

/*
 * If you want keys other than the X11 function keys (0xFD00 - 0xFFFF)
 * to be mapped below, add them to this array.
 */
// static MAPPEDKEYS: KeySym[] = { -1 }; // TODO: type KeySym

/*
 * State bits to ignore when matching key or button events.  By default,
 * numlock (Mod2Mask) and keyboard layout (XK_SWITCH_MOD) are ignored.
 */ // TODO: type XK_SWITCH_MOD = ???
static IGNOREMOD: u32 = x11::xlib::Mod2Mask /*| XK_SWITCH_MOD*/;

/*
 * This is the huge key array which defines all compatibility to the Linux
 * world. Please decide about changes wisely.
 */
// static KEY: Key[] = { // TODO: type Key = ???
// 	/* keysym           mask            string      appkey appcursor */
// 	{ XK_KP_Home,       ShiftMask,      "\x33[2J",       0,   -1},
// 	{ XK_KP_Home,       ShiftMask,      "\x33[1;2H",     0,   +1},
// 	{ XK_KP_Home,       XK_ANY_MOD,     "\x33[H",        0,   -1},
// 	{ XK_KP_Home,       XK_ANY_MOD,     "\x33[1~",       0,   +1},
// 	{ XK_KP_Up,         XK_ANY_MOD,     "\x33Ox",       +1,    0},
// 	{ XK_KP_Up,         XK_ANY_MOD,     "\x33[A",        0,   -1},
// 	{ XK_KP_Up,         XK_ANY_MOD,     "\x33OA",        0,   +1},
// 	{ XK_KP_Down,       XK_ANY_MOD,     "\x33Or",       +1,    0},
// 	{ XK_KP_Down,       XK_ANY_MOD,     "\x33[B",        0,   -1},
// 	{ XK_KP_Down,       XK_ANY_MOD,     "\x33OB",        0,   +1},
// 	{ XK_KP_Left,       XK_ANY_MOD,     "\x33Ot",       +1,    0},
// 	{ XK_KP_Left,       XK_ANY_MOD,     "\x33[D",        0,   -1},
// 	{ XK_KP_Left,       XK_ANY_MOD,     "\x33OD",        0,   +1},
// 	{ XK_KP_Right,      XK_ANY_MOD,     "\x33Ov",       +1,    0},
// 	{ XK_KP_Right,      XK_ANY_MOD,     "\x33[C",        0,   -1},
// 	{ XK_KP_Right,      XK_ANY_MOD,     "\x33OC",        0,   +1},
// 	{ XK_KP_Prior,      ShiftMask,      "\x33[5;2~",     0,    0},
// 	{ XK_KP_Prior,      XK_ANY_MOD,     "\x33[5~",       0,    0},
// 	{ XK_KP_Begin,      XK_ANY_MOD,     "\x33[E",        0,    0},
// 	{ XK_KP_End,        ControlMask,    "\x33[J",       -1,    0},
// 	{ XK_KP_End,        ControlMask,    "\x33[1;5F",    +1,    0},
// 	{ XK_KP_End,        ShiftMask,      "\x33[K",       -1,    0},
// 	{ XK_KP_End,        ShiftMask,      "\x33[1;2F",    +1,    0},
// 	{ XK_KP_End,        XK_ANY_MOD,     "\x33[4~",       0,    0},
// 	{ XK_KP_Next,       ShiftMask,      "\x33[6;2~",     0,    0},
// 	{ XK_KP_Next,       XK_ANY_MOD,     "\x33[6~",       0,    0},
// 	{ XK_KP_Insert,     ShiftMask,      "\x33[2;2~",    +1,    0},
// 	{ XK_KP_Insert,     ShiftMask,      "\x33[4l",      -1,    0},
// 	{ XK_KP_Insert,     ControlMask,    "\x33[L",       -1,    0},
// 	{ XK_KP_Insert,     ControlMask,    "\x33[2;5~",    +1,    0},
// 	{ XK_KP_Insert,     XK_ANY_MOD,     "\x33[4h",      -1,    0},
// 	{ XK_KP_Insert,     XK_ANY_MOD,     "\x33[2~",      +1,    0},
// 	{ XK_KP_Delete,     ControlMask,    "\x33[M",       -1,    0},
// 	{ XK_KP_Delete,     ControlMask,    "\x33[3;5~",    +1,    0},
// 	{ XK_KP_Delete,     ShiftMask,      "\x33[2K",      -1,    0},
// 	{ XK_KP_Delete,     ShiftMask,      "\x33[3;2~",    +1,    0},
// 	{ XK_KP_Delete,     XK_ANY_MOD,     "\x33[P",       -1,    0},
// 	{ XK_KP_Delete,     XK_ANY_MOD,     "\x33[3~",      +1,    0},
// 	{ XK_KP_Multiply,   XK_ANY_MOD,     "\x33Oj",       +2,    0},
// 	{ XK_KP_Add,        XK_ANY_MOD,     "\x33Ok",       +2,    0},
// 	{ XK_KP_Enter,      XK_ANY_MOD,     "\x33OM",       +2,    0},
// 	{ XK_KP_Enter,      XK_ANY_MOD,     "\r",           -1,    0},
// 	{ XK_KP_Subtract,   XK_ANY_MOD,     "\x33Om",       +2,    0},
// 	{ XK_KP_Decimal,    XK_ANY_MOD,     "\x33On",       +2,    0},
// 	{ XK_KP_Divide,     XK_ANY_MOD,     "\x33Oo",       +2,    0},
// 	{ XK_KP_0,          XK_ANY_MOD,     "\x33Op",       +2,    0},
// 	{ XK_KP_1,          XK_ANY_MOD,     "\x33Oq",       +2,    0},
// 	{ XK_KP_2,          XK_ANY_MOD,     "\x33Or",       +2,    0},
// 	{ XK_KP_3,          XK_ANY_MOD,     "\x33Os",       +2,    0},
// 	{ XK_KP_4,          XK_ANY_MOD,     "\x33Ot",       +2,    0},
// 	{ XK_KP_5,          XK_ANY_MOD,     "\x33Ou",       +2,    0},
// 	{ XK_KP_6,          XK_ANY_MOD,     "\x33Ov",       +2,    0},
// 	{ XK_KP_7,          XK_ANY_MOD,     "\x33Ow",       +2,    0},
// 	{ XK_KP_8,          XK_ANY_MOD,     "\x33Ox",       +2,    0},
// 	{ XK_KP_9,          XK_ANY_MOD,     "\x33Oy",       +2,    0},
// 	{ XK_Up,            ShiftMask,      "\x33[1;2A",     0,    0},
// 	{ XK_Up,            Mod1Mask,       "\x33[1;3A",     0,    0},
// 	{ XK_Up,         ShiftMask|Mod1Mask,"\x33[1;4A",     0,    0},
// 	{ XK_Up,            ControlMask,    "\x33[1;5A",     0,    0},
// 	{ XK_Up,      ShiftMask|ControlMask,"\x33[1;6A",     0,    0},
// 	{ XK_Up,       ControlMask|Mod1Mask,"\x33[1;7A",     0,    0},
// 	{ XK_Up,ShiftMask|ControlMask|Mod1Mask,"\x33[1;8A",  0,    0},
// 	{ XK_Up,            XK_ANY_MOD,     "\x33[A",        0,   -1},
// 	{ XK_Up,            XK_ANY_MOD,     "\x33OA",        0,   +1},
// 	{ XK_Down,          ShiftMask,      "\x33[1;2B",     0,    0},
// 	{ XK_Down,          Mod1Mask,       "\x33[1;3B",     0,    0},
// 	{ XK_Down,       ShiftMask|Mod1Mask,"\x33[1;4B",     0,    0},
// 	{ XK_Down,          ControlMask,    "\x33[1;5B",     0,    0},
// 	{ XK_Down,    ShiftMask|ControlMask,"\x33[1;6B",     0,    0},
// 	{ XK_Down,     ControlMask|Mod1Mask,"\x33[1;7B",     0,    0},
// 	{ XK_Down,ShiftMask|ControlMask|Mod1Mask,"\x33[1;8B",0,    0},
// 	{ XK_Down,          XK_ANY_MOD,     "\x33[B",        0,   -1},
// 	{ XK_Down,          XK_ANY_MOD,     "\x33OB",        0,   +1},
// 	{ XK_Left,          ShiftMask,      "\x33[1;2D",     0,    0},
// 	{ XK_Left,          Mod1Mask,       "\x33[1;3D",     0,    0},
// 	{ XK_Left,       ShiftMask|Mod1Mask,"\x33[1;4D",     0,    0},
// 	{ XK_Left,          ControlMask,    "\x33[1;5D",     0,    0},
// 	{ XK_Left,    ShiftMask|ControlMask,"\x33[1;6D",     0,    0},
// 	{ XK_Left,     ControlMask|Mod1Mask,"\x33[1;7D",     0,    0},
// 	{ XK_Left,ShiftMask|ControlMask|Mod1Mask,"\x33[1;8D",0,    0},
// 	{ XK_Left,          XK_ANY_MOD,     "\x33[D",        0,   -1},
// 	{ XK_Left,          XK_ANY_MOD,     "\x33OD",        0,   +1},
// 	{ XK_Right,         ShiftMask,      "\x33[1;2C",     0,    0},
// 	{ XK_Right,         Mod1Mask,       "\x33[1;3C",     0,    0},
// 	{ XK_Right,      ShiftMask|Mod1Mask,"\x33[1;4C",     0,    0},
// 	{ XK_Right,         ControlMask,    "\x33[1;5C",     0,    0},
// 	{ XK_Right,   ShiftMask|ControlMask,"\x33[1;6C",     0,    0},
// 	{ XK_Right,    ControlMask|Mod1Mask,"\x33[1;7C",     0,    0},
// 	{ XK_Right,ShiftMask|ControlMask|Mod1Mask,"\x33[1;8C",0,   0},
// 	{ XK_Right,         XK_ANY_MOD,     "\x33[C",        0,   -1},
// 	{ XK_Right,         XK_ANY_MOD,     "\x33OC",        0,   +1},
// 	{ XK_ISO_Left_Tab,  ShiftMask,      "\x33[Z",        0,    0},
// 	{ XK_Return,        Mod1Mask,       "\x33\r",        0,    0},
// 	{ XK_Return,        XK_ANY_MOD,     "\r",            0,    0},
// 	{ XK_Insert,        ShiftMask,      "\x33[4l",      -1,    0},
// 	{ XK_Insert,        ShiftMask,      "\x33[2;2~",    +1,    0},
// 	{ XK_Insert,        ControlMask,    "\x33[L",       -1,    0},
// 	{ XK_Insert,        ControlMask,    "\x33[2;5~",    +1,    0},
// 	{ XK_Insert,        XK_ANY_MOD,     "\x33[4h",      -1,    0},
// 	{ XK_Insert,        XK_ANY_MOD,     "\x33[2~",      +1,    0},
// 	{ XK_Delete,        ControlMask,    "\x33[M",       -1,    0},
// 	{ XK_Delete,        ControlMask,    "\x33[3;5~",    +1,    0},
// 	{ XK_Delete,        ShiftMask,      "\x33[2K",      -1,    0},
// 	{ XK_Delete,        ShiftMask,      "\x33[3;2~",    +1,    0},
// 	{ XK_Delete,        XK_ANY_MOD,     "\x33[P",       -1,    0},
// 	{ XK_Delete,        XK_ANY_MOD,     "\x33[3~",      +1,    0},
// 	{ XK_BackSpace,     XK_NO_MOD,      "\x177",          0,    0},
// 	{ XK_BackSpace,     Mod1Mask,       "\x33\x177",      0,    0},
// 	{ XK_Home,          ShiftMask,      "\x33[2J",       0,   -1},
// 	{ XK_Home,          ShiftMask,      "\x33[1;2H",     0,   +1},
// 	{ XK_Home,          XK_ANY_MOD,     "\x33[H",        0,   -1},
// 	{ XK_Home,          XK_ANY_MOD,     "\x33[1~",       0,   +1},
// 	{ XK_End,           ControlMask,    "\x33[J",       -1,    0},
// 	{ XK_End,           ControlMask,    "\x33[1;5F",    +1,    0},
// 	{ XK_End,           ShiftMask,      "\x33[K",       -1,    0},
// 	{ XK_End,           ShiftMask,      "\x33[1;2F",    +1,    0},
// 	{ XK_End,           XK_ANY_MOD,     "\x33[4~",       0,    0},
// 	{ XK_Prior,         ControlMask,    "\x33[5;5~",     0,    0},
// 	{ XK_Prior,         ShiftMask,      "\x33[5;2~",     0,    0},
// 	{ XK_Prior,         XK_ANY_MOD,     "\x33[5~",       0,    0},
// 	{ XK_Next,          ControlMask,    "\x33[6;5~",     0,    0},
// 	{ XK_Next,          ShiftMask,      "\x33[6;2~",     0,    0},
// 	{ XK_Next,          XK_ANY_MOD,     "\x33[6~",       0,    0},
// 	{ XK_F1,            XK_NO_MOD,      "\x33OP" ,       0,    0},
// 	{ XK_F1, /* F13 */  ShiftMask,      "\x33[1;2P",     0,    0},
// 	{ XK_F1, /* F25 */  ControlMask,    "\x33[1;5P",     0,    0},
// 	{ XK_F1, /* F37 */  Mod4Mask,       "\x33[1;6P",     0,    0},
// 	{ XK_F1, /* F49 */  Mod1Mask,       "\x33[1;3P",     0,    0},
// 	{ XK_F1, /* F61 */  Mod3Mask,       "\x33[1;4P",     0,    0},
// 	{ XK_F2,            XK_NO_MOD,      "\x33OQ" ,       0,    0},
// 	{ XK_F2, /* F14 */  ShiftMask,      "\x33[1;2Q",     0,    0},
// 	{ XK_F2, /* F26 */  ControlMask,    "\x33[1;5Q",     0,    0},
// 	{ XK_F2, /* F38 */  Mod4Mask,       "\x33[1;6Q",     0,    0},
// 	{ XK_F2, /* F50 */  Mod1Mask,       "\x33[1;3Q",     0,    0},
// 	{ XK_F2, /* F62 */  Mod3Mask,       "\x33[1;4Q",     0,    0},
// 	{ XK_F3,            XK_NO_MOD,      "\x33OR" ,       0,    0},
// 	{ XK_F3, /* F15 */  ShiftMask,      "\x33[1;2R",     0,    0},
// 	{ XK_F3, /* F27 */  ControlMask,    "\x33[1;5R",     0,    0},
// 	{ XK_F3, /* F39 */  Mod4Mask,       "\x33[1;6R",     0,    0},
// 	{ XK_F3, /* F51 */  Mod1Mask,       "\x33[1;3R",     0,    0},
// 	{ XK_F3, /* F63 */  Mod3Mask,       "\x33[1;4R",     0,    0},
// 	{ XK_F4,            XK_NO_MOD,      "\x33OS" ,       0,    0},
// 	{ XK_F4, /* F16 */  ShiftMask,      "\x33[1;2S",     0,    0},
// 	{ XK_F4, /* F28 */  ControlMask,    "\x33[1;5S",     0,    0},
// 	{ XK_F4, /* F40 */  Mod4Mask,       "\x33[1;6S",     0,    0},
// 	{ XK_F4, /* F52 */  Mod1Mask,       "\x33[1;3S",     0,    0},
// 	{ XK_F5,            XK_NO_MOD,      "\x33[15~",      0,    0},
// 	{ XK_F5, /* F17 */  ShiftMask,      "\x33[15;2~",    0,    0},
// 	{ XK_F5, /* F29 */  ControlMask,    "\x33[15;5~",    0,    0},
// 	{ XK_F5, /* F41 */  Mod4Mask,       "\x33[15;6~",    0,    0},
// 	{ XK_F5, /* F53 */  Mod1Mask,       "\x33[15;3~",    0,    0},
// 	{ XK_F6,            XK_NO_MOD,      "\x33[17~",      0,    0},
// 	{ XK_F6, /* F18 */  ShiftMask,      "\x33[17;2~",    0,    0},
// 	{ XK_F6, /* F30 */  ControlMask,    "\x33[17;5~",    0,    0},
// 	{ XK_F6, /* F42 */  Mod4Mask,       "\x33[17;6~",    0,    0},
// 	{ XK_F6, /* F54 */  Mod1Mask,       "\x33[17;3~",    0,    0},
// 	{ XK_F7,            XK_NO_MOD,      "\x33[18~",      0,    0},
// 	{ XK_F7, /* F19 */  ShiftMask,      "\x33[18;2~",    0,    0},
// 	{ XK_F7, /* F31 */  ControlMask,    "\x33[18;5~",    0,    0},
// 	{ XK_F7, /* F43 */  Mod4Mask,       "\x33[18;6~",    0,    0},
// 	{ XK_F7, /* F55 */  Mod1Mask,       "\x33[18;3~",    0,    0},
// 	{ XK_F8,            XK_NO_MOD,      "\x33[19~",      0,    0},
// 	{ XK_F8, /* F20 */  ShiftMask,      "\x33[19;2~",    0,    0},
// 	{ XK_F8, /* F32 */  ControlMask,    "\x33[19;5~",    0,    0},
// 	{ XK_F8, /* F44 */  Mod4Mask,       "\x33[19;6~",    0,    0},
// 	{ XK_F8, /* F56 */  Mod1Mask,       "\x33[19;3~",    0,    0},
// 	{ XK_F9,            XK_NO_MOD,      "\x33[20~",      0,    0},
// 	{ XK_F9, /* F21 */  ShiftMask,      "\x33[20;2~",    0,    0},
// 	{ XK_F9, /* F33 */  ControlMask,    "\x33[20;5~",    0,    0},
// 	{ XK_F9, /* F45 */  Mod4Mask,       "\x33[20;6~",    0,    0},
// 	{ XK_F9, /* F57 */  Mod1Mask,       "\x33[20;3~",    0,    0},
// 	{ XK_F10,           XK_NO_MOD,      "\x33[21~",      0,    0},
// 	{ XK_F10, /* F22 */ ShiftMask,      "\x33[21;2~",    0,    0},
// 	{ XK_F10, /* F34 */ ControlMask,    "\x33[21;5~",    0,    0},
// 	{ XK_F10, /* F46 */ Mod4Mask,       "\x33[21;6~",    0,    0},
// 	{ XK_F10, /* F58 */ Mod1Mask,       "\x33[21;3~",    0,    0},
// 	{ XK_F11,           XK_NO_MOD,      "\x33[23~",      0,    0},
// 	{ XK_F11, /* F23 */ ShiftMask,      "\x33[23;2~",    0,    0},
// 	{ XK_F11, /* F35 */ ControlMask,    "\x33[23;5~",    0,    0},
// 	{ XK_F11, /* F47 */ Mod4Mask,       "\x33[23;6~",    0,    0},
// 	{ XK_F11, /* F59 */ Mod1Mask,       "\x33[23;3~",    0,    0},
// 	{ XK_F12,           XK_NO_MOD,      "\x33[24~",      0,    0},
// 	{ XK_F12, /* F24 */ ShiftMask,      "\x33[24;2~",    0,    0},
// 	{ XK_F12, /* F36 */ ControlMask,    "\x33[24;5~",    0,    0},
// 	{ XK_F12, /* F48 */ Mod4Mask,       "\x33[24;6~",    0,    0},
// 	{ XK_F12, /* F60 */ Mod1Mask,       "\x33[24;3~",    0,    0},
// 	{ XK_F13,           XK_NO_MOD,      "\x33[1;2P",     0,    0},
// 	{ XK_F14,           XK_NO_MOD,      "\x33[1;2Q",     0,    0},
// 	{ XK_F15,           XK_NO_MOD,      "\x33[1;2R",     0,    0},
// 	{ XK_F16,           XK_NO_MOD,      "\x33[1;2S",     0,    0},
// 	{ XK_F17,           XK_NO_MOD,      "\x33[15;2~",    0,    0},
// 	{ XK_F18,           XK_NO_MOD,      "\x33[17;2~",    0,    0},
// 	{ XK_F19,           XK_NO_MOD,      "\x33[18;2~",    0,    0},
// 	{ XK_F20,           XK_NO_MOD,      "\x33[19;2~",    0,    0},
// 	{ XK_F21,           XK_NO_MOD,      "\x33[20;2~",    0,    0},
// 	{ XK_F22,           XK_NO_MOD,      "\x33[21;2~",    0,    0},
// 	{ XK_F23,           XK_NO_MOD,      "\x33[23;2~",    0,    0},
// 	{ XK_F24,           XK_NO_MOD,      "\x33[24;2~",    0,    0},
// 	{ XK_F25,           XK_NO_MOD,      "\x33[1;5P",     0,    0},
// 	{ XK_F26,           XK_NO_MOD,      "\x33[1;5Q",     0,    0},
// 	{ XK_F27,           XK_NO_MOD,      "\x33[1;5R",     0,    0},
// 	{ XK_F28,           XK_NO_MOD,      "\x33[1;5S",     0,    0},
// 	{ XK_F29,           XK_NO_MOD,      "\x33[15;5~",    0,    0},
// 	{ XK_F30,           XK_NO_MOD,      "\x33[17;5~",    0,    0},
// 	{ XK_F31,           XK_NO_MOD,      "\x33[18;5~",    0,    0},
// 	{ XK_F32,           XK_NO_MOD,      "\x33[19;5~",    0,    0},
// 	{ XK_F33,           XK_NO_MOD,      "\x33[20;5~",    0,    0},
// 	{ XK_F34,           XK_NO_MOD,      "\x33[21;5~",    0,    0},
// 	{ XK_F35,           XK_NO_MOD,      "\x33[23;5~",    0,    0},
// };

/*
 * Selection types' masks.
 * Use the same masks as usual.
 * Button1Mask is always unset, to make masks match between ButtonPress.
 * ButtonRelease and MotionNotify.
 * If no match is found, regular selection is used.
 */
static SELMASKS: [u64; 0] = [
    // TODO: static SEL_RECTANGULAR = ???
	// [SEL_RECTANGULAR] = x11::xlib::Mod1Mask,
];

/*
 * Printable characters in ASCII, used to estimate the advance width
 * of single wide characters.
 */
static ASCII_PRINTABLE: &str = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
