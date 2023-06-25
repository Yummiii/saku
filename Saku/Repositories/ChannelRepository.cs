using Lina.DynamicServicesProvider.Attributes;
using Microsoft.EntityFrameworkCore;
using Saku.Database;
using Saku.Models;
using Saku.Repositories.Interfaces;

namespace Saku.Repositories;

[Repository(typeof(IChannelRepository))]
public class ChannelRepository : IChannelRepository
{
    private readonly SakuDbContext _dbContext;

    public ChannelRepository(SakuDbContext dbContext)
    {
        _dbContext = dbContext;
    }

    public async ValueTask Add(ChannelModel model)
    {
        await _dbContext.Channels.AddAsync(model);
    }

    public async Task<ChannelModel?> GetByDiscordId(ulong discordChannelId)
    {
        return await _dbContext.Channels.FirstOrDefaultAsync(x => x.DiscordChannelId == discordChannelId);
    }

    public async ValueTask<bool> HasChannel(ulong discordChannelId)
    {
        return await _dbContext.Channels.AnyAsync(x => x.DiscordChannelId == discordChannelId);
    }

    public void Update(ChannelModel model)
    {
        _dbContext.Channels.Update(model);
    }
}