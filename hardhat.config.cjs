/** @type import('hardhat/config').HardhatUserConfig */
require("@matterlabs/hardhat-zksync-solc");
require("@matterlabs/hardhat-zksync-deploy");
require("@matterlabs/hardhat-zksync");
require("@nomicfoundation/hardhat-toolbox");
require('dotenv').config();


module.exports = {
  zksolc: {
    version: "latest",
    compilerSource: "binary",
    settings: {},
  },
  defaultNetwork: "zkSyncTestnet",
  networks: {
    zkSyncTestnet: {
      url: "https://sepolia.era.zksync.dev",
      ethNetwork: "sepolia",
      zksync: true,
      // verifyURL:
      //   "https://explorer.sepolia.era.zksync.dev/contract_verification",
      accounts: process.env.WALLET_PRIVATE_KEY
        ? [process.env.WALLET_PRIVATE_KEY]
        : [],
    },
  },
  solidity: {
    version: "0.8.24"
  },
  hardhat: {
    zksync: true,
  },
  paths: {
    sources: "./eth/contracts",
    artifacts: "./eth/artifacts-zk",
    cache: "./eth/cache-zk",
    tests: "./eth/test",
  }
};