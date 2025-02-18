import { ethers } from "ethers";

const ConnectWallet = async () => {
  try {
    // Check if ethereum object exists
    if (typeof window.ethereum === "undefined") {
      alert("Please install MetaMask to connect your wallet!");
      return { success: false, signer: null, address: null };
    }

    try {
      // Request account access
      await window.ethereum.request({
        method: "eth_requestAccounts",
      });
    } catch (requestError) {
      // Handle user rejection or other request errors
      console.error("Account request error:", requestError);
      if (requestError.code === 4001) {
        alert("Please connect your MetaMask wallet to continue.");
      } else {
        alert("Failed to connect to MetaMask. Please try again.");
      }
      return { success: false, signer: null, address: null };
    }

    // Initialize provider after successful account request
    const provider = new ethers.providers.Web3Provider(window.ethereum);
    const signer = provider.getSigner();

    try {
      const address = await signer.getAddress();
      console.log(`Connected wallet: ${address}`);
      return { success: true, signer, address };
    } catch (addressError) {
      console.error("Error getting address:", addressError);
      alert("Failed to get wallet address. Please try again.");
      return { success: false, signer: null, address: null };
    }
  } catch (error) {
    console.error("Wallet connection failed:", error);
    alert("Failed to connect wallet. Please try again.");
    return { success: false, signer: null, address: null };
  }
};

export default ConnectWallet;
