using Saku.ViewModels;

namespace Saku.Services.Interfaces;

public interface IIaChatService
{
    Task<string> ProcessMessageSend(InputChatMessageViewModel input);
}