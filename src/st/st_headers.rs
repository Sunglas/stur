
/* See LICENSE for license details. */

/* TODO: macros */
/*
#define MIN(a, b)		((a) < (b) ? (a) : (b))
#define MAX(a, b)		((a) < (b) ? (b) : (a))
#define LEN(a)			(sizeof(a) / sizeof(a)[0])
#define BETWEEN(x, a, b)	((a) <= (x) && (x) <= (b))
#define DIVCEIL(n, d)		(((n) + ((d) - 1)) / (d))
#define DEFAULT(a, b)		(a) = (a) ? (a) : (b)
#define LIMIT(x, a, b)		(x) = (x) < (a) ? (a) : (x) > (b) ? (b) : (x)
#define ATTRCMP(a, b)		((a).mode != (b).mode || (a).fg != (b).fg || \
				(a).bg != (b).bg)
#define TIMEDIFF(t1, t2)	((t1.tv_sec-t2.tv_sec)*1000 + \
				(t1.tv_nsec-t2.tv_nsec)/1E6)
#define MODBIT(x, set, bit)	((set) ? ((x) |= (bit)) : ((x) &= ~(bit)))

#define TRUECOLOR(r,g,b)	(1 << 24 | (r) << 16 | (g) << 8 | (b))
#define IS_TRUECOL(x)		(1 << 24 & (x))
*/

enum GlyphAttribute {
	AttrNull,
	AttrBold,
	AttrFaint,
	AttrItalic,
	AttrUnderline,
	AttrBlink,
	AttrReverse,
	AttrInvisible,
	AttrStruck,
	AttrWrap,
	AttrWide,
	AttrWdummy,
	AttrBoldFaint, // ATTR_BOLD|ATTR_FAINT missing
}

enum SelectionMode {
	SelIdle,
	SelEmpty,
	SelReady,
}

enum SelectionType {
	SelRegular,
	SelRectangular,
}

enum SelectionSnap {
	SnapWord,
	SnapLine,
}

// type uchar = char;
// type uint = u32;
// type ulong = u64;
// type ushort = u16;

type Rune = u32;

struct Glyph {
	u: Rune,           /* character code */
	mode: u16,      /* attribute flags */
	fg: u32,      /* foreground  */
	bg: u32,      /* background  */
}

type Line = *mut Glyph;

#[repr(C)]
union Arg {
	i: i32,
	ui: u32,
	f: f32,
	v: *const fn(),
	s: *const fn() -> &'static str,
}

/*
void die(const char *, ...);
void redraw(void);
void draw(void);

void printscreen(const Arg *);
void printsel(const Arg *);
void sendbreak(const Arg *);
void toggleprinter(const Arg *);

int tattrset(int);
void tnew(int, int);
void tresize(int, int);
void tsetdirtattr(int);
void ttyhangup(void);
int ttynew(const char *, char *, const char *, char **);
size_t ttyread(void);
void ttyresize(int, int);
void ttywrite(const char *, size_t, int);

void resettitle(void);

void selclear(void);
void selinit(void);
void selstart(int, int, int);
void selextend(int, int, int, int);
int selected(int, int);
char *getsel(void);

size_t utf8encode(Rune, char *);

void *xmalloc(size_t);
void *xrealloc(void *, size_t);
char *xstrdup(const char *);
*/

/* config.h globals */
// TODO: rust doesn't do it like this; maybe remove?
// extern char *utmp;
// extern char *scroll;
// extern char *stty_args;
// extern char *vtiden;
// extern wchar_t *worddelimiters;
// extern int allowaltscreen;
// extern int allowwindowops;
// extern char *termname;
// extern unsigned int tabspaces;
// extern unsigned int defaultfg;
// extern unsigned int defaultbg;
// extern unsigned int defaultcs;
