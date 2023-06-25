using Saku.Models;

namespace Saku.Repositories.Interfaces;

public interface IChannelRepository
{
    ValueTask Add(ChannelModel model);
    Task<ChannelModel?> GetByDiscordId(ulong discordChannelId);
    ValueTask<bool> HasChannel(ulong discordChannelId);
    void Update(ChannelModel model);
}