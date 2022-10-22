import { near, json, log, BigInt } from "@graphprotocol/graph-ts"
import { Transaction } from "../generated/schema"

export function handleReceipt(receipt: near.ReceiptWithOutcome): void {
  const actions = receipt.receipt.actions
  for (let i = 0; i < actions.length; i++) {
    handleAction(actions[i], receipt, receipt.block)
  }
}

function handleAction(
  action: near.ActionValue,
  receiptWithOutcome: near.ReceiptWithOutcome,
  block: near.Block
): void {
  if (action.kind != near.ActionKind.FUNCTION_CALL) {
    return
  }
  const outcome = receiptWithOutcome.outcome
  const functionCall = action.toFunctionCall()
  const methodName = functionCall.methodName
  const timestamp = block.header.timestampNanosec.toString()

  if (methodName == "resolve_purchase") {
    for (let logIndex = 0; logIndex < outcome.logs.length; logIndex++) {
      const outcomeLog = outcome.logs[logIndex].toString()

      log.info("outcomeLog {}", [outcomeLog])

      const jsonData = json.try_fromString(outcomeLog)
      const jsonObject = jsonData.value.toObject()
      const purchaseObject = jsonObject.get("params")

      if (purchaseObject) {
        const purchaseLogObject = purchaseObject.toObject()

        const nftContractIdJson = purchaseLogObject.get("nft_contract_id")
        const ownerIdJson = purchaseLogObject.get("owner_id")
        const buyerIdJson = purchaseLogObject.get("buyer_id")
        const tokenIdJson = purchaseLogObject.get("token_id")
        const ftTokenIdJson = purchaseLogObject.get("ft_token_id")
        const priceIdJson = purchaseLogObject.get("price")

        if (
          !nftContractIdJson ||
          !ownerIdJson ||
          !buyerIdJson ||
          !tokenIdJson ||
          !ftTokenIdJson ||
          !priceIdJson
        )
          return
        const nftContractId = nftContractIdJson.toString()
        const ownerId = ownerIdJson.toString()
        const buyerId = buyerIdJson.toString()
        const tokenId = tokenIdJson.toString()
        const ftTokenId = ftTokenIdJson.toString()
        const price = BigInt.fromString(priceIdJson.toString())

        const entityId = timestamp + nftContractId + tokenId
        let transaction = Transaction.load(entityId)

        if (!transaction) {
          transaction = new Transaction(entityId)
          transaction.nftContractId = nftContractId
          transaction.ownerId = ownerId
          transaction.buyerId = buyerId
          transaction.tokenId = tokenId
          transaction.ftTokenId = ftTokenId
          transaction.price = price
          transaction.timestamp = BigInt.fromString(timestamp)

          transaction.save()
        }
      }
    }
  }
}
