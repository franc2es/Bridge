// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

// 因为Solana链使用了某种跨链桥发送跨链请求，
// 我们将一个简单的Solidity合约与Solana上的跨链通信对接。
contract CrossChainNFT is ERC721, Ownable {
    uint256 public nextTokenId;
    address public solanaBridge; // Solana跨链桥的合约地址

    constructor(string memory name, string memory symbol) ERC721(name, symbol) {
        nextTokenId = 1; // 初始化token ID
    }

    // 用于接收来自Solana链的跨链请求
    function mintFromSolana(
        address to,
        uint256 solanaTokenId,
        string memory metadataURI
    ) external {
        require(msg.sender == solanaBridge, "Only Solana bridge can mint"); // 只允许Solana跨链桥调用

        uint256 tokenId = nextTokenId;
        nextTokenId++;

        _safeMint(to, tokenId); // 铸造NFT
        _setTokenURI(tokenId, metadataURI); // 设置NFT元数据

        // 记录跨链操作
        emit CrossChainMinted(solanaTokenId, tokenId, to, metadataURI);
    }

    // 设置跨链桥合约地址
    function setSolanaBridge(address _solanaBridge) external onlyOwner {
        solanaBridge = _solanaBridge;
    }

    // 用于记录跨链铸造事件
    event CrossChainMinted(uint256 solanaTokenId, uint256 ethTokenId, address to, string metadataURI);
}
