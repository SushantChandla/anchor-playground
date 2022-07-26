use anchor_lang::{system_program, solana_program::{self}, prelude::Pubkey};
use solana_program_test::{ProgramTest, processor, tokio};
use solana_sdk::{account::Account, instruction::Instruction, signature::Keypair, transaction::Transaction, signer::Signer};
use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;

#[tokio::test]
async fn testing_initialize() {
    let mut program = ProgramTest::new(
        "anchor playground",
        playground::id(),
        // the entrypoint function is generated by the #[program] macro
        processor!(playground::entry),
    );

    let system_program = system_program::ID;
    let token_program_id = anchor_spl::token::ID;
    let rent = solana_program::sysvar::rent::ID;
    let associated_token_program_id = anchor_spl::associated_token::ID;
    println!(
        "{} / {} / {} / {}",
        system_program, token_program_id, rent, associated_token_program_id
    );
    
    let owner = Keypair::new();

    program.add_account(
        owner.pubkey(),
        Account {
            lamports: 1_000_000_000,
            ..Account::default()
        },
    );
    let global_state = Pubkey::find_program_address(&[b"seed"], &playground::id()).0;

    program.add_account(
        owner.pubkey(),
        Account {
            lamports: 1_000_000_000,
            ..Account::default()
        },
    );

    let initalize_ix =Instruction {
        program_id: playground::ID,
        data: playground::instruction::Initialize{}
        .data(),
        accounts: playground::accounts::Initialize {
            owner: owner.pubkey(),
            global_state,
            rent,
            system_program,
        }
        .to_account_metas(None),
    };
    let mut program_context = program.start_with_context().await;

    let mut initalize_tx = Transaction::new_with_payer(&[initalize_ix],Some(&owner.pubkey()));
    let recent_blockhash = program_context.last_blockhash.clone();

    initalize_tx.partial_sign(&[&owner], recent_blockhash);
    program_context
        .banks_client
        .process_transaction(initalize_tx)
        .await
        .unwrap();
}