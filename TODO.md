# Pending files to port over to rust:
* arg.h: -> arg.rs
  * Status: not yet implemented
* config.def.h: -> config.rs
  * Status: Partially implemented
  * Learn how to implement wchar_t in a minimal way
  * Find out a good way to set up COLORNAME array
  * Missing types:
    * XC_xterm
    * MouseShortcut
    * Shortcut
    * KeySym
    * XK_SWITCH_MOD
    * Key
  * Missing variables:
    * SEL_RECTANGULAR
* st.c: -> st.rs
  * Status: not yet implemented
* st.h: st/st_headers.rs
  * Status: Partially implemented
  * Macros not yet implemented
  * Function prototypes not yet implemented
  * Variable declarations are invalid rust
* win.h: st/win_headers.rs
  * Status: Partially implemented
  * Missing variables:
    * MODE_MOUSEBTN
    * MODE_MOUSEMOTION
    * MODE_MOUSEX10
    * MODE_MOUSEMANY
  * Function prototypes are not yet implemented
* x.c: -> x.rs
  * Status: Not yet implemented
