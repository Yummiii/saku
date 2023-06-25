using Lina.DynamicServicesProvider.Attributes;
using Microsoft.EntityFrameworkCore;
using Saku.Database;
using Saku.Models;
using Saku.Repositories.Interfaces;

namespace Saku.Repositories;

[Repository(typeof(IChatContextRepository))]
public class ChatContextRepository : IChatContextRepository
{
    private readonly SakuDbContext _dbContext;

    public ChatContextRepository(SakuDbContext dbContext)
    {
        _dbContext = dbContext;
    }

    public async ValueTask Add(ChatContextModel model)
    {
        await _dbContext.ChatContexts.AddAsync(model);
    }

    public async Task<IEnumerable<ChatContextModel>> GetChannelContext(ulong channelId,
        bool isPresentInCurrentContext = true)
    {
        return await _dbContext.ChatContexts
            .Where(x => x.ChannelId == channelId)
            .Where(x => x.IsPresentInCurrentContext == isPresentInCurrentContext)
            .ToListAsync();
    }
}