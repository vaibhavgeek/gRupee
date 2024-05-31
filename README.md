# Creating Digital Rupee Stablecoin with Stellar 
We are using onramp.money to create a digital rupee stablecoin which is overcollaterlised by USDC to allow seamless capital flow of global rupee in the global markets. 
## Goals

- [x] Create a smart contract that deposits and gives gINR 
- [x] Allow people to withdraw gINR to their frieghter wallet 
- [x] Burn USDC equivalent from the repository
- [x] Create frontend to execute deposit and withdraw functions 
- [ ] Create a landing page
- [ ] Deploy on Mainnet 
- [ ] Ensure compliance and regulatory feasibility 

### Run this locally

Install Rust and other packages from the instructions mentioned 
(here)[https://developers.stellar.org/docs/smart-contracts/getting-started/setup]

To deploy the contracts see `scripts/exec.sh`

To run the entire dapp, run 

- `cp .env.example .env`
- `npm install . `
- `npm init`
- `npm run dev`


# Soroban Frontend in Astro

A Frontend Template suitable for use with `soroban contract init --frontend-template`, powered by [Astro](https://astro.build/).

