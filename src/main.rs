use std::env;

use fast_math::log2;
use owo_colors::OwoColorize;
use primes::is_prime;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 { println!("{} Wrong number of arguments.", "ERROR".bright_red()); return; }
    args.iter().skip(1).for_each(|arg| {
        match arg.parse::<u64>() {
            Ok(n) => translate_and_print(n),
            Err(_) => parse_and_print(arg.to_string())
        }
    });
}

fn parse_and_print(tx: String) {
    match is_valid_tx_chars(tx.clone()) {
        Ok(_) => (),
        Err(e) => {
            println!("{} {}", "ERROR".bright_red(), e);
            return;
        }
    }
    match parse_tx(tx.clone()) {
        Ok(res) => println!("{} -> {}", tx, res),
        Err(e) => println!("{} {}", "ERROR".bright_red(), e)
    }
}

fn is_valid_tx_chars(s: String) -> Result<(), String> {
    for c in s.chars() {
        if c != ':' && c != '(' && c != ')' && c != '-' && c != 'P' {
            return Err("Invalid character(s). If this was meant to be numerical it failed to parse. Make sure it fits in a 64-bit unsigned integer.".to_string());
        }
    }
    Ok(())
}

fn translate_and_print(n: u64) {
    let tx = translate(n);
    match tx {
        Ok(tx) => println!("{} -> {}", n, tx.codegen()),
        Err(e) => println!("{}", e),
    }
}

fn translate(n: u64) -> Result<Tx, String> {
    if n == 0 {
        Ok(Tx { members: vec!{TxNode::Bracketed(
            Tx {
                members: vec!{
                    TxNode::Bracketed(
                        Tx {
                            members: vec!{TxNode::MinusP}
                        }
                    )
                }
            }
        ), TxNode::Colon} })
    } else if n == 1 {
        Ok(Tx { members: vec!{TxNode::Bracketed(
            Tx {
                members: vec!{TxNode::MinusP}
            }
        ), TxNode::Colon} })
    } else {
        find_tx(n)
    }
}

fn find_tx(n: u64) -> Result<Tx, String> {
    if is_power_of_two(n) {
        return Ok(Tx { members: create_colon_list(log2(n as f32) as u64) });
    } else if is_prime(n) {
        let inner = get_prime_index(n);
        let inner_tx = find_tx(inner).unwrap();
        return Ok(Tx { members: vec![TxNode::Bracketed(inner_tx)] });
    } else {
        let fac = find_first_prime_factor(n);
        let mut a = find_tx(fac).unwrap().members;
        let b = find_tx(n/fac).unwrap().members;
        a.extend(b);
        return Ok(Tx { members: a });
    }
}

type TxNodeList = Vec<TxNode>;
struct Tx {
    members: TxNodeList,
}

enum TxNode {
    Colon,
    Bracketed(Tx),
    MinusP
}

impl Tx {
    fn codegen(&self) -> String {
        let mut code = String::new();
        for member in &self.members {
            code.push_str(&member.codegen());
        }
        code
    }
}

fn parse_tx(tx: String) -> Result<u64, String> {
    let mut child_call_buffer = "".to_string();
    let mut n = 1;
    let mut p: i64 = 0;
    let mut brack_depth = 0;
    for c in tx.chars(){
        if c == ')' {
            brack_depth -= 1;
            if brack_depth == 0 {
                run_recusivly_and_flush_buffer(&mut child_call_buffer, &mut p, &mut n)?;
            }
        }
        if brack_depth > 0 {
            child_call_buffer.push(c);
        } else if c == ':' {
            n <<= 1;
        } else if c == 'P' || c == '-' {
            return Err(format!{"Found unexpected \"{}\".", c});
        }
        if c == '(' {
            brack_depth += 1
        } 
    }
    if brack_depth != 0 {
        return Err("Unbalanced brackets.".to_string());
    }
    let r = n as i128 + p as i128;
    if r < 0 {
        println!("{} {}", "INFO".yellow(), "This number may have been negative and resulted in a binary overflow.");
    }
    Ok(r as u64)
}

fn run_recusivly_and_flush_buffer(child_call_buffer: &mut String, p: &mut i64, n: &mut u64) -> Result<(), String> {
    Ok(if *child_call_buffer == "-P" {
        *p -= 1;
    } else if *child_call_buffer == "(-P)" {
        *p -= 2
    } else {
        let child = parse_tx(child_call_buffer.clone())?;
        *child_call_buffer = "".to_string();
        *n *= nth_prime(child);
    })
}

impl TxNode {
    fn codegen(&self) -> String {
        match self {
            TxNode::Colon => ":".to_string(),
            TxNode::Bracketed(tx) => format!("({})", tx.codegen()),
            TxNode::MinusP => "-P".to_string()
        }
    }
}

fn create_colon_list(n: u64) -> TxNodeList {
    let mut members = Vec::new();
    for _ in 0..n {
        members.push(TxNode::Colon);
    }
    members
}

fn is_power_of_two(n: u64) -> bool {
    n != 0 && (n & (n - 1)) == 0
}

fn get_prime_index(n: u64) -> u64 {
    let mut i = 1;
    let mut p = 2;
    while p < n {
        p = next_prime(p);
        i += 1;
    }
    i
}

fn nth_prime(n: u64) -> u64 {
    let mut i = 0;
    let mut p = 0;
    while i < n {
        p = next_prime(p);
        i += 1;
    }
    p
}

fn next_prime(n: u64) -> u64 {
    let mut p = n + 1;
    while !is_prime(p) {
        p += 1;
    }
    p
}

fn find_first_prime_factor(n: u64) -> u64 {
    let mut p = 2;
    let mut n = n;
    while n > 1 {
        let mut count = 0;
        while n % p == 0 {
            count += 1;
            n /= p;
        }
        if count > 0 {
            return p;
        }
        p = next_prime(p);
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symetrical_translations() {
        for n in 0..1_000 {
            let tx = translate(n).unwrap();
            let n2 = parse_tx(tx.codegen()).unwrap();
            assert_eq!(n, n2);
        }
    }

    static KNOWN_TABLE: [(&str, u64); 13] = [
        ("((-P)):", 0),
        ("(-P):", 1),
        (":", 2),
        ("(:)", 3),
        ("::", 4),
        ("((:))", 5),
        (":(:)", 6),
        ("(::)", 7),
        (":::", 8),
        ("(:)(:)", 9),
        (":((:))", 10),
        ("(((:)))", 11),
        ("::::::((:))((:))((:))((:))((:))((:))", 1000000)
    ];

    #[test]
    fn known_translations() {
        for (tx, n) in KNOWN_TABLE.iter() {
            let tx2 = translate(*n).unwrap();
            assert_eq!(tx2.codegen(), *tx);
            let n2 = parse_tx(tx.to_string()).unwrap();
            assert_eq!(*n, n2);
        }
    }

    #[test]
    fn powers_of_two() {
        let mut n = 2;
        for i in 1..30 {
            let tx = translate(n).unwrap().codegen();
            tx.chars().for_each(|c| {
                if c != ':' {
                    panic!("Power of 2 (harmonic) had none colon character.");
                }
            });
            if tx.len() != i {
                panic!("Power of 2 (harmonic) was not correct.");
            }
            n <<= 1;
        }
    }
}