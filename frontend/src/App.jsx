import React, { useEffect } from "react";
import { UseInkathonProvider, useInkathon } from "@scio-labs/use-inkathon";
import { alephzeroTestnet } from "@scio-labs/use-inkathon";

// Import your other components here
import RegisterWinery from "./components/RegisterWinery";

// Define your deployments
const getDeployments = async () => {
  return [
    {
      contractId: "winery_management",
      networkId: alephzeroTestnet.network,
      abi: await import("./Abi/WineryManagement.json"),
      address: "5ENjGEdFjadS24ZEFJCYuAwGFi3LSG5bHVjozNCg1iwUir7L",
    },
    // Add more deployments for other contracts if needed
  ];
};

function AppContent() {
  const { connect, isConnecting } = useInkathon();

  useEffect(() => {
    connect();
  }, [connect]);

  if (isConnecting) {
    return <div>Connecting to wallet...</div>;
  }

  return <RegisterWinery />;
}

function App() {
  return (
    <UseInkathonProvider
      appName="Wine-To-Chain"
      defaultChain={alephzeroTestnet}
      deployments={getDeployments()}
    >
      <AppContent />
    </UseInkathonProvider>
  );
}

export default App;
