import type { view as ViewType, update as UpdateType, Message as MessageId } from "iced:app/app@0.1.0"
import type { cloneMessage as CloneMessageType } from "iced:app/message@0.1.0"
import { textToElement } from "iced:app/element@0.1.0"

interface MessageType {
    cloneMessage: typeof CloneMessageType;
}


export const message: MessageType = {
    cloneMessage: (
        message_id: MessageId
    ) => {
        return message_id
    }
}


export const update: typeof UpdateType = (message_id: MessageId) => { }

export const view: typeof ViewType = () => {
    return textToElement({
        text: "Hello from JS plugin!"
    });
}