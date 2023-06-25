using Saku.Models;

namespace Saku.Repositories.Interfaces;

public interface IChatContextRepository
{
    ValueTask Add(ChatContextModel model);

    Task<IEnumerable<ChatContextModel>> GetChannelContext(ulong channelId,
        bool isPresentInCurrentContext = true);
}