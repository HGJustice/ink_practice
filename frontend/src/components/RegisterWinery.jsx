import React, { useState } from "react";
import { useInkathon, useRegisteredContract } from "@scio-labs/use-inkathon";

export default function RegisterWinery() {
  const { api, activeAccount } = useInkathon();
  const { contract, address } = useRegisteredContract("winery_management");

  const [formData, setFormData] = useState({
    wineryName: "",
  });

  const handleInputChange = (e) => {
    setFormData({
      ...formData,
      [e.target.name]: e.target.value,
    });
  };

  const handleSubmit = async (e) => {
    e.preventDefault();

    const hardcodedAltitude = 500;
    const hardcodedLongitude = 455;

    const estimatedGas = await contract.query.createWinery(
      activeAccount.address,
      { value: 0 },
      formData.wineryName,
      hardcodedAltitude,
      hardcodedLongitude
    );

    contract.tx
      .createWinery(
        { estimatedGas, value: 0 },
        formData.wineryName,
        hardcodedAltitude,
        hardcodedLongitude
      )
      .signAndSend(activeAccount.address);
  };

  return (
    <div>
      <h2>Register Winery</h2>
      <form onSubmit={handleSubmit}>
        <label>Winery Name</label>
        <input
          type="text"
          name="wineryName"
          placeholder="Winery Name"
          value={formData.wineryName}
          onChange={handleInputChange}
        />
        <button type="submit">Create Winery</button>
      </form>
    </div>
  );
}
