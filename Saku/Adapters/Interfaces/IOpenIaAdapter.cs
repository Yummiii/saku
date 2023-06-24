using Saku.ViewModels;

namespace Saku.Adapters.Interfaces;

public interface IOpenIaAdapter
{
    Task<IEnumerable<ChatMessageViewModel>> SendChat(IEnumerable<ChatMessageViewModel> chats,
        GptModelType model = GptModelType.Gpt35Turbo);
}