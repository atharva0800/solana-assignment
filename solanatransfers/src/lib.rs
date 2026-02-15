mod pb;
use pb::solana_transfers_v1 as transfers;
use substreams_solana::pb::sf::solana::r#type::v1 as pb_solana;

#[substreams::handlers::map]
fn map_transfers(block: pb_solana::Block) -> Result<transfers::Transfers, substreams::errors::Error> {
    let mut out = transfers::Transfers::default();

    for trx in block.transactions {
        if let Some(transaction) = trx.transaction {
            let msg = transaction.message.unwrap();
            let accounts = &msg.account_keys;

            for inst in msg.instructions {
                let program_id = &accounts[inst.program_id_index as usize];
                
                // 1. Identify System Program (Transfer)
                if bs58::encode(program_id).into_string() == "11111111111111111111111111111111" {
                    
                    // 2. Simple manual decode for SOL Transfer (Instruction index 2)
                    if inst.data.len() >= 4 && inst.data[0] == 2 {
                        // Extract lamports from bytes 4 to 12
                        let mut amount_bytes = [0u8; 8];
                        if inst.data.len() >= 12 {
                            amount_bytes.copy_from_slice(&inst.data[4..12]);
                            let lamports = u64::from_le_bytes(amount_bytes);

                            out.transfers.push(transfers::Transfer {
                                from: bs58::encode(&accounts[inst.accounts[0] as usize]).into_string(),
                                to: bs58::encode(&accounts[inst.accounts[1] as usize]).into_string(),
                                amount: lamports,
                            });
                        }
                    }
                }
            }
        }
    }
    Ok(out)
}