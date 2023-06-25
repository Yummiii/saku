using Discord.WebSocket;
using Lina.DynamicServicesProvider;
using Lina.DynamicServicesProvider.Attributes;
using Saku.Events.Interfaces;
using Saku.Services.Interfaces;
using Saku.ViewModels;

namespace Saku.Events;

[Dependency(LifeTime.Transient, typeof(IAutoLoadEvents))]
public class MessageCreatedEvent : IAutoLoadEvents
{
    private readonly IIaChatService _iaChatService;

    public MessageCreatedEvent(IIaChatService iaChatService)
    {
        _iaChatService = iaChatService;
    }

    public void AddEvent(DiscordSocketClient discordClient)
    {
        discordClient.MessageReceived += DiscordClientOnMessageReceived;
    }

    private async Task DiscordClientOnMessageReceived(SocketMessage arg)
    {
        if(arg.Author.IsBot) return;
        
        if(!await _iaChatService.CheckPermissions(arg.Author.Id, arg.Channel.Id)) return;

        await arg.Channel.TriggerTypingAsync();
        var input = new InputChatMessageViewModel(arg.Author.Id, arg.Channel.Id, arg.Content, arg.Author.Username,
            arg.CreatedAt.DateTime);
        var response = await _iaChatService.ProcessMessageSend(input);
        
        if(response is null) return;
        
        await arg.Channel.SendMessageAsync(response);
    }
}