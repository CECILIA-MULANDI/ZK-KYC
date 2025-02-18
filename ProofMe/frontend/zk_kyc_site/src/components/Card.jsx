/* eslint-disable react/no-unescaped-entities */
import { CloudArrowUpIcon } from "@heroicons/react/24/solid";
const Card = () => {
  return (
    <div className="flex flex-col bg-white w-1/2 mx-auto h-[350px] mt-52 shadow-lg px-3 py-6 gap-4">
      <h1 className="font-bold text-left text-black text-xl">
        Zero-Knowledge KYC Verification
      </h1>
      <h3 className="text-gray-400 text-left">
        Privately verify your identity using zero-knowledge proofs
      </h3>
      <button></button>
      <h2 className="mx-auto text-black text-xl font-bold">
        Upload Identity Documents
      </h2>
      <h4 className="text-gray-400 text-left mx-auto">
        Supported formats: Passport, Driver's License, or National ID
      </h4>
      <div className="flex flex-col items-center gap-2">
        <CloudArrowUpIcon className="h-12 w-12 text-gray-500" />
        <button className="bg-black flex items-center justify-center gap-2 p-2 rounded text-white w-3/4">
          Upload Documents
        </button>
      </div>

      <h4 className="text-gray-400 text-left  ">
        Your personal information is never stored or shared. Only zero-knowledge
        proofs are generated.
      </h4>
    </div>
  );
};

export default Card;
