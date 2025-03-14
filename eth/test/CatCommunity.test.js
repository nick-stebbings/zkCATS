const { expect } = require("chai");
const hre = require("hardhat");
const { Wallet, Provider } = require("zksync-ethers");
const { Deployer } = require("@matterlabs/hardhat-zksync");
require("dotenv").config();

const RICH_WALLET_PK =
  "0x3d3cbc973389cb26f657686445bcc75662b415b656078503592ac8c1abb8810e";
const MEMBER_1_PK =
  "0x509ca2e9e6acf0ba086477910950125e698d4ea70fa6f63e000c5a22bda9361c";
// const MEMBER_2_PK = "0x71781d3a358e7a65150e894264ccc594993fbc0ea12d69508a340bc1d4f5bfbc";
const NON_MEMBER_PK =
  "0x379d31d4a7031ead87397f332aab69ef5cd843ba3898249ca1046633c0c7eefe";

describe("CatCommunity", function () {
  let contract;
  let member1;
  // let member2;
  let nonMember;
  let provider;

  beforeEach(async function () {
    provider = new Provider("http://127.0.0.1:8011");

    const wallet = new Wallet(RICH_WALLET_PK, provider);
    const deployer = new Deployer(hre, wallet);
    const artifact = await deployer.loadArtifact("CatCommunity");

    contract = await deployer.deploy(artifact, [wallet.address]);
    console.log("Contract deployed to:", await contract.getAddress()); // Debug log
  });

  describe("Member Management", function () {
    it("Should allow owner to register a member", async function () {
      member1 = new Wallet(MEMBER_1_PK, provider);

      const tx = await contract.registerMember(member1.address);
      await tx.wait();
      expect(await contract.isMember(await member1.address)).to.be.true;
    });

    it("Should emit MemberRegistered event", async function () {
      member1 = new Wallet(MEMBER_1_PK, provider);

      const tx = await contract.registerMember(member1.address);
      await expect(tx)
        .to.emit(contract, "MemberRegistered")
        .withArgs(member1.address);
    });

    it("Should allow owner to remove a member", async function () {
      member1 = new Wallet(MEMBER_1_PK, provider);

      const tx1 = await contract.registerMember(member1.address);
      await tx1.wait();

      const tx2 = await contract.removeMember(member1.address);
      await tx2.wait();
      expect(await contract.isMember(member1.address)).to.be.false;
    });

    it("Should emit MemberRemoved event", async function () {
      member1 = new Wallet(MEMBER_1_PK, provider);
      const tx1 = await contract.registerMember(member1.address);
      await tx1.wait();

      const tx2 = await contract.removeMember(member1.address);
      await expect(tx2)
        .to.emit(contract, "MemberRemoved")
        .withArgs(member1.address);
    });

    it("Should prevent non-owners from registering members", async function () {
      nonMember = new Wallet(NON_MEMBER_PK, provider);
      await expect(
        contract.connect(nonMember).registerMember(nonMember.address)
      )
        .to.be.revertedWithCustomError(contract, "OwnableUnauthorizedAccount")
        .withArgs(nonMember.address);
    });

    it("Should expose isMember with value true for a member", async function () {
      member1 = new Wallet(MEMBER_1_PK, provider);
      const tx1 = await contract.registerMember(member1.address);
      await tx1.wait();

      await expect(await contract.connect(member1).isMember(member1.address)).to
        .be.true;
    });

    it("Should expose isMember with value false for a non-member", async function () {
      nonMember = new Wallet(NON_MEMBER_PK, provider);
      await expect(
        await contract.connect(nonMember).isMember(nonMember.address)
      ).to.be.false;
    });
  });
});
