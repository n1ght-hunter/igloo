import type { view as ViewType, update as UpdateType, MessageId, Message } from "iced:app/app@0.1.0"
import type { cloneMessage as CloneMessageType } from "iced:app/message@0.1.0"
import { createApp, Text, Column, Button, Row } from "igloo-ts"

interface MessageExport {
    cloneMessage: typeof CloneMessageType;
}

export const message: MessageExport = {
    cloneMessage: (messageId: MessageId) => {
        return messageId
    }
}

// Define app state and messages
type State = { count: number }
type Msg = { type: 'increment' } | { type: 'decrement' }

// Create the app using igloo-ts
const app = createApp<State, Msg>({
    init: () => ({ count: 0 }),

    update: (state, msg) => {
        switch (msg.type) {
            case 'increment': return { count: state.count + 1 }
            case 'decrement': return { count: state.count - 1 }
        }
    },

    view: (state, messages) => {
        return Column.new()
            .spacing(10)
            .push(Text.new(`Count: ${state.count}`).size(24))
            .push(
                Row.new()
                    .spacing(10)
                    .push(
                        Button.new(Text.new('+'))
                            .onPress(messages, () => ({ type: 'increment' }))
                    )
                    .push(
                        Button.new(Text.new('-'))
                            .onPress(messages, () => ({ type: 'decrement' }))
                    )
            )
    }
})

export const update: typeof UpdateType = app.update
export const view: typeof ViewType = app.view
