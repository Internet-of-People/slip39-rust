use failure::Fallible;
use sssmc39::*;

fn main()-> Fallible<()> {
    let threshold = 2;
    let groups = generate_mnemonics_random(threshold, &vec![(1, 1), (1, 1), (3, 5), (2, 6)], 256, "morpheus", 4)?;
    println!("Secret ({}-of-{})", threshold, groups.len());
    for (g, group) in groups.iter().enumerate() {
        println!("  Group-{:02} ({}-of-{})", g + 1, group.member_threshold, group.member_shares.len());
        let mnemonics = group.mnemonic_list()?;
        for (m, mnemonic) in mnemonics.iter().enumerate() {
            println!("    Mnemonic-{:02}-{:02}", g + 1, m + 1);
            let mut i = 0;
            for row in mnemonic.as_slice().chunks(3) {
                print!("      ");
                for word in row {
                    i += 1;
                    let mut left = word.clone();
                    let right = left.split_off(4);
                    print!("{:02}:{}·{:·<4}  ", i, left, right);
                }
                print!("\n");
            }
        }
    }
    Ok(())
}