soroban contract build 

soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/soroban_grupee_contract.wasm \
  --network testnet --source vaibhav

soroban contract invoke \
  --id CDWMGTUQRSEG2MLZE75CBLZGIP2T2CMEKR4NATOAGIFXLBS44N2NGS5A \
  --network testnet \
  --source vaibhav \
  -- \
  initialize \
  --admin vaibhav \
  --token_wasm_hash 1d7515989335d6974948a295e76509ad5625bf168f2e69a0d7b9cc7b41c6cb43 \
  --token CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC

  soroban contract invoke \
  --id CDWMGTUQRSEG2MLZE75CBLZGIP2T2CMEKR4NATOAGIFXLBS44N2NGS5A \
  --network testnet \
  --source vaibhav \
  -- \
  deposit \
  --contributor vaibhav \
  --token CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC \
  --amount 10000000 \
  --exchange_price 83

  soroban contract invoke \
  --id CDWMGTUQRSEG2MLZE75CBLZGIP2T2CMEKR4NATOAGIFXLBS44N2NGS5A \
  --source vaibhav \
  --network testnet \
  -- \
  get_share_token_balance \
  --user vaibhav

  soroban contract invoke \
  --id CDWMGTUQRSEG2MLZE75CBLZGIP2T2CMEKR4NATOAGIFXLBS44N2NGS5A \
  --network testnet \
  --source vaibhav \
  -- \
  withdraw \
  --contributor vaibhav \
  --recipient vaibhav \
  --token CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC

  soroban contract invoke \
  --id CDWMGTUQRSEG2MLZE75CBLZGIP2T2CMEKR4NATOAGIFXLBS44N2NGS5A \
  --source vaibhav \
  --network testnet \
  -- \
  get_share_token_balance \
  --user vaibhav
  
  soroban contract invoke \
  --id CDWMGTUQRSEG2MLZE75CBLZGIP2T2CMEKR4NATOAGIFXLBS44N2NGS5A \
  --source vaibhav \
  --network testnet \
  -- \
  get_share_token 

  soroban contract bindings typescript \
  --network testnet \
  --contract-id CDWMGTUQRSEG2MLZE75CBLZGIP2T2CMEKR4NATOAGIFXLBS44N2NGS5A \
  --output-dir packages/grup --overwrite