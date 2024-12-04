import { WalletAdapterNetwork } from '@solana/wallet-adapter-base';
import { ConnectionProvider, WalletProvider } from '@solana/wallet-adapter-react';
import { WalletModalProvider } from '@solana/wallet-adapter-react-ui';
import {
    PhantomWalletAdapter,
    SolflareWalletAdapter,
    BackpackWalletAdapter,
    GlowWalletAdapter,
    LedgerWalletAdapter,
    SlopeWalletAdapter,
    TorusWalletAdapter,
} from '@solana/wallet-adapter-wallets';
import { clusterApiUrl } from '@solana/web3.js';
import { useMemo } from 'react';
require('@solana/wallet-adapter-react-ui/styles.css');

export const WalletContextProvider = ({ children }) => {
    const network = WalletAdapterNetwork.Devnet;
    const endpoint = useMemo(() => clusterApiUrl(network), [network]);
    
    const wallets = useMemo(
        () => [
            new PhantomWalletAdapter(),
            new SolflareWalletAdapter(),
            new BackpackWalletAdapter(),
            new GlowWalletAdapter(),
            new LedgerWalletAdapter(),
            new SlopeWalletAdapter(),
            new TorusWalletAdapter(),
        ],
        []
    );

    return (
        <ConnectionProvider endpoint={endpoint}>
            <WalletProvider wallets={wallets} autoConnect>
                <WalletModalProvider>{children}</WalletModalProvider>
            </WalletProvider>
        </ConnectionProvider>
    );
};
