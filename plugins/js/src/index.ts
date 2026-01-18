import type { view as ViewType, update as UpdateType, MessageId, Message } from "iced:app/app@0.1.0"
import type { cloneMessage as CloneMessageType } from "iced:app/message@0.1.0"
import { textToElement } from "iced:app/element@0.1.0"

interface MessageExport {
    cloneMessage: typeof CloneMessageType;
}

export const message: MessageExport = {
    cloneMessage: (messageId: MessageId) => {
        return messageId
    }
}

export const update: typeof UpdateType = (messageId: MessageId, msg: Message) => { }

export const view: typeof ViewType = () => {
    return textToElement({
        text: "Hello from JS plugin!"
    });
}