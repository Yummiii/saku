using Discord;
using Discord.Interactions;
using Saku.Permissions;
using Saku.Services.Interfaces;
using Saku.ViewModels;

namespace Saku.Modules;

[RequireBotOwner]
[Group("cs", "Configuração dos Estados")]
public class ChangeStateModule : InteractionModuleBase
{
    private readonly IChannelService _channelService;

    public ChangeStateModule(IChannelService channelService)
    {
        _channelService = channelService;
    }

    [SlashCommand("channel", "Mudar configuração do estado do canal")]
    public async Task ChangeChannelStateSubCommand([Summary("canal", "Canal que será alterado")] ITextChannel channel,
        [Summary("estado", "Novo estado do canal")]
        ChannelStateInCommand state)
    {
        var channelState = new ChannelRegisterViewModel(Context.Channel.Id,
            state is ChannelStateInCommand.Enabled or ChannelStateInCommand.NoLogs,
            state == ChannelStateInCommand.NoLogs);

        await _channelService.ChangeChannelState(channelState);

        await RespondAsync(":D", ephemeral: true);
    }
}