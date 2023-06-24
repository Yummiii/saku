using Discord;
using Discord.Interactions;
using Discord.WebSocket;
using Lina.DynamicServicesProvider;
using Lina.DynamicServicesProvider.Attributes;
using Saku.Events.Interfaces;

namespace Saku.Events;

[Dependency(LifeTime.Transient, typeof(IAutoLoadEvents))]
public class InteractionCreatedEvent : IAutoLoadEvents
{
    private readonly IDiscordClient _discordClient;
    private readonly InteractionService _interactionService;
    private readonly IServiceProvider _serviceProvider;

    public InteractionCreatedEvent(
        IDiscordClient discordClient,
        InteractionService interactionService,
        IServiceProvider serviceProvider)
    {
        _discordClient = discordClient;
        _interactionService = interactionService;
        _serviceProvider = serviceProvider;
    }

    public void AddEvent(DiscordSocketClient discordClient)
    {
        discordClient.InteractionCreated += DiscordClientOnInteractionCreated;
    }

    private async Task DiscordClientOnInteractionCreated(SocketInteraction arg)
    {
        if(_discordClient is not DiscordSocketClient discordSocketClient) return;
        var ctx = new SocketInteractionContext(discordSocketClient, arg);
        await _interactionService.ExecuteCommandAsync(ctx, _serviceProvider);
    }
}