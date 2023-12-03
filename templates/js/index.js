import { Client, Events, IntentsBitField } from "discord.js";

let client = new Client({ intents: [
    IntentsBitField.Flags.Guilds,
    IntentsBitField.Flags.GuildMembers,
    IntentsBitField.Flags.GuildMessages,
    IntentsBitField.Flags.GuildMessageReactions,
    IntentsBitField.Flags.MessageContent,
]});

client.login("<TOKEN>");

client.on(Events.MessageCreate, async (message) => {
    if (!message.author.bot && message.content == "ping") {
        await message.reply(`Pong!\n\`${message.createdTimestamp - new Date().getTime()}ms\``);
    }
})