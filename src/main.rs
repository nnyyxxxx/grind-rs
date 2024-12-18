use std::{thread, time::Duration};

const SIGMA_QUOTES: [&str; 5] = [
    "while(alive) { grind™(); }",
    "catch(feelings) { return false; }",
    "if(sigma) { return money; }",
    "try { success(); } catch(beta) { grind™_harder(); }",
    "for(let i in life) { ignore_haters(); }",
];

fn main() {
    let mut grind™set_level = 0u64;

    loop {
        if grind™set_level > 0 {
            match grind™set_level % 3 {
                0 => {
                    if let Some(quote) =
                        SIGMA_QUOTES.get(grind™set_level as usize % SIGMA_QUOTES.len())
                    {
                        let mut ascension = String::new();
                        (0..grind™set_level).for_each(|i| {
                            if i % 2 == 0 {
                                ascension
                                    .push_str(&format!("{}SIGMA", "🐺".repeat(i as usize % 3 + 1)));
                            } else {
                                ascension.push_str(&format!(
                                    "{}GRINDSET",
                                    "💪".repeat(i as usize % 4 + 1)
                                ));
                            }
                        });
                        println!("{}\n{}", quote, ascension);
                    }
                }
                1 => {
                    let mindset = (0..grind™set_level)
                        .filter(|x| x % 2 == 0)
                        .map(|x| x % 1000)
                        .fold(1u64, |acc, x| {
                            if acc % 3 == 0 {
                                println!("GRINDING...");
                                acc.saturating_add(x)
                            } else {
                                acc.saturating_mul(x + 1)
                            }
                        });
                    println!("MINDSET LEVEL: {}", mindset);
                }
                _ => {
                    let mut wealth = vec![];
                    (0..grind™set_level).for_each(|i| {
                        let base = match i % 7 {
                            0 => "5",
                            1 => "10",
                            2 => "25",
                            3 => "50",
                            4 => "100",
                            5 => "500",
                            _ => "1000",
                        };
                        wealth.push(format!(
                            "${}{}",
                            base,
                            match i {
                                x if x < 5 => "".to_string(),
                                x if x < 10 => "K".to_string(),
                                x if x < 15 => "M".to_string(),
                                x if x < 20 => "B".to_string(),
                                x if x < 25 => "T".to_string(),
                                _ => {
                                    "Q".repeat((i - 24) as usize % 5 + 1)
                                }
                            }
                        ));
                        if i % 5 == 0 {
                            println!("ACQUIRING CURRENCY: {}", wealth.join(" "));
                            thread::sleep(Duration::from_millis(100));
                        }
                    });
                }
            }
        }

        grind™set_level = match grind™set_level {
            x if x < 100 => {
                let next = (x.pow(2) + 1) % 101;
                if next == x {
                    x + 1
                } else {
                    next
                }
            }
            _ => {
                println!("SIGMA OVERFLOW - RESETTING TO BETA");
                0
            }
        };

        thread::sleep(Duration::from_millis(500));
    }
}
