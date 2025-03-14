// SPDX-License-Identifier: MIT

pragma solidity ^0.8.24;
import "./CatCommunity.sol";

contract MilkFund {
    struct Recipient {
        address addr;
        uint256 percentage; // Based on 10000 (100.00%)
    }

    CatCommunity public community;
    Recipient[] public recipients;
    bool public active;
    
    event FundCreated(Recipient[] recipients);
    
    constructor(address _community) {
        community = CatCommunity(_community);
    }
    
    modifier onlyMember() {
        require(community.isMember(msg.sender), "Not a member");
        _;
    }
    
    function createFund(
        address[] calldata _addresses,
        uint256[] calldata _percentages
    ) external onlyMember {
        require(!active, "Fund already exists");
        require(_addresses.length == _percentages.length, "Length mismatch");
        
        uint256 total;
        for(uint i = 0; i < _addresses.length; i++) {
            recipients.push(Recipient({
                addr: _addresses[i],
                percentage: _percentages[i]
            }));
            total += _percentages[i];
        }
        
        require(total == 10000, "Percentages must total 100.00%");
        active = true;
        
        emit FundCreated(recipients);
    }
    
    function getFundDetails() external view returns (
        address[] memory addresses,
        uint256[] memory percentages
    ) {
        addresses = new address[](recipients.length);
        percentages = new uint256[](recipients.length);
        
        for(uint i = 0; i < recipients.length; i++) {
            addresses[i] = recipients[i].addr;
            percentages[i] = recipients[i].percentage;
        }
        
        return (addresses, percentages);
    }
}