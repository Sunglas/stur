#![allow(dead_code)]
/* See LICENSE for license details. */

enum WinMode {
	ModeVisible,
	ModeFocused,
	ModeAppkeypad,
	ModeMousebtn,
	ModeMousemotion,
	ModeReverse,
	ModeKbdlock,
	ModeHide,
	ModeAppcursor,
	ModeMousesgr,
	Mode8bit,
	ModeBlink,
	ModeFblink,
	ModeFocus,
	ModeMousex10,
	ModeMousemany,
	ModeBrcktpaste,
	ModeNumlock,
    // TODO: whatever this is
	//MODE_MOUSE       = MODE_MOUSEBTN|MODE_MOUSEMOTION|MODE_MOUSEX10\
	//                  |MODE_MOUSEMANY,
}

// void xbell(void);
// void xclipcopy(void);
// void xdrawcursor(int, int, Glyph, int, int, Glyph);
// void xdrawline(Line, int, int, int);
// void xfinishdraw(void);
// void xloadcols(void);
// int xsetcolorname(int, const char *);
// int xgetcolor(int, unsigned char *, unsigned char *, unsigned char *);
// void xseticontitle(char *);
// void xsettitle(char *);
// int xsetcursor(int);
// void xsetmode(int, unsigned int);
// void xsetpointermotion(int);
// void xsetsel(char *);
// int xstartdraw(void);
// void xximspot(int, int);
