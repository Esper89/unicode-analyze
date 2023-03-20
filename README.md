# unicode-analyze

`unicode-analyze` is a simple command-line program for investigating strings of UTF-8 text.

## Examples

`unicode-analyze` shows the characters that make up a string:

```
$ unicode-analyze 'Hello, World!'
['H', 'e', 'l', 'l', 'o', ',', ' ', 'W', 'o', 'r', 'l', 'd', '!']
U+0048 'H' LATIN CAPITAL LETTER H
U+0065 'e' LATIN SMALL LETTER E
U+006C 'l' LATIN SMALL LETTER L
U+006C 'l' LATIN SMALL LETTER L
U+006F 'o' LATIN SMALL LETTER O
U+002C ',' COMMA
U+0020 ' ' SPACE
U+0057 'W' LATIN CAPITAL LETTER W
U+006F 'o' LATIN SMALL LETTER O
U+0072 'r' LATIN SMALL LETTER R
U+006C 'l' LATIN SMALL LETTER L
U+0064 'd' LATIN SMALL LETTER D
U+0021 '!' EXCLAMATION MARK
```

Control codes are replaced with abbreviations, to make sure the output displays correctly:

```
$ unicode-analyze $'\v\t\r\n'
[VT, HT, [CR + LF]]
U+000B VT LINE TABULATION
U+0009 HT CHARACTER TABULATION
U+000D CR CARRIAGE RETURN
U+000A LF LINE FEED
```

It can show you the [grapheme cluster](https://unicode.org/glossary/#extended_grapheme_cluster)s and
[scalar value](https://unicode.org/glossary/#unicode_scalar_value)s that make up a string, including
emojis:

```
$ unicode-analyze '👩‍👩‍👧‍👦 😵‍💫'
[['👩' + ZWJ + '👩' + ZWJ + '👧' + ZWJ + '👦'], ' ', ['😵' + ZWJ + '💫']]
U+01F469 '👩' WOMAN
U+200D ZWJ ZERO WIDTH JOINER
U+01F469 '👩' WOMAN
U+200D ZWJ ZERO WIDTH JOINER
U+01F467 '👧' GIRL
U+200D ZWJ ZERO WIDTH JOINER
U+01F466 '👦' BOY
U+0020 ' ' SPACE
U+01F635 '😵' DIZZY FACE
U+200D ZWJ ZERO WIDTH JOINER
U+01F4AB '💫' DIZZY SYMBOL
```

It displays diacritics and other combining characters on dotted circles (`U+25CC ◌`):

```
$ unicode-analyze m͌͊e̵͂o͐͝w͐̾
[['m' + '◌͌' + '◌͊'], ['e' + '◌̵' + '◌͂'], ['o' + '◌͐' + '◌͝◌'], ['w' + '◌͐' + '◌̾']]
U+006D 'm' LATIN SMALL LETTER M
U+034C '◌͌' COMBINING ALMOST EQUAL TO ABOVE
U+034A '◌͊' COMBINING NOT TILDE ABOVE
U+0065 'e' LATIN SMALL LETTER E
U+0335 '◌̵' COMBINING SHORT STROKE OVERLAY
U+0342 '◌͂' COMBINING GREEK PERISPOMENI
U+006F 'o' LATIN SMALL LETTER O
U+0350 '◌͐' COMBINING RIGHT ARROWHEAD ABOVE
U+035D '◌͝◌' COMBINING DOUBLE BREVE
U+0077 'w' LATIN SMALL LETTER W
U+0350 '◌͐' COMBINING RIGHT ARROWHEAD ABOVE
U+033E '◌̾' COMBINING VERTICAL TILDE
```

It displays right-to-left text correctly, in the order the text is stored in memory:

```
$ unicode-analyze اَلْعَرَبِيَّةُ
[['‎ا‎' + '◌َ'], ['‎ل‎' + '◌ْ'], ['‎ع‎' + '◌َ'], ['‎ر‎' + '◌َ'], ['‎ب‎' + '◌ِ'], ['‎ي‎' + '◌َ' + '◌ّ'], ['‎ة‎' + '◌ُ']]
U+0627 '‎ا‎' ARABIC LETTER ALEF
U+064E '◌َ' ARABIC FATHA
U+0644 '‎ل‎' ARABIC LETTER LAM
U+0652 '◌ْ' ARABIC SUKUN
U+0639 '‎ع‎' ARABIC LETTER AIN
U+064E '◌َ' ARABIC FATHA
U+0631 '‎ر‎' ARABIC LETTER REH
U+064E '◌َ' ARABIC FATHA
U+0628 '‎ب‎' ARABIC LETTER BEH
U+0650 '◌ِ' ARABIC KASRA
U+064A '‎ي‎' ARABIC LETTER YEH
U+064E '◌َ' ARABIC FATHA
U+0651 '◌ّ' ARABIC SHADDA
U+0629 '‎ة‎' ARABIC LETTER TEH MARBUTA
U+064F '◌ُ' ARABIC DAMMA
```

It even tells you what kind of invalid data you're looking at:

```
$ unicode-analyze $'\xF2\x80\x80\x80\xF4\x8F\xBF\xBD\xEF\xBF\xBF\xFF'
[U+080000, U+10FFFD, U+FFFF, 0xFF]
U+080000 ? UNKNOWN CHARACTER
U+10FFFD ▨ RESERVED FOR PRIVATE USE
U+FFFF ∅ NOT A CHARACTER
0xFF � INVALID UTF-8
```

## License

```
Copyright (C) 2023 Esper Thomson

This program is free software: you can redistribute it and/or modify
it under the terms of version 3 of the GNU Affero General Public License,
as published by the Free Software Foundation.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>.
```
