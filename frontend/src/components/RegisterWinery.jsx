import React, { useState } from "react";
import { useInkathon, useRegisteredContract } from "@scio-labs/use-inkathon";

export default function RegisterWinery() {
  const { activeAccount } = useInkathon();
  const { contract } = useRegisteredContract("winery_management");
  const [wineryName, setWineryName] = useState("");

  const handleSubmit = async (e) => {
    e.preventDefault();

    const hardcodedAltitude = 500;
    const hardcodedLongitude = 455;

    await contract.tx
      .createWinery(
        { gasLimit: -1 },
        wineryName,
        hardcodedAltitude,
        hardcodedLongitude
      )
      .signAndSend(activeAccount.address);
  };

  return (
    <div>
      <h2>Register Winery</h2>
      <form onSubmit={handleSubmit}>
        <input
          type="text"
          value={wineryName}
          onChange={(e) => setWineryName(e.target.value)}
          placeholder="Winery Name"
        />
        <button type="submit">Create Winery</button>
      </form>
    </div>
  );
}
