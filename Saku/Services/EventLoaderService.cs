using Discord;
using Discord.WebSocket;
using Lina.DynamicServicesProvider.Attributes;
using Microsoft.Extensions.Logging;
using Saku.Events.Interfaces;
using Saku.Services.Interfaces;

namespace Saku.Services;

[Service(typeof(IEventLoaderService))]
public class EventLoaderService : IEventLoaderService
{
    private readonly IEnumerable<IAutoLoadEvents> _loadedEvents;
    private readonly IDiscordClient _discordClient;
    private readonly ILogger<EventLoaderService> _logger;


    public EventLoaderService(
        IEnumerable<IAutoLoadEvents> loadedEvents,
        IDiscordClient discordClient, 
        ILogger<EventLoaderService> logger)
    {
        _loadedEvents = loadedEvents;
        _discordClient = discordClient;
        _logger = logger;
    }

    public void Initialize()
    {
        if(_discordClient is not DiscordSocketClient discordSocketClient) return;
        foreach (var @event in _loadedEvents)
        {
            @event.AddEvent(discordSocketClient);
        }
        
        _logger.LogInformation("Loaded {} Events", _loadedEvents.Count());
    }
}