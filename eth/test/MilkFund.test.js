const { expect } = require("chai");
const hre = require("hardhat");
const { Wallet, Provider } = require("zksync-ethers");
const { Deployer } = require("@matterlabs/hardhat-zksync");
require("dotenv").config();

const RICH_WALLET_PK = "0x3d3cbc973389cb26f657686445bcc75662b415b656078503592ac8c1abb8810e";
const MEMBER_1_PK = "0x509ca2e9e6acf0ba086477910950125e698d4ea70fa6f63e000c5a22bda9361c";
const NON_MEMBER_PK = "0x379d31d4a7031ead87397f332aab69ef5cd843ba3898249ca1046633c0c7eefe";

const RECIPIENT_1_PK = "0x105de4e75fe465d075e1daae5647a02e3aad54b8d23cf1f70ba382b9f9bee839";
const RECIPIENT_2_PK = "0x7becc4a46e0c3b512d380ca73a4c868f790d1055a7698f38fb3ca2b2ac97efbb";

describe("MilkFund", function () {
  let catCommunity;
  let milkFund;
  let owner;
  let member1;
  let nonMember;
  let recipient1;
  let recipient2;
  let provider;

  beforeEach(async function () {
    provider = new Provider(process.env.TESTNET_URL);

    owner = new Wallet(RICH_WALLET_PK, provider);
    member1 = new Wallet(MEMBER_1_PK, provider);
    nonMember = new Wallet(NON_MEMBER_PK, provider);
    recipient1 = new Wallet(RECIPIENT_1_PK, provider);
    recipient2 = new Wallet(RECIPIENT_2_PK, provider);

    const deployer = new Deployer(hre, owner);
    
    // Deploy CatCommunity first
    const catArtifact = await deployer.loadArtifact("CatCommunity");
    catCommunity = await deployer.deploy(catArtifact, [owner.address]);
    
    // Register member1
    const tx = await catCommunity.registerMember(member1.address);
    await tx.wait();

    // Deploy MilkFund
    const milkArtifact = await deployer.loadArtifact("MilkFund");
    milkFund = await deployer.deploy(milkArtifact, [await catCommunity.getAddress()]);
    
    console.log("MilkFund deployed to:", await milkFund.getAddress());
  });

  describe("Simple Fund Management", function () {
    it("Should allow member to create a fund", async function () {
      const addresses = [recipient1.address, recipient2.address];
      const percentages = [5000, 5000]; // 50% each

      const tx = await milkFund.connect(member1).createFund(addresses, percentages);
      await tx.wait();

      expect(await milkFund.active()).to.be.true;
    });

    it("Should emit FundCreated event", async function () {
      const addresses = [recipient1.address, recipient2.address];
      const percentages = [5000, 5000];

      await expect(milkFund.connect(member1).createFund(addresses, percentages))
        .to.emit(milkFund, "FundCreated");
    });

    it("Should prevent non-members from creating a fund", async function () {
      const addresses = [recipient1.address, recipient2.address];
      const percentages = [5000, 5000];

      await expect(
        milkFund.connect(nonMember).createFund(addresses, percentages)
      ).to.be.revertedWith("Not a member");
    });

    it("Should prevent creating a fund when one already exists", async function () {
      const addresses = [recipient1.address, recipient2.address];
      const percentages = [5000, 5000];

      await milkFund.connect(member1).createFund(addresses, percentages);
      
      await expect(
        milkFund.connect(member1).createFund(addresses, percentages)
      ).to.be.revertedWith("Fund already exists");
    });

    it("Should prevent creating a fund with mismatched arrays", async function () {
      const addresses = [recipient1.address, recipient2.address];
      const percentages = [5000]; // Only one percentage

      await expect(
        milkFund.connect(member1).createFund(addresses, percentages)
      ).to.be.revertedWith("Length mismatch");
    });

    it("Should prevent creating a fund with incorrect total percentage", async function () {
      const addresses = [recipient1.address, recipient2.address];
      const percentages = [3000, 3000]; // Only 60% total

      await expect(
        milkFund.connect(member1).createFund(addresses, percentages)
      ).to.be.revertedWith("Percentages must total 100.00%");
    });

    it("Should correctly return fund details", async function () {
      const addresses = [recipient1.address, recipient2.address];
      const percentages = [5000, 5000];

      await milkFund.connect(member1).createFund(addresses, percentages);

      const [returnedAddresses, returnedPercentages] = await milkFund.getFundDetails();
      
      expect(returnedAddresses).to.deep.equal(addresses);
      expect(returnedPercentages.map(p => Number(p))).to.deep.equal(percentages);
    });
  });

  describe("Complex Fund Scenarios", function () {
    it("Should handle larger arrays of recipients", async function () {
      const addresses = [
        recipient1.address,
        recipient2.address,
        owner.address,
        member1.address,
        nonMember.address
      ];
      const percentages = [2000, 2000, 2000, 2000, 2000]; // 20% each

      const tx = await milkFund.connect(member1).createFund(addresses, percentages);
      await tx.wait();

      const [returnedAddresses, returnedPercentages] = await milkFund.getFundDetails();
      expect(returnedAddresses).to.deep.equal(addresses);
      expect(returnedPercentages.map(p => Number(p))).to.deep.equal(percentages);
    });

    it("Should handle uneven percentage distributions", async function () {
      const addresses = [recipient1.address, recipient2.address, owner.address];
      const percentages = [1234, 3333, 5433]; // 12.34%, 33.33%, 54.33%

      const tx = await milkFund.connect(member1).createFund(addresses, percentages);
      await tx.wait();

      const [returnedAddresses, returnedPercentages] = await milkFund.getFundDetails();
      expect(returnedPercentages.map(p => Number(p))).to.deep.equal(percentages);
    });

    it("Should reject when percentages are slightly over 100%", async function () {
      const addresses = [recipient1.address, recipient2.address];
      const percentages = [5001, 5000]; // 100.01%

      await expect(
        milkFund.connect(member1).createFund(addresses, percentages)
      ).to.be.revertedWith("Percentages must total 100.00%");
    });

    it("Should reject when percentages are slightly under 100%", async function () {
      const addresses = [recipient1.address, recipient2.address];
      const percentages = [4999, 5000]; // 99.99%

      await expect(
        milkFund.connect(member1).createFund(addresses, percentages)
      ).to.be.revertedWith("Percentages must total 100.00%");
    });

    it("Should reject with many recipients not totaling 100%", async function () {
      const addresses = [
        recipient1.address,
        recipient2.address,
        owner.address,
        member1.address,
        nonMember.address
      ];
      const percentages = [2000, 2000, 2000, 2000, 1999]; // 99.99%

      await expect(
        milkFund.connect(member1).createFund(addresses, percentages)
      ).to.be.revertedWith("Percentages must total 100.00%");
    });

    it("Should handle precise percentage distributions", async function () {
      const addresses = [
        recipient1.address,
        recipient2.address,
        owner.address,
        member1.address
      ];
      const percentages = [1111, 2222, 3333, 3334]; // 11.11%, 22.22%, 33.33%, 33.34%

      const tx = await milkFund.connect(member1).createFund(addresses, percentages);
      await tx.wait();

      const [returnedAddresses, returnedPercentages] = await milkFund.getFundDetails();
      expect(returnedPercentages.map(p => Number(p))).to.deep.equal(percentages);
    });
  });
});