#![deny(unsafe_code)]
#![allow(dead_code)]

//Superscript
pub fn super_panic(input: String) -> String {
    super_base(input, |c| { panic!("Can't convert Char: {} to superscript!", c.to_string())})
}

pub fn super_ignore_unable(input: String) -> String {
    super_base(input, |c| {c})
}

fn super_base(input: String, else_case: fn(char) -> char) -> String {
    let mut output: String = "".to_string();
    for char in input.chars() {
        let sub: char;
        match char {
            '0'=> {sub = '⁰' }
            '1'=> {sub = '¹' }
            '2'=> {sub = '²' }
            '3'=> {sub = '³' }
            '4'=> {sub = '⁴' }
            '5'=> {sub = '⁵' }
            '6'=> {sub = '⁶' }
            '7'=> {sub = '⁷' }
            '8'=> {sub = '⁸' }
            '9'=> {sub = '⁹' }
            'a'=> {sub = 'ᵃ' }
            'b'=> {sub = 'ᵇ' }
            'c'=> {sub = 'ᶜ' }
            'd'=> {sub = 'ᵈ' }
            'e'=> {sub = 'ᵉ' }
            'f'=> {sub = 'ᶠ' }
            'g'=> {sub = 'ᵍ' }
            'h'=> {sub = 'ʰ' }
            'i'=> {sub = 'ᶦ' }
            'j'=> {sub = 'ʲ' }
            'k'=> {sub = 'ᵏ' }
            'l'=> {sub = 'ˡ' }
            'm'=> {sub = 'ᵐ' }
            'n'=> {sub = 'ⁿ' }
            'o'=> {sub = 'ᵒ' }
            'p'=> {sub = 'ᵖ' }
            'r'=> {sub = 'ʳ' }
            's'=> {sub = 'ˢ' }
            't'=> {sub = 'ᵗ' }
            'u'=> {sub = 'ᵘ' }
            'v'=> {sub = 'ᵛ' }
            'w'=> {sub = 'ʷ' }
            'x'=> {sub = 'ˣ' }
            'y'=> {sub = 'ʸ' }
            'z'=> {sub = 'ᶻ' }
            'A'=> {sub = 'ᴬ' }
            'B'=> {sub = 'ᴮ' }
            'C'=> {sub = 'ᶜ' }
            'D'=> {sub = 'ᴰ' }
            'E'=> {sub = 'ᴱ' }
            'F'=> {sub = 'ᶠ' }
            'G'=> {sub = 'ᴳ' }
            'H'=> {sub = 'ᴴ' }
            'I'=> {sub = 'ᴵ' }
            'J'=> {sub = 'ᴶ' }
            'K'=> {sub = 'ᴷ' }
            'L'=> {sub = 'ᴸ' }
            'M'=> {sub = 'ᴹ' }
            'N'=> {sub = 'ᴺ' }
            'O'=> {sub = 'ᴼ' }
            'P'=> {sub = 'ᴾ' }
            'R'=> {sub = 'ᴿ' }
            'S'=> {sub = 'ˢ' }
            'T'=> {sub = 'ᵀ' }
            'U'=> {sub = 'ᵁ' }
            'V'=> {sub = 'ⱽ' }
            'W'=> {sub = 'ᵂ' }
            'X'=> {sub = 'ˣ' }
            'Y'=> {sub = 'ʸ' }
            'Z'=> {sub = 'ᶻ' }
            '\''=> {sub = '\'' }
            '('=> {sub = '⁽' }
            ')'=> {sub = '⁾' }
            '*'=> {sub = '*' }
            '+'=> {sub = '⁺' }
            '-'=> {sub = '⁻' }
            '='=> {sub = '⁼' }
            _ => { sub = else_case(char)}
        }
        output += &*sub.to_string();
    }
    output
}

//Subscript
pub fn sub(input: String) -> String {
    sub_base(input, |c| { panic!("Can't convert Char: {} to subscript!", c.to_string())})
}

pub fn sub_ignore_unable(input: String) -> String {
    sub_base(input, |c| {c})
}

fn sub_base(input: String, else_case: fn(char) -> char) -> String {
    let mut output: String = "".to_string();
        for char in input.chars() {
            let sub: char;
            match char {
                '0' => {sub = '₀'}
                    '1' => {sub = '₁'}
                '2' => {sub = '₂'}
                    '3' => {sub = '₃'}
                '4' => {sub = '₄'}
                    '5' => {sub = '₅'}
                '6' => {sub = '₆'}
                    '7' => {sub = '₇'}
                '8' => {sub = '₈'}
                    '9' => {sub = '₉'}
                'a' => {sub = 'ₐ'}
                    'b' => {sub = 'ᵦ'}
                'e' => {sub = 'ₑ'}
                    'h' => {sub = 'ₕ'}
                'i' => {sub = 'ᵢ'}
                    'j' => {sub = 'ⱼ'}
                'k' => {sub = 'ₖ'}
                    'l' => {sub = 'ₗ'}
                'm' => {sub = 'ₘ'}
                    'n' => {sub = 'ₙ'}
                'o' => {sub = 'ₒ'}
                    'p' => {sub = 'ₚ'}
                'r' => {sub = 'ᵣ'}
                    's' => {sub = 'ₛ'}
                't' => {sub = 'ₜ'}
                    'u' => {sub = 'ᵤ'}
                'v' => {sub = 'ᵥ'}
                    'x' => {sub = 'ₓ'}
                'y' => {sub = 'ᵧ'}
                    'A' => {sub = 'ₐ'}
                'H' => {sub = 'ₕ'}
                    'I' => {sub = 'ᵢ'}
                'J' => {sub = 'ⱼ'}
                    'K' => {sub = 'ₖ'}
                'L' => {sub = 'ₗ'}
                    'M' => {sub = 'ₘ'}
                'N' => {sub = 'ₙ'}
                    'O' => {sub = 'ₒ'}
                'P' => {sub = 'ₚ'}
                    'R' => {sub = 'ᵣ'}
                'S' => {sub = 'ₛ'}
                    'T' => {sub = 'ₜ'}
                'U' => {sub = 'ᵤ'}
                    'V' => {sub = 'ᵥ'}
                'X' => {sub = 'ₓ'}
                '(' => {sub = '₍'}
                ')' => {sub = '₎'}
                '+' => {sub = '₊'}
                '-' => {sub = '₋'}
                '=' => {sub = '₌'}
                _ => { sub = else_case(char)}
            }
            output += &*sub.to_string();
        }
    output
}