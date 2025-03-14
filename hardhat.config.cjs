/** @type import('hardhat/config').HardhatUserConfig */
require("@matterlabs/hardhat-zksync-solc");
require("@matterlabs/hardhat-zksync-deploy");
require("@matterlabs/hardhat-zksync");
require("@nomicfoundation/hardhat-toolbox");
require("@nomicfoundation/hardhat-chai-matchers");
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
    anvilZKsync: {
      url: "http://127.0.0.1:8011",
      ethNetwork: "localhost", // anvil doesn't support eth node; removing this line will cause an error
      zksync: true,
      accounts: process.env.WALLET_PRIVATE_KEY
        ? [process.env.WALLET_PRIVATE_KEY]
        : [],
    },
    hardhat: {
      zksync: true,
    },
  },
  solidity: {
    version: "0.8.24"
  },
  paths: {
    sources: "./eth/contracts",
    artifacts: "./eth/artifacts-zk",
    cache: "./eth/cache-zk",
    tests: "./eth/test",
  }
};