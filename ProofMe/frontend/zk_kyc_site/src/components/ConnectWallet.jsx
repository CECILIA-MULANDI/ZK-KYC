import { ethers } from "ethers";

const ConnectWallet = async () => {
  try {
    if (!window.ethereum) {
      alert("Please install Metamask  to be able to connect to your wallet!");
      return null;
    }
    const provider = new ethers.providers.Web3Provider(window.ethereum);
    const accounts = await provider.send("eth_requestAccounts", []);
    if (accounts.length === 0) {
      console.log("No accounts found!");
      return null;
    }
    const signer = provider.getSigner();
    const address = await signer.getAddress();
    console.log(`Connected wallet: ${address}`);
    return signer;
  } catch (error) {
    console.error("Wallet connection failed:", error);
    alert("Failed to connect wallet. Please try again.");
    return null;
  }
  // check if metamask is install
};

export default ConnectWallet;
