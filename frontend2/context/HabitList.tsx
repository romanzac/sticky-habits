import React, { useState } from "react";
import Web3Modal from "web3modal";
import { ethers } from "ethers";
import { Wallet } from './near-wallet';
import { StickyHabits } from './near-interface';
import { create as ipfsHttpClient } from "ipfs-http-client";

const CONTRACT_ADDRESS = process.env.CONTRACT_NAME;

// Wallet instance
const wallet = new Wallet({ createAccessKeyFor: CONTRACT_ADDRESS })

// Logic for interacting with the contract
const stickyHabits = new StickyHabits({ contractId: CONTRACT_ADDRESS, walletToUse: wallet });

export const HabitListContext = React.createContext("");

export const HabitProvider = ({ children }) => {
  const [currentAccount, setCurrentAccount] = useState("");
  const [error, setError] = useState("");
  const [allHabitList, setAllHabitList] = useState([]);
  const [myList, setmyList] = useState([]);

  const [allAddress, setAllAddress] = useState([]);
  //----CONNECTING METAMASK

  const checkIfWalletIsConnected = async () => {
    if (!window.ethereum) return setError("Please Install MetaMask");

    const account = await window.ethereum.request({ method: "eth_accounts" });

    if (account.length) {
      setCurrentAccount(account[0]);
    } else {
      setError("Please Install MetaMask & Connect, Reload");
    }
  };

  //-----CONNECT WALLET
  const connectWallet = async () => {
    if (!window.ethereum) return setError("Please Install MetaMask");

    const account = await window.ethereum.request({
      method: "eth_requestAccounts",
    });

    setCurrentAccount(account[0]);
  };

  //----UPLOAD TO IPFS VOTER IMAGE
  const uploadToIPFS = async (file) => {
    try {
      const added = await client.add({ content: file });

      const url = `https://ipfs.infura.io/ipfs/${added.path}`;
      return url;
    } catch (error) {
      setError("Error Uploading file to IPFS");
    }
  };

  const HabitList = async (message) => {
    try {
      //CONNECTING SMART CONTRACT
      const web3Modal = new Web3Modal();
      const connection = await web3Modal.connect();
      const provider = new ethers.providers.Web3Provider(connection);
      const signer = provider.getSigner();
      const contract = await fetchContract(signer);

      const data = JSON.stringify({ message });
      const added = await client.add(data);

      const url = `https://ipfs.infura.io/ipfs/${added.path}`;
      console.log(url);

      const createList = await contract.createList(message);
      createList.wait();
      consolelog(createList);
    } catch (error) {
      setError("something wrong creating list");
    }
  };

  const getHabitList = async () => {
    try {
      //CONNECTING SMART CONTRACT
      const web3Modal = new Web3Modal();
      const connection = await web3Modal.connect();
      const provider = new ethers.providers.Web3Provider(connection);
      const signer = provider.getSigner();
      const contract = await fetchContract(signer);

      //GET DATA
      const getAllAddress = await contract.getAddress();
      setAllAddress(getAllAddress);
      console.log(getAllAddress);

      getAllAddress.map(async (el) => {
        const getSingleData = await contract.getCreatorData(el);
        allHabitList.push(getSingleData);
        console.log(getSingleData);
      });

      const allMessage = await contract.getMessage();
      setmyList(allMessage);
    } catch (error) {
      setError("Something wrong while getting the data");
    }
  };

  const change = async (address) => {
    try {
      //CONNECTING SMART CONTRACT
      const web3Modal = new Web3Modal();
      const connection = await web3Modal.connect();
      const provider = new ethers.providers.Web3Provider(connection);
      const signer = provider.getSigner();
      const contract = await fetchContract(signer);

      const state = await contract.toggle(address);
      state.wait();
      console.log(state);
    } catch (error) {
      console.log("Wrong");
    }
  };

  return (
    <HabitListContext.Provider
      value={{
    checkIfWalletIsConnected,
      connectWallet,
      uploadToIPFS,
      HabitList,
      allHabitList,
      currentAccount,
      getHabitList,
      error,
      allAddress,
      myList,
      change,
  }}
>
  {children}
  </HabitListContext.Provider>
);
};
