using Saku.Models;

namespace Saku.Repositories.Interfaces;

public interface IUserRepository
{
    ValueTask Add(UserModel model);
    Task<UserModel?> GetByDiscordId(ulong discordId);
}