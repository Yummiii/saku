using Discord.WebSocket;

namespace Saku.Events.Interfaces;

public interface IAutoLoadEvents
{
    void AddEvent(DiscordSocketClient discordClient);
}