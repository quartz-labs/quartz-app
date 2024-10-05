"use client";

import dynamic from 'next/dynamic';
import Image from 'next/image';
import styles from './page.module.css';
import Balance from "@/components/balance/Balance";
import { useAnchorWallet, useConnection, useWallet } from '@solana/wallet-adapter-react';
import { useRouter } from 'next/navigation';
import { useEffect, useState } from 'react';
import { isVaultInitialized } from '@/utils/utils';
import Modal, { ModalProps } from '@/components/modal/Modal';
import { depositLamports, withdrawLamports } from '@/utils/instructions';
import { LAMPORTS_PER_SOL, SystemProgram, Transaction, VersionedTransaction } from '@solana/web3.js';
import { getVault } from '@/utils/getPDAs';
import { web3 } from '@coral-xyz/anchor';

const WalletMultiButtonDynamic = dynamic(
    () => import("@solana/wallet-adapter-react-ui").then((mod) => mod.WalletMultiButton),
    { ssr: false }
);

export default function Dashboard() {
    const { connection } = useConnection();
    const wallet = useAnchorWallet();
    const router = useRouter();
    const { publicKey, sendTransaction } = useWallet();

    const [modalEnabled, setModalEnabled] = useState(false);
    const [modalData, setModalData] = useState<ModalProps>({
        title: "",
        denomination: "",
        buttonText: "",
        onConfirm: (amount: number) => { },
        onCancel: () => { }
    });

    useEffect(() => {
        const isLoggedIn = async () => {
            if (!wallet || !await isVaultInitialized(wallet, connection)) {
                router.push("/");
            }
        }
        isLoggedIn();
    }, [wallet]);

    const handleDeposit = () => {
        setModalEnabled(true);
        setModalData({
            title: "Deposit SOL",
            denomination: "SOL",
            buttonText: "Deposit",
            onConfirm: async (amount: number) => {
                if (!wallet) {
                    console.error("Error: Wallet not connected");
                    return;
                }
                const signature = await depositLamports(wallet, connection, amount * LAMPORTS_PER_SOL);
                console.log(signature);
                if (signature) setModalEnabled(false);
            },
            onCancel: () => { setModalEnabled(false); }
        })
    }

    const handleWithdraw = () => {
        setModalEnabled(true);
        setModalData({
            title: "Withdraw SOL",
            denomination: "SOL",
            buttonText: "Withdraw",
            onConfirm: async (amount: number) => {
                if (!wallet) {
                    console.error("Error: Wallet not connected");
                    return;
                }
                const signature = await withdrawLamports(wallet, connection, amount * LAMPORTS_PER_SOL);
                console.log(signature);
                if (signature) setModalEnabled(false);
            },
            onCancel: () => { setModalEnabled(false); }
        })
    }

    const handleLiquidate = () => {
        setModalEnabled(true);
        setModalData({
            title: "Liquidate SOL",
            denomination: "SOL",
            buttonText: "Liquidate",
            onConfirm: async (amount: number) => {
                console.log("Liquidate " + amount);
                setModalEnabled(false);
            },
            onCancel: () => { setModalEnabled(false); }
        })
    }

    const handleOfframp = () => {
        setModalEnabled(true);
        setModalData({
            title: "Offramp USDC",
            denomination: "USDC",
            buttonText: "Offramp",
            onConfirm: async (amount: number) => {
                console.log("Offramp " + amount);
                //Add code here to call the get loan instruction
                // const signature = "sig";                
                // const checkTransaction = async (signature: string) => {
                //     try {
                //         const connection = new web3.Connection(web3.clusterApiUrl('mainnet-beta'), 'confirmed');
                //         const transaction = await connection.getParsedTransaction(signature);
                //         if (transaction) {
                //             console.log("Transaction is confirmed");
                //             // Open the offramp page
                //             window.open(rampUrl, '_blank');
                //         } else {
                //             console.error("Transaction is not confirmed");
                //             // Show a message to the user
                //             alert("Transaction is not confirmed. Please try again later.");
                //         }
                //     } catch (error) {
                //         console.error("Error checking transaction:", error);
                //         // Show a message to the user
                //         alert("Error checking transaction. Please try again later.");
                //     }
                // };
                // checkTransaction(signature);

                const rampUrl = `https://exchange.mercuryo.io/?fiat_currency=EUR&currency=USDC&network=SOLANA&amount=${amount}&type=sell`
                window.open(rampUrl, '_blank');
                setModalEnabled(false);

            },
            onCancel: () => { setModalEnabled(false); }
        })
    }

    return (
        <main className="container">
            {modalEnabled && (
                <Modal {...modalData} />
            )}

            <div className={styles.navBar}>
                <Image
                    src="/logo.svg"
                    alt="Quartz"
                    width={200}
                    height={69}
                />

                <WalletMultiButtonDynamic />
            </div>

            <Balance />

            <div className={styles.buttons}>
                <button onClick={handleDeposit} className={`${styles.mainButton} glassButton`}>Deposit</button>
                <button onClick={handleWithdraw} className={`${styles.mainButton} glassButton`}>Withdraw</button>
                <button onClick={handleLiquidate} className={`${styles.mainButton} glassButton`}>Liquidate</button>
                <button onClick={handleOfframp} className={`${styles.mainButton} glassButton`}>Off-ramp</button>
            </div>
        </main>
    )
}