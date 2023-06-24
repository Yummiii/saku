using Discord.Interactions;
using Discord.WebSocket;
using Lina.DynamicServicesProvider;
using Lina.DynamicServicesProvider.Attributes;
using Microsoft.Extensions.Logging;
using Saku.Events.Interfaces;
using Saku.ViewModels.Interfaces;

namespace Saku.Events;

[Dependency(LifeTime.Transient, typeof(IAutoLoadEvents))]
public class ReadyEvent : IAutoLoadEvents
{
    private readonly ILogger<ReadyEvent> _logger;
    private readonly InteractionService _interactionService;
    private readonly ISakuConfig _sakuConfig;

    public ReadyEvent(ILogger<ReadyEvent> logger, InteractionService interactionService, ISakuConfig sakuConfig)
    {
        _logger = logger;
        _interactionService = interactionService;
        _sakuConfig = sakuConfig;
    }

    public void AddEvent(DiscordSocketClient discordClient)
    {
        discordClient.Ready += DiscordClientOnReady;
    }

    private async Task DiscordClientOnReady()
    {
        if (ulong.TryParse(_sakuConfig.DevelopGuild, out var discordGuildId))
        {
#if DEBUG
            const string Type = "\"Guild\"";
            var commands = await _interactionService.RegisterCommandsToGuildAsync(discordGuildId);
#else
            const string Type = "\"Global\"";
            var commands = await _interactionService.RegisterCommandsGloballyAsync();
#endif
            _logger.LogInformation("Loaded {} {} Commands", Type, commands.Count);
        }
        else
        {
            throw new Exception("The developer guild id isn't a valid ulong");
        }
    }
}