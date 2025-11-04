use usbd_hid::descriptor::KeyboardUsage as KU;


// Keyboard Symbols
pub struct KS {
    pub c: KU,    // Keyboard scan code
    pub s: char,  // Unicode character to display
}


pub const KEYMAPS: [[KS; 40]; 2] = [
    // ASCII letters a-z
    [
        KS{c: KU::KeyboardAa, s: 'a'},
        KS{c: KU::KeyboardBb, s: 'b'},
        KS{c: KU::KeyboardCc, s: 'c'},
        KS{c: KU::KeyboardDd, s: 'd'},
        KS{c: KU::KeyboardEe, s: 'e'},
        KS{c: KU::KeyboardFf, s: 'f'},
        KS{c: KU::KeyboardGg, s: 'g'},
        KS{c: KU::KeyboardHh, s: 'h'},
        KS{c: KU::KeyboardIi, s: 'i'},
        KS{c: KU::KeyboardJj, s: 'j'},
        KS{c: KU::KeyboardKk, s: 'k'},
        KS{c: KU::KeyboardLl, s: 'l'},
        KS{c: KU::KeyboardMm, s: 'm'},
        KS{c: KU::KeyboardNn, s: 'n'},
        KS{c: KU::KeyboardOo, s: 'o'},
        KS{c: KU::KeyboardPp, s: 'p'},
        KS{c: KU::KeyboardQq, s: 'q'},
        KS{c: KU::KeyboardRr, s: 'r'},
        KS{c: KU::KeyboardSs, s: 's'},
        KS{c: KU::KeyboardTt, s: 't'},
        KS{c: KU::KeyboardUu, s: 'u'},
        KS{c: KU::KeyboardVv, s: 'v'},
        KS{c: KU::KeyboardWw, s: 'w'},
        KS{c: KU::KeyboardXx, s: 'x'},
        KS{c: KU::KeyboardYy, s: 'y'},
        KS{c: KU::KeyboardZz, s: 'z'},
        KS{c: KU::KeyboardErrorUndefined, s: '\u{0000}'},
        KS{c: KU::KeyboardErrorUndefined, s: '\u{0000}'},
        KS{c: KU::KeyboardErrorUndefined, s: '\u{0000}'},
        KS{c: KU::KeyboardErrorUndefined, s: '\u{0000}'},
        KS{c: KU::KeyboardErrorUndefined, s: '\u{0000}'},
        KS{c: KU::KeyboardErrorUndefined, s: '\u{0000}'},
        KS{c: KU::KeyboardErrorUndefined, s: '\u{0000}'},
        KS{c: KU::KeyboardErrorUndefined, s: '\u{0000}'},
        KS{c: KU::KeyboardErrorUndefined, s: '\u{0000}'},
        KS{c: KU::KeyboardErrorUndefined, s: '\u{0000}'},
        KS{c: KU::KeyboardErrorUndefined, s: '\u{0000}'},
        KS{c: KU::KeyboardErrorUndefined, s: '\u{0000}'},
        KS{c: KU::KeyboardErrorUndefined, s: '\u{0000}'},
        KS{c: KU::KeyboardErrorUndefined, s: '\u{0000}'},
    ],
    // Ukrainian alphabet, see https://kbdlayout.info/KBDUR/
    [
        KS{c: KU::KeyboardFf, s: '\u{0430}'}, // CYRILLIC SMALL LETTER A
        KS{c: KU::KeyboardCommaLess, s: '\u{0431}'}, // CYRILLIC SMALL LETTER BE
        KS{c: KU::KeyboardDd, s: '\u{0432}'}, // CYRILLIC SMALL LETTER VE
        KS{c: KU::KeyboardUu, s: '\u{0433}'}, // CYRILLIC SMALL LETTER GHE
        KS{c: KU::KeyboardNonUSSlash, s: '\u{0491}'}, // CYRILLIC SMALL LETTER GHE WITH UPTURN
        KS{c: KU::KeyboardLl, s: '\u{0434}'}, // CYRILLIC SMALL LETTER DE
        KS{c: KU::KeyboardTt, s: '\u{0435}'}, // CYRILLIC SMALL LETTER IE
        KS{c: KU::KeyboardSingleDoubleQuote, s: '\u{0454}'}, // CYRILLIC SMALL LETTER UKRAINIAN IE
        KS{c: KU::KeyboardSemiColon, s: '\u{0436}'}, // CYRILLIC SMALL LETTER ZHE
        KS{c: KU::KeyboardPp, s: '\u{0437}'}, // CYRILLIC SMALL LETTER ZE
        KS{c: KU::KeyboardBb, s: '\u{0438}'}, // CYRILLIC SMALL LETTER I
        KS{c: KU::KeyboardSs, s: '\u{0456}'}, // CYRILLIC SMALL LETTER BYELORUSSIAN-UKRAINIAN I
        KS{c: KU::KeyboardCloseBracketBrace, s: '\u{0457}'}, // CYRILLIC SMALL LETTER YI
        KS{c: KU::KeyboardQq, s: '\u{0439}'}, // CYRILLIC SMALL LETTER SHORT I
        KS{c: KU::KeyboardRr, s: '\u{043a}'}, // CYRILLIC SMALL LETTER KA
        KS{c: KU::KeyboardKk, s: '\u{043b}'}, // CYRILLIC SMALL LETTER EL
        KS{c: KU::KeyboardVv, s: '\u{043c}'}, // CYRILLIC SMALL LETTER EM
        KS{c: KU::KeyboardYy, s: '\u{043d}'}, // CYRILLIC SMALL LETTER EN
        KS{c: KU::KeyboardJj, s: '\u{043e}'}, // CYRILLIC SMALL LETTER O
        KS{c: KU::KeyboardGg, s: '\u{043f}'}, // CYRILLIC SMALL LETTER PE
        KS{c: KU::KeyboardHh, s: '\u{0440}'}, // CYRILLIC SMALL LETTER ER
        KS{c: KU::KeyboardCc, s: '\u{0441}'}, // CYRILLIC SMALL LETTER ES
        KS{c: KU::KeyboardNn, s: '\u{0442}'}, // CYRILLIC SMALL LETTER TE
        KS{c: KU::KeyboardEe, s: '\u{0443}'}, // CYRILLIC SMALL LETTER U
        KS{c: KU::KeyboardAa, s: '\u{0444}'}, // CYRILLIC SMALL LETTER EF
        KS{c: KU::KeyboardOpenBracketBrace, s: '\u{0445}'}, // CYRILLIC SMALL LETTER HA
        KS{c: KU::KeyboardWw, s: '\u{0446}'}, // CYRILLIC SMALL LETTER TSE
        KS{c: KU::KeyboardXx, s: '\u{0447}'}, // CYRILLIC SMALL LETTER CHE
        KS{c: KU::KeyboardIi, s: '\u{0448}'}, // CYRILLIC SMALL LETTER SHA
        KS{c: KU::KeyboardOo, s: '\u{0449}'}, // CYRILLIC SMALL LETTER SHCHA
        KS{c: KU::KeyboardMm, s: '\u{044c}'}, // CYRILLIC SMALL LETTER SOFT SIGN
        KS{c: KU::KeyboardPeriodGreater, s: '\u{044e}'}, // CYRILLIC SMALL LETTER YU
        KS{c: KU::KeyboardZz, s: '\u{044f}'}, // CYRILLIC SMALL LETTER YA
        KS{c: KU::KeyboardErrorUndefined, s: '\u{0000}'},
        KS{c: KU::KeyboardErrorUndefined, s: '\u{0000}'},
        KS{c: KU::KeyboardErrorUndefined, s: '\u{0000}'},
        KS{c: KU::KeyboardErrorUndefined, s: '\u{0000}'},
        KS{c: KU::KeyboardErrorUndefined, s: '\u{0000}'},
        KS{c: KU::KeyboardErrorUndefined, s: '\u{0000}'},
        KS{c: KU::KeyboardErrorUndefined, s: '\u{0000}'},
    ],
];
