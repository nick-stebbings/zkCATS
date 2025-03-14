// SPDX-License-Identifier: MIT

pragma solidity ^0.8.24;
import "@openzeppelin/contracts/access/Ownable.sol";

contract CatCommunity is Ownable {
    mapping(address => bool) public members;
    
    event MemberRegistered(address member);
    event MemberRemoved(address member);
    
    constructor(address initialOwner) Ownable(initialOwner) {}

    function registerMember(address member) external onlyOwner {
        members[member] = true;
        emit MemberRegistered(member);
    }
    
    function removeMember(address member) external onlyOwner {
        members[member] = false;
        emit MemberRemoved(member);
    }
    
    function isMember(address addr) external view returns (bool) {
        return members[addr];
    }
}
