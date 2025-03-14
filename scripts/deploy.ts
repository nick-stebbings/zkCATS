import { HardhatRuntimeEnvironment } from 'hardhat/types';
import { Wallet } from "zksync-ethers";
import { Deployer } from '@matterlabs/hardhat-zksync-deploy';
import * as hre from 'hardhat';

async function main() {
  const contractName = 'CatCommunity';

  // Get the private key from the environment
  const WALLET_PRIVATE_KEY = process.env.WALLET_PRIVATE_KEY || "";
  if (!WALLET_PRIVATE_KEY) {
    throw new Error("Please set WALLET_PRIVATE_KEY in your environment");
  }

  // Initialize the wallet
  const zkWallet = new Wallet(WALLET_PRIVATE_KEY);
  
  // Initialize the deployer with both required parameters
  const deployer = new Deployer(hre as unknown as HardhatRuntimeEnvironment, zkWallet);
  // Load the artifact
  const artifact = await deployer.loadArtifact(contractName);

  // Deploy the contract
  const contract = await deployer.deploy(artifact, [
    await deployer.zkWallet.getAddress()
  ]);

  // Show the contract info
  const contractAddress = await contract.getAddress();
  console.log(`${contractName} contract address: ${contractAddress}`);
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
