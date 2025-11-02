// Keyboard Symbols
pub struct KS {
    pub c: u8,
    pub s: char,
}


pub const KEYMAPS: [[KS; 40]; 2] = [
    // ASCII letters a-z
    [
        KS{c: 0x04, s: 'a'},
        KS{c: 0x05, s: 'b'},
        KS{c: 0x06, s: 'c'},
        KS{c: 0x07, s: 'd'},
        KS{c: 0x08, s: 'e'},
        KS{c: 0x09, s: 'f'},
        KS{c: 0x0a, s: 'g'},
        KS{c: 0x0b, s: 'h'},
        KS{c: 0x0c, s: 'i'},
        KS{c: 0x0d, s: 'j'},
        KS{c: 0x0e, s: 'k'},
        KS{c: 0x0f, s: 'l'},
        KS{c: 0x10, s: 'm'},
        KS{c: 0x11, s: 'n'},
        KS{c: 0x12, s: 'o'},
        KS{c: 0x13, s: 'p'},
        KS{c: 0x14, s: 'q'},
        KS{c: 0x15, s: 'r'},
        KS{c: 0x16, s: 's'},
        KS{c: 0x17, s: 't'},
        KS{c: 0x18, s: 'u'},
        KS{c: 0x19, s: 'v'},
        KS{c: 0x1a, s: 'w'},
        KS{c: 0x1b, s: 'x'},
        KS{c: 0x1c, s: 'y'},
        KS{c: 0x1d, s: 'z'},
        KS{c: 0, s: '\u{0000}'},
        KS{c: 0, s: '\u{0000}'},
        KS{c: 0, s: '\u{0000}'},
        KS{c: 0, s: '\u{0000}'},
        KS{c: 0, s: '\u{0000}'},
        KS{c: 0, s: '\u{0000}'},
        KS{c: 0, s: '\u{0000}'},
        KS{c: 0, s: '\u{0000}'},
        KS{c: 0, s: '\u{0000}'},
        KS{c: 0, s: '\u{0000}'},
        KS{c: 0, s: '\u{0000}'},
        KS{c: 0, s: '\u{0000}'},
        KS{c: 0, s: '\u{0000}'},
        KS{c: 0, s: '\u{0000}'},
    ],
    // Ukrainian alphabet, see https://kbdlayout.info/KBDUR/
    [
        KS{c: 0x21, s: '\u{0430}'}, // CYRILLIC SMALL LETTER A
        KS{c: 0x33, s: '\u{0431}'}, // CYRILLIC SMALL LETTER BE
        KS{c: 0x20, s: '\u{0432}'}, // CYRILLIC SMALL LETTER VE
        KS{c: 0x16, s: '\u{0433}'}, // CYRILLIC SMALL LETTER GHE
        KS{c: 0x56, s: '\u{0491}'}, // CYRILLIC SMALL LETTER GHE WITH UPTURN
        KS{c: 0x26, s: '\u{0434}'}, // CYRILLIC SMALL LETTER DE
        KS{c: 0x14, s: '\u{0435}'}, // CYRILLIC SMALL LETTER IE
        KS{c: 0x28, s: '\u{0454}'}, // CYRILLIC SMALL LETTER UKRAINIAN IE
        KS{c: 0x27, s: '\u{0436}'}, // CYRILLIC SMALL LETTER ZHE
        KS{c: 0x19, s: '\u{0437}'}, // CYRILLIC SMALL LETTER ZE
        KS{c: 0x30, s: '\u{0438}'}, // CYRILLIC SMALL LETTER I
        KS{c: 0x1f, s: '\u{0456}'}, // CYRILLIC SMALL LETTER BYELORUSSIAN-UKRAINIAN I
        KS{c: 0x1b, s: '\u{0457}'}, // CYRILLIC SMALL LETTER YI
        KS{c: 0x10, s: '\u{0439}'}, // CYRILLIC SMALL LETTER SHORT I
        KS{c: 0x13, s: '\u{043a}'}, // CYRILLIC SMALL LETTER KA
        KS{c: 0x25, s: '\u{043b}'}, // CYRILLIC SMALL LETTER EL
        KS{c: 0x2f, s: '\u{043c}'}, // CYRILLIC SMALL LETTER EM
        KS{c: 0x15, s: '\u{043d}'}, // CYRILLIC SMALL LETTER EN
        KS{c: 0x24, s: '\u{043e}'}, // CYRILLIC SMALL LETTER O
        KS{c: 0x22, s: '\u{043f}'}, // CYRILLIC SMALL LETTER PE
        KS{c: 0x23, s: '\u{0440}'}, // CYRILLIC SMALL LETTER ER
        KS{c: 0x2e, s: '\u{0441}'}, // CYRILLIC SMALL LETTER ES
        KS{c: 0x31, s: '\u{0442}'}, // CYRILLIC SMALL LETTER TE
        KS{c: 0x12, s: '\u{0443}'}, // CYRILLIC SMALL LETTER U
        KS{c: 0x1e, s: '\u{0444}'}, // CYRILLIC SMALL LETTER EF
        KS{c: 0x1a, s: '\u{0445}'}, // CYRILLIC SMALL LETTER HA
        KS{c: 0x11, s: '\u{0446}'}, // CYRILLIC SMALL LETTER TSE
        KS{c: 0x2d, s: '\u{0447}'}, // CYRILLIC SMALL LETTER CHE
        KS{c: 0x17, s: '\u{0448}'}, // CYRILLIC SMALL LETTER SHA
        KS{c: 0x18, s: '\u{0449}'}, // CYRILLIC SMALL LETTER SHCHA
        KS{c: 0x32, s: '\u{044c}'}, // CYRILLIC SMALL LETTER SOFT SIGN
        KS{c: 0x34, s: '\u{044e}'}, // CYRILLIC SMALL LETTER YU
        KS{c: 0x2c, s: '\u{044f}'}, // CYRILLIC SMALL LETTER YA
        KS{c: 0, s: '\u{0000}'},
        KS{c: 0, s: '\u{0000}'},
        KS{c: 0, s: '\u{0000}'},
        KS{c: 0, s: '\u{0000}'},
        KS{c: 0, s: '\u{0000}'},
        KS{c: 0, s: '\u{0000}'},
        KS{c: 0, s: '\u{0000}'},
    ],
];
