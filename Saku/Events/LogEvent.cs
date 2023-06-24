using Discord;
using Discord.WebSocket;
using Lina.DynamicServicesProvider;
using Lina.DynamicServicesProvider.Attributes;
using Saku.Events.Interfaces;
using Saku.Services.Interfaces;

namespace Saku.Events;

[Dependency(LifeTime.Transient, typeof(IAutoLoadEvents))]
public class LogEvent : IAutoLoadEvents
{
    private readonly ILogService _logService;

    public LogEvent(ILogService logService)
    {
        _logService = logService;
    }

    public void AddEvent(DiscordSocketClient discordClient)
    {
        discordClient.Log += DiscordClientOnLog;       
    }

    private async Task DiscordClientOnLog(LogMessage arg)
    {
        _logService.DiscordLogWriter(arg);
        await Task.CompletedTask;
    }
}