import { Client, Events, IntentsBitField, Message } from "discord.js";

let client: Client = new Client({ intents: [
    IntentsBitField.Flags.Guilds,
    IntentsBitField.Flags.GuildMembers,
    IntentsBitField.Flags.GuildMessages,
    IntentsBitField.Flags.GuildMessageReactions,
    IntentsBitField.Flags.MessageContent,
]});

client.login("<TOKEN>");

client.on(Events.MessageCreate, async (message: Message) => {
    if (!message.author.bot && message.content == "ping") {
        await message.reply(`Pong!\n\`${message.createdTimestamp - new Date().getTime()}ms\``);
    }
})