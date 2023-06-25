using Lina.DynamicServicesProvider.Attributes;
using Microsoft.EntityFrameworkCore;
using Saku.Database;
using Saku.Models;
using Saku.Repositories.Interfaces;

namespace Saku.Repositories;

[Repository(typeof(IUserRepository))]
public class UserRepository : IUserRepository
{
    private readonly SakuDbContext _dbContext;

    public UserRepository(SakuDbContext dbContext)
    {
        _dbContext = dbContext;
    }

    public async ValueTask Add(UserModel model)
    {
        await _dbContext.Users.AddAsync(model);
    }

    public async Task<UserModel?> GetByDiscordId(ulong discordId)
    {
        return await _dbContext.Users.FirstOrDefaultAsync(x => x.DiscordId == discordId);
    }
}