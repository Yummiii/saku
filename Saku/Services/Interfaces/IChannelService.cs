using Saku.ViewModels;

namespace Saku.Services.Interfaces;

public interface IChannelService
{
    ValueTask ChangeChannelState(ChannelRegisterViewModel register);
    Task<ChannelViewModel?> GetByDiscordId(ulong discordChannelId);
}