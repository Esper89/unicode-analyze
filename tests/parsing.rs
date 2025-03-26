use unicode_analyze::Text;

struct TestCase {
    text: Text,
    string_rep: &'static str,
    out: &'static [(&'static str, &'static str, &'static str)],
}

impl TestCase {
    fn run(self) {
        assert_eq!(self.text.to_string(), self.string_rep);
        let codepoints: Vec<_> = self.text.codepoints().collect();
        assert_eq!(self.out.len(), codepoints.len());

        for ((value, character, name), codepoint) in self.out.iter().cloned().zip(codepoints) {
            assert_eq!(value, codepoint.display_value().to_string());
            assert_eq!(character, codepoint.display_character().to_string());
            assert_eq!(name, codepoint.display_name().to_string());
        }
    }
}

#[test]
fn hello_world() {
    // 'Hello, World!'
    TestCase {
        text: Text::parse_str("Hello, World!"),
        string_rep: "['H', 'e', 'l', 'l', 'o', ',', ' ', 'W', 'o', 'r', 'l', 'd', '!']",
        out: &[
            ("U+0048", "'H'", "LATIN CAPITAL LETTER H"),
            ("U+0065", "'e'", "LATIN SMALL LETTER E"),
            ("U+006C", "'l'", "LATIN SMALL LETTER L"),
            ("U+006C", "'l'", "LATIN SMALL LETTER L"),
            ("U+006F", "'o'", "LATIN SMALL LETTER O"),
            ("U+002C", "','", "COMMA"),
            ("U+0020", "' '", "SPACE"),
            ("U+0057", "'W'", "LATIN CAPITAL LETTER W"),
            ("U+006F", "'o'", "LATIN SMALL LETTER O"),
            ("U+0072", "'r'", "LATIN SMALL LETTER R"),
            ("U+006C", "'l'", "LATIN SMALL LETTER L"),
            ("U+0064", "'d'", "LATIN SMALL LETTER D"),
            ("U+0021", "'!'", "EXCLAMATION MARK"),
        ],
    }
    .run()
}

#[test]
fn control_codes() {
    // $'\v\t\r\n'
    TestCase {
        text: Text::parse_str("\u{000B}\t\r\n"),
        string_rep: "[VT, HT, [CR + LF]]",
        out: &[
            ("U+000B", "VT", "LINE TABULATION"),
            ("U+0009", "HT", "CHARACTER TABULATION"),
            ("U+000D", "CR", "CARRIAGE RETURN"),
            ("U+000A", "LF", "LINE FEED"),
        ],
    }
    .run()
}

#[test]
fn emojis() {
    // '👩‍👩‍👧‍👦 😵‍💫'
    TestCase {
        text: Text::parse_str("👩‍👩‍👧‍👦 😵‍💫"),
        string_rep: "[['👩' + ZWJ + '👩' + ZWJ + '👧' + ZWJ + '👦'], ' ', ['😵' + ZWJ + '💫']]",
        out: &[
            ("U+01F469", "'👩'", "WOMAN"),
            ("U+200D", "ZWJ", "ZERO WIDTH JOINER"),
            ("U+01F469", "'👩'", "WOMAN"),
            ("U+200D", "ZWJ", "ZERO WIDTH JOINER"),
            ("U+01F467", "'👧'", "GIRL"),
            ("U+200D", "ZWJ", "ZERO WIDTH JOINER"),
            ("U+01F466", "'👦'", "BOY"),
            ("U+0020", "' '", "SPACE"),
            ("U+01F635", "'😵'", "DIZZY FACE"),
            ("U+200D", "ZWJ", "ZERO WIDTH JOINER"),
            ("U+01F4AB", "'💫'", "DIZZY SYMBOL"),
        ],
    }
    .run()
}

#[test]
fn combining_characters() {
    // m͌͊e̵͂o͐͝w͐̾
    TestCase {
        text: Text::parse_str("m͌͊e̵͂o͐͝w͐̾"),
        string_rep: "[['m' + '◌͌' + '◌͊'], ['e' + '◌̵' + '◌͂'], ['o' + '◌͐' + '◌͝◌'], ['w' + '◌͐' \
                     + '◌̾']]",
        out: &[
            ("U+006D", "'m'", "LATIN SMALL LETTER M"),
            ("U+034C", "'◌͌'", "COMBINING ALMOST EQUAL TO ABOVE"),
            ("U+034A", "'◌͊'", "COMBINING NOT TILDE ABOVE"),
            ("U+0065", "'e'", "LATIN SMALL LETTER E"),
            ("U+0335", "'◌̵'", "COMBINING SHORT STROKE OVERLAY"),
            ("U+0342", "'◌͂'", "COMBINING GREEK PERISPOMENI"),
            ("U+006F", "'o'", "LATIN SMALL LETTER O"),
            ("U+0350", "'◌͐'", "COMBINING RIGHT ARROWHEAD ABOVE"),
            ("U+035D", "'◌͝◌'", "COMBINING DOUBLE BREVE"),
            ("U+0077", "'w'", "LATIN SMALL LETTER W"),
            ("U+0350", "'◌͐'", "COMBINING RIGHT ARROWHEAD ABOVE"),
            ("U+033E", "'◌̾'", "COMBINING VERTICAL TILDE"),
        ],
    }
    .run()
}

#[test]
fn rtl() {
    // اَلْعَرَبِيَّةُ
    TestCase {
        text: Text::parse_str("اَلْعَرَبِيَّةُ"),
        string_rep: "[['‎ا‎' + '◌َ'], ['‎ل‎' + '◌ْ'], ['‎ع‎' + '◌َ'], ['‎ر‎' + '◌َ'], ['‎ب‎' + '◌ِ'\
                     ], ['‎ي‎' + '◌َ' + '◌ّ'], ['‎ة‎' + '◌ُ']]",
        out: &[
            ("U+0627", "'‎ا‎'", "ARABIC LETTER ALEF"),
            ("U+064E", "'◌َ'", "ARABIC FATHA"),
            ("U+0644", "'‎ل‎'", "ARABIC LETTER LAM"),
            ("U+0652", "'◌ْ'", "ARABIC SUKUN"),
            ("U+0639", "'‎ع‎'", "ARABIC LETTER AIN"),
            ("U+064E", "'◌َ'", "ARABIC FATHA"),
            ("U+0631", "'‎ر‎'", "ARABIC LETTER REH"),
            ("U+064E", "'◌َ'", "ARABIC FATHA"),
            ("U+0628", "'‎ب‎'", "ARABIC LETTER BEH"),
            ("U+0650", "'◌ِ'", "ARABIC KASRA"),
            ("U+064A", "'‎ي‎'", "ARABIC LETTER YEH"),
            ("U+064E", "'◌َ'", "ARABIC FATHA"),
            ("U+0651", "'◌ّ'", "ARABIC SHADDA"),
            ("U+0629", "'‎ة‎'", "ARABIC LETTER TEH MARBUTA"),
            ("U+064F", "'◌ُ'", "ARABIC DAMMA"),
        ],
    }
    .run()
}

#[test]
fn invalid() {
    // $'\xF2\x80\x80\x80\xF4\x8F\xBF\xBD\xEF\xBF\xBF\xFF'
    TestCase {
        text: Text::parse_bytes(
            &[0xF2, 0x80, 0x80, 0x80, 0xF4, 0x8F, 0xBF, 0xBD, 0xEF, 0xBF, 0xBF, 0xFF]
        ),
        string_rep: "[U+080000, U+10FFFD, U+FFFF, 0xFF]",
        out: &[
            ("U+080000", "?", "UNKNOWN CHARACTER"),
            ("U+10FFFD", "▨", "RESERVED FOR PRIVATE USE"),
            ("U+FFFF", "∅", "NOT A CHARACTER"),
            ("0xFF", "�", "INVALID UTF-8"),
        ],
    }
    .run()
}
