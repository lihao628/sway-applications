import { Card } from "@fuel-ui/react";

import { DECIMAL_PLACES } from "../../../config";
import type { EscrowInfoOutput } from "../../../types/contracts/EscrowAbi";
import type { EscrowInfoWorkaround } from "../hooks/useEscrows";
import { formatValue } from "../utils/helpers";

interface EscrowInfoProps {
  escrows: EscrowInfoWorkaround[];
}

export function EscrowInfo({ escrows }: EscrowInfoProps) {
  return (
    <Card.Body>
      <div>{`Arbiter: ${
        escrows[0].arbiter.address.Address
          ? escrows[0].arbiter.address.Address?.value
          : escrows[0].arbiter.address.ContractId!.value
      }`}</div>
      <div>{`Arbiter Asset: ${escrows[0].arbiter.asset.value}`}</div>
      <div>{`Arbiter Fee: ${formatValue(
        escrows[0].arbiter.fee_amount,
        DECIMAL_PLACES
      )}`}</div>
      {escrows[0].assets.map((assetInfo, i) => {
        return (
          <div key={i}>
            <div>{`Asset Id: ${assetInfo.id.value}`}</div>
            <div>{`Asset Amount: ${formatValue(
              assetInfo.amount,
              DECIMAL_PLACES
            )}`}</div>
          </div>
        );
      })}
      <div>{`Buyer: ${
        escrows[0].buyer.address.Address
          ? escrows[0].buyer.address.Address?.value
          : escrows[0].buyer.address.ContractId?.value
      }`}</div>
      <div>{`Buyer Desposit Asset: ${
        !escrows[0].buyer.asset ? "None" : escrows[0].buyer.asset.value
      }`}</div>
      <div>{`Buyer Deposit Amount: ${formatValue(
        escrows[0].buyer.deposited_amount,
        DECIMAL_PLACES
      )}`}</div>
      <div>{`Seller: ${
        escrows[0].seller.address.Address
          ? escrows[0].seller.address.Address?.value
          : escrows[0].seller.address.ContractId?.value
      }`}</div>
      <div>{`Deadline: ${escrows[0].deadline.toString()}`}</div>
      <div>{`Disputed: ${escrows[0].disputed}`}</div>
      <div>{`State: ${
        escrows[0].state.Pending ? "Pending" : "Completed"
      }`}</div>
    </Card.Body>
  );
}
