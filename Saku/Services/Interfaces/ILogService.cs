using Discord;

namespace Saku.Services.Interfaces;

public interface ILogService
{
    void DiscordLogWriter(LogMessage message);
}