/* eslint-disable react/no-unescaped-entities */
import { useState } from "react";
import { CloudArrowUpIcon } from "@heroicons/react/24/solid";
import { ethers } from "ethers";
import ConnectWallet from "./ConnectWallet";

interface WalletData {
  isConnected: boolean;
  address: string | null;
  signer: ethers.Signer | null;
}

const Card = () => {
  const [isConnecting, setIsConnecting] = useState(false);
  const [currentStep, setCurrentStep] = useState(1);
  const [selectedFile, setSelectedFile] = useState<File | null>(null);
  const [walletData, setWalletData] = useState<WalletData>({
    isConnected: false,
    address: null,
    signer: null,
  });

  const steps = [
    { number: 1, title: "Upload", description: "Upload your documents" },
    { number: 2, title: "Generate Proof", description: "Create ZK proof" },
    { number: 3, title: "Verify", description: "Verify identity" },
  ];

  const handleConnect = async () => {
    if (isConnecting) return;

    try {
      setIsConnecting(true);
      const result = await ConnectWallet();

      if (result.success) {
        setWalletData({
          isConnected: true,
          address: result.address,
          signer: result.signer,
        });
        setCurrentStep(2); // Move to next step after wallet connection
        console.log("Wallet connected, ready for document upload");
      }
    } catch (error) {
      console.error("Connection handling error:", error);
      alert("An error occurred while connecting. Please try again.");
    } finally {
      setIsConnecting(false);
    }
  };
  const handleFileUpload = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (file) {
      const allowedTypes = [
        "application/pdf",
        "image/jpeg",
        "image/png",
        "image/jpg",
      ];
      if (!allowedTypes.includes(file.type)) {
        alert("Please select one of the supported types: PDF,JPEG or PNG");
        return;
      }
      //10MB
      const maxSize = 10 * 1024 * 1024;
      if (file.size > maxSize) {
        alert("File size must be less than 10MB");
        return;
      }
      setSelectedFile(file);
      console.log("File selected:", file.name);
    }
  };
  const triggerFileInput = () => {
    const fileInput = document.getElementById(
      "file-upload"
    ) as HTMLInputElement;
    if (fileInput) {
      fileInput.click();
    }
  };

  return (
    <div className="min-h-screen bg-gray-50">
      {/* Progress Bar Section */}
      <div className="w-full bg-white shadow-md py-6 px-4 fixed top-0 z-10">
        <div className="max-w-4xl mx-auto">
          <div className="flex justify-between">
            {steps.map((step) => (
              <div
                key={step.number}
                className="flex flex-col items-center relative"
              >
                <div
                  className={`w-8 h-8 sm:w-10 sm:h-10 rounded-full flex items-center justify-center
                    transition-colors duration-300
                    ${
                      currentStep >= step.number
                        ? "bg-black text-white"
                        : "bg-gray-200 text-gray-500"
                    }`}
                >
                  {step.number}
                </div>
                <div className="text-xs sm:text-sm mt-2 font-medium text-center">
                  {step.title}
                </div>
                <div className="text-xs text-gray-500 hidden sm:block text-center">
                  {step.description}
                </div>
                {step.number < steps.length && (
                  <div
                    className={`absolute left-1/2 top-4 sm:top-5 w-full h-0.5 
                      transition-colors duration-300
                      ${
                        currentStep > step.number ? "bg-black" : "bg-gray-200"
                      }`}
                    style={{ transform: "translateX(50%)" }}
                  />
                )}
              </div>
            ))}
          </div>
        </div>
      </div>

      {/* Main Card Section */}
      <div className="flex w-full items-center justify-center p-4 pt-50 sm:pt-40">
        <div
          className="flex flex-col bg-white w-full md:w-3/4 lg:w-1/2 max-w-2xl mx-auto 
                        min-h-[350px] shadow-lg px-4 sm:px-6 py-6 gap-4 
                        rounded-lg"
        >
          <h1 className="font-bold text-left text-black text-lg sm:text-xl lg:text-2xl">
            Zero-Knowledge KYC Verification
          </h1>

          <h3 className="text-gray-400 text-left text-sm sm:text-base">
            Privately verify your identity using zero-knowledge proofs
          </h3>

          <h2 className="mx-auto text-black text-lg sm:text-xl font-bold">
            Upload Identity Documents
          </h2>

          <h4 className="text-gray-400 text-left mx-auto text-sm sm:text-base">
            Supported formats: Passport, Driver's License, or National ID
          </h4>

          <div className="flex flex-col items-center gap-3 py-2">
            {/* Hidden file input */}
            <input
              id="file-upload"
              type="file"
              accept=".pdf,.jpg,.jpeg,.png"
              onChange={handleFileUpload}
              className="hidden"
            />

            {/* Clickable upload icon */}
            <div
              className="cursor-pointer hover:scale-110 transition-transform duration-200"
              onClick={triggerFileInput}
            >
              <CloudArrowUpIcon className="h-8 w-8 sm:h-10 sm:w-10 lg:h-12 lg:w-12 text-gray-500 hover:text-gray-700" />
            </div>

            {/* Show selected file name */}
            {selectedFile && (
              <div className="text-sm text-green-600 bg-green-50 px-3 py-1 rounded">
                📄 {selectedFile.name}
              </div>
            )}

            <button
              className={`bg-black flex items-center justify-center gap-2 p-2 
                         rounded text-white w-full sm:w-3/4 max-w-md
                         text-sm sm:text-base transition-all duration-200
                         ${
                           isConnecting
                             ? "opacity-50 cursor-not-allowed"
                             : "hover:bg-gray-800"
                         }`}
              onClick={handleConnect}
              disabled={isConnecting}
            >
              {isConnecting
                ? "Connecting..."
                : walletData.isConnected
                ? "Upload Documents"
                : "Connect Wallet"}
            </button>

            {walletData.isConnected && walletData.address && (
              <span className="text-xs sm:text-sm text-green-600">
                Connected: {walletData.address.slice(0, 6)}...
                {walletData.address.slice(-4)}
              </span>
            )}
          </div>

          <h4 className="text-gray-400 text-left text-xs sm:text-sm md:text-base">
            Your personal information is never stored or shared. Only
            zero-knowledge proofs are generated.
          </h4>
        </div>
      </div>
    </div>
  );
};

export default Card;
