use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    /// number of greetings
    pub name: String,
}

const DUMMY_STRING: &str = "0000000000000000000000000000000000000000000";

pub fn init_greeting_account() -> GreetingAccount {
    GreetingAccount {
        name: String::from(DUMMY_STRING),
    }
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    msg!("Hello World Rust program entrypoint");

    // Iterating accounts is safer then indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    msg!("Successfully passed account.owner");

    msg!(
        "Successfully passed account.data {}",
        account.data.borrow().len()
    );

    // Increment and store the number of times the account has been greeted
    let mut greeting_account = match GreetingAccount::try_from_slice(&account.data.borrow_mut()) {
        Ok(data) => {
            msg!("Successfully reached account.data {:?}", data.name);
            data
        }
        Err(err) => {
            msg!("Failed to deserialize greeting account {}", err);
            init_greeting_account()
        }
    };

    msg!(
        "Successfully got greeting account {:?}",
        greeting_account.name
    );

    let instruction_data_message =
        GreetingAccount::try_from_slice(instruction_data).map_err(|err| {
            msg!(
                "Attempt to deserialize instruction data has failed. {:?}",
                err
            );
            ProgramError::InvalidInstructionData
        })?;

    msg!(
        "Successfully got  instruction_data_message {}",
        instruction_data_message.name
    );

    greeting_account.name = instruction_data_message.name;

    greeting_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    msg!("The new Greeting is {}!", greeting_account.name);

    Ok(())
}

// Sanity tests
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;

    #[test]
    fn test_sanity() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let message = init_greeting_account();
        let mut data = message.try_to_vec().unwrap();
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );

        let greeting = GreetingAccount {
            name: String::from("abcdefghijabcdefghijabcdefghijabcdefghijabc"),
        };

        let instruction_data = greeting.try_to_vec().unwrap();
        let accounts = vec![account];

        process_instruction(&program_id, &accounts, &instruction_data).unwrap();

        let greeting_recieved =
            GreetingAccount::try_from_slice(&accounts[0].data.borrow()).unwrap();

        assert_eq!(greeting_recieved.name.eq(&greeting.name), true);

    }
}
