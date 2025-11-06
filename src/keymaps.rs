use usbd_hid::descriptor::KeyboardUsage as KU;

// Keyboard Symbols
// KU::KeyboardErrorRollOver indicates a reference to COMPLEX_KEYMAPS, s is treatted as integer
pub struct KS {
    pub c: KU,         // Keyboard scan code
    pub s: [char; 3],  // Unicode symbols to display
}


pub const KEYMAPS: [[KS; 40]; 4] = [
    // ASCII letters a-z
    [
        KS{c: KU::KeyboardAa, s: ['a', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardBb, s: ['b', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardCc, s: ['c', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardDd, s: ['d', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardEe, s: ['e', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardFf, s: ['f', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardGg, s: ['g', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardHh, s: ['h', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardIi, s: ['i', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardJj, s: ['j', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardKk, s: ['k', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardLl, s: ['l', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardMm, s: ['m', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardNn, s: ['n', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardOo, s: ['o', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardPp, s: ['p', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardQq, s: ['q', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardRr, s: ['r', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardSs, s: ['s', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardTt, s: ['t', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardUu, s: ['u', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardVv, s: ['v', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardWw, s: ['w', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardXx, s: ['x', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardYy, s: ['y', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardZz, s: ['z', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
    ],
    // Ukrainian alphabet, see https://kbdlayout.info/KBDUR/
    [
        KS{c: KU::KeyboardFf, s: ['\u{0430}', '\u{0000}', '\u{0000}']}, // CYRILLIC SMALL LETTER A
        KS{c: KU::KeyboardCommaLess, s: ['\u{0431}', '\u{0000}', '\u{0000}']}, // CYRILLIC SMALL LETTER BE
        KS{c: KU::KeyboardDd, s: ['\u{0432}', '\u{0000}', '\u{0000}']}, // CYRILLIC SMALL LETTER VE
        KS{c: KU::KeyboardUu, s: ['\u{0433}', '\u{0000}', '\u{0000}']}, // CYRILLIC SMALL LETTER GHE
        KS{c: KU::KeyboardErrorRollOver, s: [0 as char, '\u{0000}', '\u{0000}']}, // CYRILLIC SMALL LETTER GHE WITH UPTURN
        KS{c: KU::KeyboardLl, s: ['\u{0434}', '\u{0000}', '\u{0000}']}, // CYRILLIC SMALL LETTER DE
        KS{c: KU::KeyboardTt, s: ['\u{0435}', '\u{0000}', '\u{0000}']}, // CYRILLIC SMALL LETTER IE
        KS{c: KU::KeyboardSingleDoubleQuote, s: ['\u{0454}', '\u{0000}', '\u{0000}']}, // CYRILLIC SMALL LETTER UKRAINIAN IE
        KS{c: KU::KeyboardSemiColon, s: ['\u{0436}', '\u{0000}', '\u{0000}']}, // CYRILLIC SMALL LETTER ZHE
        KS{c: KU::KeyboardPp, s: ['\u{0437}', '\u{0000}', '\u{0000}']}, // CYRILLIC SMALL LETTER ZE
        KS{c: KU::KeyboardBb, s: ['\u{0438}', '\u{0000}', '\u{0000}']}, // CYRILLIC SMALL LETTER I
        KS{c: KU::KeyboardSs, s: ['\u{0456}', '\u{0000}', '\u{0000}']}, // CYRILLIC SMALL LETTER BYELORUSSIAN-UKRAINIAN I
        KS{c: KU::KeyboardCloseBracketBrace, s: ['\u{0457}', '\u{0000}', '\u{0000}']}, // CYRILLIC SMALL LETTER YI
        KS{c: KU::KeyboardQq, s: ['\u{0439}', '\u{0000}', '\u{0000}']}, // CYRILLIC SMALL LETTER SHORT I
        KS{c: KU::KeyboardRr, s: ['\u{043a}', '\u{0000}', '\u{0000}']}, // CYRILLIC SMALL LETTER KA
        KS{c: KU::KeyboardKk, s: ['\u{043b}', '\u{0000}', '\u{0000}']}, // CYRILLIC SMALL LETTER EL
        KS{c: KU::KeyboardVv, s: ['\u{043c}', '\u{0000}', '\u{0000}']}, // CYRILLIC SMALL LETTER EM
        KS{c: KU::KeyboardYy, s: ['\u{043d}', '\u{0000}', '\u{0000}']}, // CYRILLIC SMALL LETTER EN
        KS{c: KU::KeyboardJj, s: ['\u{043e}', '\u{0000}', '\u{0000}']}, // CYRILLIC SMALL LETTER O
        KS{c: KU::KeyboardGg, s: ['\u{043f}', '\u{0000}', '\u{0000}']}, // CYRILLIC SMALL LETTER PE
        KS{c: KU::KeyboardHh, s: ['\u{0440}', '\u{0000}', '\u{0000}']}, // CYRILLIC SMALL LETTER ER
        KS{c: KU::KeyboardCc, s: ['\u{0441}', '\u{0000}', '\u{0000}']}, // CYRILLIC SMALL LETTER ES
        KS{c: KU::KeyboardNn, s: ['\u{0442}', '\u{0000}', '\u{0000}']}, // CYRILLIC SMALL LETTER TE
        KS{c: KU::KeyboardEe, s: ['\u{0443}', '\u{0000}', '\u{0000}']}, // CYRILLIC SMALL LETTER U
        KS{c: KU::KeyboardAa, s: ['\u{0444}', '\u{0000}', '\u{0000}']}, // CYRILLIC SMALL LETTER EF
        KS{c: KU::KeyboardOpenBracketBrace, s: ['\u{0445}', '\u{0000}', '\u{0000}']}, // CYRILLIC SMALL LETTER HA
        KS{c: KU::KeyboardWw, s: ['\u{0446}', '\u{0000}', '\u{0000}']}, // CYRILLIC SMALL LETTER TSE
        KS{c: KU::KeyboardXx, s: ['\u{0447}', '\u{0000}', '\u{0000}']}, // CYRILLIC SMALL LETTER CHE
        KS{c: KU::KeyboardIi, s: ['\u{0448}', '\u{0000}', '\u{0000}']}, // CYRILLIC SMALL LETTER SHA
        KS{c: KU::KeyboardOo, s: ['\u{0449}', '\u{0000}', '\u{0000}']}, // CYRILLIC SMALL LETTER SHCHA
        KS{c: KU::KeyboardMm, s: ['\u{044c}', '\u{0000}', '\u{0000}']}, // CYRILLIC SMALL LETTER SOFT SIGN
        KS{c: KU::KeyboardPeriodGreater, s: ['\u{044e}', '\u{0000}', '\u{0000}']}, // CYRILLIC SMALL LETTER YU
        KS{c: KU::KeyboardZz, s: ['\u{044f}', '\u{0000}', '\u{0000}']}, // CYRILLIC SMALL LETTER YA
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
    ],
    // Digits and symbols
    [
        KS{c: KU::KeyboardErrorRollOver, s: [1 as char, '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorRollOver, s: [2 as char, '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorRollOver, s: [3 as char, '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorRollOver, s: [4 as char, '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorRollOver, s: [5 as char, '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorRollOver, s: [6 as char, '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorRollOver, s: [7 as char, '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorRollOver, s: [8 as char, '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorRollOver, s: [9 as char, '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorRollOver, s: [10 as char, '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorRollOver, s: [11 as char, '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorRollOver, s: [12 as char, '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorRollOver, s: [13 as char, '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorRollOver, s: [14 as char, '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorRollOver, s: [15 as char, '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorRollOver, s: [16 as char, '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorRollOver, s: [17 as char, '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorRollOver, s: [18 as char, '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorRollOver, s: [19 as char, '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorRollOver, s: [20 as char, '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorRollOver, s: [21 as char, '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardEscape, s: ['E', 's', 'c']},
        KS{c: KU::KeyboardTab, s: ['T', 'a', 'b']},
        KS{c: KU::KeyboardLeftArrow, s: ['L', 'F', 'T']},
        KS{c: KU::KeyboardRightArrow, s: ['R', 'G', 'T']},
        KS{c: KU::KeyboardUpArrow, s: ['U', 'P', '\u{0000}']},
        KS{c: KU::KeyboardDownArrow, s: ['D', 'W', 'N']},
        KS{c: KU::KeyboardPageUp, s: ['P', 'U', '\u{0000}']},
        KS{c: KU::KeyboardPageDown, s: ['P', 'D', '\u{0000}']},
        KS{c: KU::KeyboardHome, s: ['H', 'O', 'M']},
        KS{c: KU::KeyboardEnd, s: ['E', 'N', 'D']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
    ],
    // F keys
    [
        KS{c: KU::KeyboardF1, s: ['F', '1', '\u{0000}']},
        KS{c: KU::KeyboardF2, s: ['F', '2', '\u{0000}']},
        KS{c: KU::KeyboardF3, s: ['F', '3', '\u{0000}']},
        KS{c: KU::KeyboardF4, s: ['F', '4', '\u{0000}']},
        KS{c: KU::KeyboardF5, s: ['F', '5', '\u{0000}']},
        KS{c: KU::KeyboardF6, s: ['F', '6', '\u{0000}']},
        KS{c: KU::KeyboardF7, s: ['F', '7', '\u{0000}']},
        KS{c: KU::KeyboardF8, s: ['F', '8', '\u{0000}']},
        KS{c: KU::KeyboardF9, s: ['F', '9', '\u{0000}']},
        KS{c: KU::KeyboardF10, s: ['F', '1', '0']},
        KS{c: KU::KeyboardF11, s: ['F', '1', '1']},
        KS{c: KU::KeyboardF12, s: ['F', '1', '2']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
        KS{c: KU::KeyboardErrorUndefined, s: ['\u{0000}', '\u{0000}', '\u{0000}']},
    ],
];


// Complex Keyboard Symbols
pub struct KSComplex {
    pub display_str: [char; 3],       // up to 3 characters to display
    pub keycodes: [(u8, [KU;2]); 2],  // up to 2 (modifier, keycodes) tuples
}


pub const COMPLEX_KEYMAPS: [KSComplex; 22] = [
    // 0: RightAlt+U -> CYRILLIC SMALL LETTER GHE WITH UPTURN
    KSComplex {display_str: ['\u{0491}', '\u{0000}', '\u{0000}'],
               keycodes: [(0b0100_0000, [KU::KeyboardUu, KU::KeyboardErrorUndefined]),
                          (0b0000_0000, [KU::KeyboardErrorUndefined, KU::KeyboardErrorUndefined]),
               ]},
    // 1: 1 and !
    KSComplex {display_str: ['\u{0031}', '\u{0020}', '\u{0021}'],
               keycodes: [(0b0000_0000, [KU::Keyboard1Exclamation, KU::KeyboardErrorUndefined]),
                          (0b0000_0000, [KU::KeyboardErrorUndefined, KU::KeyboardErrorUndefined]),
               ]},
    // 2: 2 and @
    KSComplex {display_str: ['\u{0032}', '\u{0020}', '\u{0040}'],
               keycodes: [(0b0000_0000, [KU::Keyboard2At, KU::KeyboardErrorUndefined]),
                          (0b0000_0000, [KU::KeyboardErrorUndefined, KU::KeyboardErrorUndefined]),
               ]},
    // 3: 3 and #
    KSComplex {display_str: ['\u{0033}', '\u{0020}', '\u{0023}'],
               keycodes: [(0b0000_0000, [KU::Keyboard3Hash, KU::KeyboardErrorUndefined]),
                          (0b0000_0000, [KU::KeyboardErrorUndefined, KU::KeyboardErrorUndefined]),
               ]},
    // 4: 4 and $
    KSComplex {display_str: ['\u{0034}', '\u{0020}', '\u{0024}'],
               keycodes: [(0b0000_0000, [KU::Keyboard4Dollar, KU::KeyboardErrorUndefined]),
                          (0b0000_0000, [KU::KeyboardErrorUndefined, KU::KeyboardErrorUndefined]),
               ]},
    // 5: 5 and %
    KSComplex {display_str: ['\u{0035}', '\u{0020}', '\u{0025}'],
               keycodes: [(0b0000_0000, [KU::Keyboard5Percent, KU::KeyboardErrorUndefined]),
                          (0b0000_0000, [KU::KeyboardErrorUndefined, KU::KeyboardErrorUndefined]),
               ]},
    // 6: 6 and ^
    KSComplex {display_str: ['\u{0036}', '\u{0020}', '\u{005E}'],
               keycodes: [(0b0000_0000, [KU::Keyboard6Caret, KU::KeyboardErrorUndefined]),
                          (0b0000_0000, [KU::KeyboardErrorUndefined, KU::KeyboardErrorUndefined]),
               ]},
    // 7: 7 and &
    KSComplex {display_str: ['\u{0037}', '\u{0020}', '\u{0026}'],
               keycodes: [(0b0000_0000, [KU::Keyboard7Ampersand, KU::KeyboardErrorUndefined]),
                          (0b0000_0000, [KU::KeyboardErrorUndefined, KU::KeyboardErrorUndefined]),
               ]},
    // 8: 8 and *
    KSComplex {display_str: ['\u{0038}', '\u{0020}', '\u{002A}'],
               keycodes: [(0b0000_0000, [KU::Keyboard8Asterisk, KU::KeyboardErrorUndefined]),
                          (0b0000_0000, [KU::KeyboardErrorUndefined, KU::KeyboardErrorUndefined]),
               ]},
    // 9: 9 and (
    KSComplex {display_str: ['\u{0039}', '\u{0020}', '\u{0028}'],
               keycodes: [(0b0000_0000, [KU::Keyboard9OpenParens, KU::KeyboardErrorUndefined]),
                          (0b0000_0000, [KU::KeyboardErrorUndefined, KU::KeyboardErrorUndefined]),
               ]},
    // 10: 0 and )
    KSComplex {display_str: ['\u{0030}', '\u{0020}', '\u{0029}'],
               keycodes: [(0b0000_0000, [KU::Keyboard0CloseParens, KU::KeyboardErrorUndefined]),
                          (0b0000_0000, [KU::KeyboardErrorUndefined, KU::KeyboardErrorUndefined]),
               ]},
    // 11: - and _
    KSComplex {display_str: ['\u{002D}', '\u{0020}', '\u{005F}'],
               keycodes: [(0b0000_0000, [KU::KeyboardDashUnderscore, KU::KeyboardErrorUndefined]),
                          (0b0000_0000, [KU::KeyboardErrorUndefined, KU::KeyboardErrorUndefined]),
               ]},
    // 12: = and +
    KSComplex {display_str: ['\u{003D}', '\u{0020}', '\u{002B}'],
               keycodes: [(0b0000_0000, [KU::KeyboardEqualPlus, KU::KeyboardErrorUndefined]),
                          (0b0000_0000, [KU::KeyboardErrorUndefined, KU::KeyboardErrorUndefined]),
               ]},
    // 13: [ and {
    KSComplex {display_str: ['\u{005B}', '\u{0020}', '\u{007B}'],
               keycodes: [(0b0000_0000, [KU::KeyboardOpenBracketBrace, KU::KeyboardErrorUndefined]),
                          (0b0000_0000, [KU::KeyboardErrorUndefined, KU::KeyboardErrorUndefined]),
               ]},
    // 14: ] and }
    KSComplex {display_str: ['\u{005D}', '\u{0020}', '\u{007D}'],
               keycodes: [(0b0000_0000, [KU::KeyboardCloseBracketBrace, KU::KeyboardErrorUndefined]),
                          (0b0000_0000, [KU::KeyboardErrorUndefined, KU::KeyboardErrorUndefined]),
               ]},
    // 15: \ and |
    KSComplex {display_str: ['\u{005C}', '\u{0020}', '\u{007C}'],
               keycodes: [(0b0000_0000, [KU::KeyboardBackslashBar, KU::KeyboardErrorUndefined]),
                          (0b0000_0000, [KU::KeyboardErrorUndefined, KU::KeyboardErrorUndefined]),
               ]},
    // 16: ; and :
    KSComplex {display_str: ['\u{003B}', '\u{0020}', '\u{003A}'],
               keycodes: [(0b0000_0000, [KU::KeyboardSemiColon, KU::KeyboardErrorUndefined]),
                          (0b0000_0000, [KU::KeyboardErrorUndefined, KU::KeyboardErrorUndefined]),
               ]},
    // 17: ' and "
    KSComplex {display_str: ['\u{0027}', '\u{0020}', '\u{0022}'],
               keycodes: [(0b0000_0000, [KU::KeyboardSingleDoubleQuote, KU::KeyboardErrorUndefined]),
                          (0b0000_0000, [KU::KeyboardErrorUndefined, KU::KeyboardErrorUndefined]),
               ]},
    // 18: ` and ~
    KSComplex {display_str: ['\u{0060}', '\u{0020}', '\u{007E}'],
               keycodes: [(0b0000_0000, [KU::KeyboardBacktickTilde, KU::KeyboardErrorUndefined]),
                          (0b0000_0000, [KU::KeyboardErrorUndefined, KU::KeyboardErrorUndefined]),
               ]},
    // 19: , and <
    KSComplex {display_str: ['\u{002C}', '\u{0020}', '\u{003C}'],
               keycodes: [(0b0000_0000, [KU::KeyboardCommaLess, KU::KeyboardErrorUndefined]),
                          (0b0000_0000, [KU::KeyboardErrorUndefined, KU::KeyboardErrorUndefined]),
               ]},
    // 20: . and >
    KSComplex {display_str: ['\u{002E}', '\u{0020}', '\u{003E}'],
               keycodes: [(0b0000_0000, [KU::KeyboardPeriodGreater, KU::KeyboardErrorUndefined]),
                          (0b0000_0000, [KU::KeyboardErrorUndefined, KU::KeyboardErrorUndefined]),
               ]},
    // 21: / and ?
    KSComplex {display_str: ['\u{002F}', '\u{0020}', '\u{003F}'],
               keycodes: [(0b0000_0000, [KU::KeyboardSlashQuestion, KU::KeyboardErrorUndefined]),
                          (0b0000_0000, [KU::KeyboardErrorUndefined, KU::KeyboardErrorUndefined]),
               ]},
];
