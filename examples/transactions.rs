// Copyright 2023, Igor Shaula
// Licensed under the MIT License <LICENSE or
// http://opensource.org/licenses/MIT>. This file
// may not be copied, modified, or distributed
// except according to those terms.
use std::io;
use winreg2::enums::*;
use winreg2::transaction::Transaction;
use winreg2::RegKey;

fn main() -> io::Result<()> {
    let t = Transaction::new()?;
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (key, _disp) = hkcu.create_subkey_transacted("Software\\RustTransaction", &t)?;
    key.set_value("TestQWORD", &1_234_567_891_011_121_314u64)?;
    key.set_value("TestDWORD", &1_234_567_890u32)?;

    println!("Commit transaction? [y/N]:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    input = input.trim_end().to_owned();
    if input == "y" || input == "Y" {
        t.commit()?;
        println!("Transaction committed.");
    } else {
        // this is optional, if transaction wasn't committed,
        // it will be rolled back on disposal
        t.rollback()?;

        println!("Transaction wasn't committed, it will be rolled back.");
    }

    Ok(())
}
